/**
 * 이 파일은 web_app_runner 모듈을 정의하고 있습니다.
    이 모듈은 웹 애플리케이션을 실행하는 데 필요한 기능을 제공합니다.
 * 1. App 구조체: 이 구조체는 애플리케이션을 실행하는 데 필요한 정보를 포함합니다.
    이에는 실행할 명령어, 인자, 환경 변수 등이 포함됩니다.
 * 2. run_app 함수: 이 함수는 App 구조체의 정보를 사용하여 애플리케이션을 실행합니다.
 * 3. is_node_installed 함수: 이 함수는 Node.js가 설치되어 있는지 확인합니다.
    Node.js의 버전을 확인하고, 이 버전이 최소 요구 버전보다 낮으면 false를 반환합니다.
 * 4. run_web_app 함수: 이 함수는 웹 애플리케이션을 실행합니다.
    웹 애플리케이션의 경로를 확인하고, Node.js가 설치되어 있는지 확인한 후, 애플리케이션을 실행합니다.
 * 5. 테스트 모듈: 이 모듈은 is_node_installed 함수의 단위 테스트를 포함하고 있습니다.
    이 테스트는 Node.js가 설치되어 있는지 확인합니다.
 */
use regex::Regex;
use std::{
    collections::HashMap,
    process::{Child, Command},
};
use tracing::{debug, error};

const WAVE_WEB_APP: &str = "wave-autoscale-ui";
const MINIMUM_NODE_VERSION: u32 = 14;

struct App {
    command: String,
    args: Vec<String>,
    envs: Option<HashMap<String, String>>,
}

fn run_app(app: &App) -> std::io::Result<Child> {
    let mut command = Command::new(&app.command);
    let command = if !app.args.is_empty() {
        command.args(&app.args)
    } else {
        &mut command
    };
    let command = if let Some(envs) = &app.envs {
        command.envs(envs)
    } else {
        command
    };
    command.spawn()
}

fn is_node_installed() -> bool {
    match Command::new("node").arg("--version").output() {
        Ok(output) => {
            let Ok(output) = String::from_utf8(output.stdout) else {
                return false;
            };
            debug!("[web-app-runner] Node version: {}", output);
            let Ok(regex) = Regex::new(r"v(\d+)\.\d+\.\d+") else {
                return false;
            };
            let Some(captured) = regex.captures(output.as_str()) else {
                return false;
            };
            let Some(major_version) = captured.get(1) else {
                return false;
            };
            debug!(
                "[web-app-runner] Node major version: {}",
                major_version.as_str()
            );
            let Ok(major_version) = major_version.as_str().parse::<u32>() else {
                return false;
            };
            if major_version < MINIMUM_NODE_VERSION {
                return false;
            }
            true
        }
        Err(_) => {
            // Failed to execute the command (e.g., "node" not found)
            false
        }
    }
}

pub fn run_web_app(host: &str, port: u16) -> anyhow::Result<()> {
    let web_app_path = format!("./{}", WAVE_WEB_APP);
    let web_app_file = std::path::Path::new(web_app_path.as_str());
    if !web_app_file.exists() {
        error!("[web-app-runner] {} does not exist", WAVE_WEB_APP);
        return Err(anyhow::anyhow!("{} does not exist", WAVE_WEB_APP));
    }
    if !is_node_installed() {
        error!(
            "{} needs Node.js to run. Minimum version is {}.",
            WAVE_WEB_APP, MINIMUM_NODE_VERSION
        );
        std::process::exit(1);
    }

    let mut envs: HashMap<String, String> = HashMap::new();
    envs.insert("HOSTNAME".to_string(), host.to_string());
    envs.insert("PORT".to_string(), port.to_string());

    let args = vec![format!("./{}/server.js", WAVE_WEB_APP)];

    let result = run_app(&App {
        command: "node".to_string(),
        args,
        envs: Some(envs),
    });
    if result.is_err() {
        error!("Failed to run {}", WAVE_WEB_APP);
        return Err(anyhow::anyhow!("Failed to run {}", WAVE_WEB_APP));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_is_node_installed() {
        assert!(is_node_installed());
    }
}
