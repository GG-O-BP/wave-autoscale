/**
 * 이 파일은 App 구조체와 그 구현을 정의하고 있습니다.
    App은 Wave Autoscale 애플리케이션의 주요 구성 요소를 관리합니다.
 * 1. 구조체 정의: App 구조체는 애플리케이션의 주요 구성 요소를 멤버로 가지고 있습니다.
    이에는 WaveConfig, DataLayer, MetricUpdater, ScalingComponentManager, ScalingPlannerManager 등이 포함됩니다.
 * 2. 생성자 함수: new 함수는 App 인스턴스를 생성합니다.
    이 함수는 WaveConfig와 DataLayer를 인자로 받아, 이를 기반으로 MetricUpdater, ScalingComponentManager, ScalingPlannerManager를 생성하고 초기화합니다.
 * 3. 애플리케이션 실행 함수: run 함수는 애플리케이션을 실행합니다.
    이 함수는 MetricUpdater를 중지하고, DataLayer에서 스케일링 컴포넌트와 스케일링 계획을 로드하여 ScalingComponentManager와 ScalingPlannerManager에 설정합니다.
 * 4. 자동 스케일링 이력 관리 함수: run_autoscaling_history_cron_job 함수와 stop_autoscaling_history_cron_job 함수는 자동 스케일링 이력을 관리하는 작업을 시작하고 중지합니다.
 * 5. 테스트용 함수: get_data_layer, get_scaling_component_manager, get_scaling_planner_manager 함수는 단위 테스트를 위해 제공되며, 각각 DataLayer, ScalingComponentManager, ScalingPlannerManager의 참조를 반환합니다.
 */
use crate::{
    metric_updater::{MetricUpdater, SharedMetricUpdater},
    scaling_component::{ScalingComponentManager, SharedScalingComponentManager},
    scaling_planner::scaling_planner_manager::{
        ScalingPlannerManager, SharedScalingPlannerManager,
    },
};
use data_layer::data_layer::DataLayer;
use std::sync::Arc;
use tokio::time::sleep;
use tracing::{debug, error, info};
use utils::wave_config::WaveConfig;

pub struct App {
    _wave_config: WaveConfig,
    shared_data_layer: Arc<DataLayer>,
    shared_metric_updater: SharedMetricUpdater,
    shared_scaling_component_manager: SharedScalingComponentManager,
    shared_scaling_planner_manager: SharedScalingPlannerManager,
    autoscaling_history_remover_handle: Option<tokio::task::JoinHandle<()>>,
}

impl App {
    pub async fn new(wave_config: WaveConfig, shared_data_layer: Arc<DataLayer>) -> Self {
        // Create MetricUpdater
        let shared_metric_updater = MetricUpdater::new_shared(shared_data_layer.clone(), 1000);

        // Create ScalingComponentManager
        let shared_scaling_component_manager = ScalingComponentManager::new_shared();

        // Create ScalingPlanManager
        let shared_scaling_planner_manager = ScalingPlannerManager::new_shared(
            shared_data_layer.clone(),
            shared_metric_updater.clone(),
            shared_scaling_component_manager.clone(),
        );

        // Create App
        App {
            _wave_config: wave_config,
            shared_data_layer,
            shared_metric_updater,
            shared_scaling_component_manager,
            shared_scaling_planner_manager,
            autoscaling_history_remover_handle: None,
        }
    }

    // Run the application with ScalingComponentManager, MetricUpdater, and ScalingPlannerManager
    pub async fn run(&mut self) {
        // Stop Metric Updater
        {
            // Scope for shared_metric_updater_writer(RwLock)
            let mut updater_writer = self.shared_metric_updater.write().await;
            updater_writer.stop();
        }

        // Scaling Component Manager
        {
            // Scope for shared_scaling_component_manager_writer(RwLock)

            // Reload scaling component definitions from DataLayer
            let scaling_component_definitions = self
                .shared_data_layer
                .get_enabled_scaling_components()
                .await;

            // If there is an error, log it and return
            if scaling_component_definitions.is_err() {
                let error = scaling_component_definitions.err().unwrap();
                error!("Error getting scaling component definitions: {}", error);
                return;
            }

            // Remove all existing scaling component definitions
            let mut manager_writer = self.shared_scaling_component_manager.write().await;
            manager_writer.remove_all();

            // Add new scaling component definitions
            let scaling_component_definitions = scaling_component_definitions.unwrap();
            let number_of_component_definitions = scaling_component_definitions.len();
            info!(
                "[app] {} scaling component definitions",
                number_of_component_definitions
            );
            if number_of_component_definitions != 0 {
                let scaling_component_result =
                    manager_writer.add_definitions(scaling_component_definitions);

                if scaling_component_result.is_err() {
                    let error = scaling_component_result.err().unwrap();
                    error!("Error adding scaling component definitions: {}", error);
                    return;
                }
            }
        }

        // Scaling Planner Manager
        {
            // Scope for shared_scaling_plan_manager_writer(RwLock)

            // Remove all existing scaling plan definitions
            let mut manager_writer = self.shared_scaling_planner_manager.write().await;
            manager_writer.stop();
            manager_writer.remove_all();

            let plan_definitions = self.shared_data_layer.get_enabled_plans().await;
            if plan_definitions.is_err() {
                let error = plan_definitions.err().unwrap();
                error!("Error getting scaling plan definitions: {}", error);
                return;
            }
            let plan_definitions = plan_definitions.unwrap();
            let number_of_plans = plan_definitions.len();
            info!("[app] {} plan definitions", number_of_plans);

            if number_of_plans != 0 {
                // Run Metric Updater
                {
                    // Scope for shared_metric_updater_writer(RwLock)
                    let mut updater_writer = self.shared_metric_updater.write().await;
                    updater_writer.run().await;
                }

                let scaling_plan_result = manager_writer.add_definitions(plan_definitions);
                if scaling_plan_result.is_err() {
                    let error = scaling_plan_result.err().unwrap();
                    error!("Error adding scaling plan definitions: {}", error);
                    return;
                }
                manager_writer.run();
                info!("[app] ScalingPlans started: {} plans", number_of_plans);
            }
        }
    }

    // Run the cron job to remove the old Autoscaling History
    pub fn run_autoscaling_history_cron_job(&mut self, duration_string: String) {
        self.stop_autoscaling_history_cron_job();
        debug!(
            "[run_autoscaling_history_cron_job] duration_string: {}",
            duration_string
        );
        let duration = duration_str::parse(&duration_string);
        if duration.is_err() {
            error!("Error parsing duration string: {}", duration_string);
            return;
        }
        let duration = duration.unwrap();
        let duration = chrono::Duration::from_std(duration);
        if duration.is_err() {
            error!("Error converting duration: {}", duration_string);
            return;
        }
        let duration = duration.unwrap();
        let to_date = chrono::Utc::now() - duration;
        let data_layer = self.shared_data_layer.clone();
        let handle = tokio::spawn(async move {
            let result = data_layer.remove_old_autoscaling_history(to_date).await;
            if result.is_err() {
                let error = result.err().unwrap();
                debug!("Error removing old autoscaling history: {}", error);
            }

            sleep(std::time::Duration::from_secs(60)).await;
        });
        self.autoscaling_history_remover_handle = Some(handle);
    }

    // Stop the cron job to remove the old Autoscaling History
    pub fn stop_autoscaling_history_cron_job(&mut self) {
        if self.autoscaling_history_remover_handle.is_some() {
            if let Some(handle) = self.autoscaling_history_remover_handle.take() {
                handle.abort();
                self.autoscaling_history_remover_handle = None;
            }
        }
    }

    // For unit testing
    #[allow(dead_code)]
    pub fn get_data_layer(&self) -> Arc<DataLayer> {
        self.shared_data_layer.clone()
    }

    // For unit testing
    #[allow(dead_code)]
    pub fn get_scaling_component_manager(&self) -> SharedScalingComponentManager {
        self.shared_scaling_component_manager.clone()
    }

    // For unit testing
    #[allow(dead_code)]
    pub fn get_scaling_planner_manager(&self) -> SharedScalingPlannerManager {
        self.shared_scaling_planner_manager.clone()
    }
}
