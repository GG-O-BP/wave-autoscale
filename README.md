<img src="https://wave-autoscale-marketplace-resources.s3.amazonaws.com/wa-marketplace-logo.png" alt="drawing" style="width:400px;"/>

Wave Autoscale은 클라우드와 쿠버네티스를 위한 통합된 자동 스케일링 및 트래픽 관리 도구입니다. 클라우드의 자동 스케일링과 트래픽 제어를 사용자 정의하여 가용성과 비용 효율성 요구 사항을 충족시킵니다.

![GitHub release (latest by date)](https://img.shields.io/github/v/release/STCLab-Inc/wave-autoscale)
![GitHub](https://img.shields.io/github/license/STCLab-Inc/wave-autoscale)
![GitHub contributors](https://img.shields.io/github/contributors/STCLab-Inc/wave-autoscale)

- 웹사이트: https://waveautoscale.com
- 문서: [https://waveautoscale.com/docs](https://www.waveautoscale.com/docs)

## 시작하기 ##
Wave Autoscale을 설치하는 여러 가지 방법이 있습니다.

- [헬름을 사용하여 쿠버네티스에 설치](https://www.waveautoscale.com/docs/guide/getting-started/k8s-helm/)
- [매니페스트를 사용하여 쿠버네티스에 설치](https://www.waveautoscale.com/docs/guide/getting-started/k8s-manifest/)
- [도커 컴포즈를 사용하여 설치](https://www.waveautoscale.com/docs/guide/getting-started/docker-compose/)
- [도커를 사용하여 설치](https://www.waveautoscale.com/docs/guide/getting-started/docker/)


## 왜 자동 스케일링 관리가 필요한가요?

자동 스케일링의 일반적인 사용 패턴은 K8s 매니페스트 또는 AWS 콘솔에서 설정된 CPU 사용률에 따라 스케일 인 및 스케일 아웃하는 것입니다. 그러나 엔지니어들은 CPU 사용률에 따라 얼마나 많은 파드 또는 VM이 스케일 인 및 스케일 아웃되는지 상상할 수 없습니다. 또한 자동 스케일링의 비용을 예측하는 것도 어렵습니다. IaC (Infrastructure as Code)와 같이, 엔지니어들은 선언적인 방식으로 자동 스케일링을 코드로 관리해야 합니다. Wave Autoscale은 "Scaling Plan as Code"라는 선언적인 방식을 제안하여 자동 스케일링을 관리합니다. 엔지니어들은 코드로 자동 스케일링 구성을 관리하여 자동 스케일링 동작을 이해하고 자동 스케일링의 비용을 예측할 수 있습니다.

- **선언적 자동 스케일링**: Wave Autoscale은 YAML을 사용하여 자동 스케일링 규칙과 리소스 스케일을 선언적으로 정의할 수 있게 해줍니다. 이를 통해 예산과 리소스 할당에 대한 더 예측 가능한 접근 방식을 제공합니다.
- **CPU 사용률 스케일링을 넘어서**: 전통적인 자동 스케일링은 종종 CPU 사용률과 같은 단순한 메트릭에 의존합니다. 이는 필수적인 요소이지만 항상 완전한 그림을 제공하지는 않습니다. Wave Autoscale은 메시지 큐의 크기, 데이터베이스의 상태, 사용자 정의 비즈니스 KPI 등과 같은 더 넓은 범위의 메트릭을 고려하여 더 미세하고 반응성 있는 스케일링 전략을 가능하게 합니다.
- **다양한 컴포넌트의 동시 스케일링**: 현대 아키텍처에서는 쿠버네티스 파드와 같은 단일 컴포넌트를 스케일링하는 것만으로는 충분하지 않을 수 있습니다. 서버리스 함수, 데이터베이스, 캐싱 레이어 등과 같은 관련 컴포넌트도 동시에 스케일링해야 합니다. Wave Autoscale은 모든 애플리케이션의 상호 연결된 부분이 조화롭게 스케일링되도록 다양한 컴포넌트를 커버하는 동시 다발적인 스케일링 기능을 제공합니다.
- **비용 효율적인 스케일링**: 불필요한 과도한 프로비저닝은 비용을 증가시키며, 프로비저닝 부족은 성능에 영향을 줄 수 있습니다. Wave Autoscale은 실제 사용량에 따라 리소스를 지속적으로 모니터링하고 조정하여 성능을 저하시키지 않고 비용 효율적인 스케일링을 보장합니다.
- **비즈니스 이벤트 기반 스케일링**: 전통적인 자동 스케일링은 종종 CPU 사용률과 같은 기술 메트릭에 기반하며, 이는 항상 비즈니스 요구 사항과 일치하지 않을 수 있습니다. Wave Autoscale은 마케팅 캠페인, 제품 출시 등의 비즈니스 이벤트에 기반한 사용자 정의 스케일링 계획을 정의할 수 있게 해줍니다. 이를 통해 리소스가 비즈니스 우선 순위에 따라 할당되도록 보장합니다.

Wave Autoscale은 이러한 도전 과제를 극복하도록 설계되었으며, 기술 복잡성과 비즈니스 요구 사항 모두를 충족하는 더 맞춤화되고 반응성 있는 자동 스케일링 방식을 제공합니다.

## 사용 사례 ##
<img src="https://wave-autoscale-marketplace-resources.s3.amazonaws.com/app-icons/kubernetes.svg" alt="drawing" style="width:50px;"/>

**쿠버네티스**
- **레플리카 스케일링**: 메트릭을 사용하여 배포의 레플리카 수를 스케일링합니다 (HPA가 아님)
- **CoreDNS 자동 스케일링**: 메트릭을 기반으로 CoreDNS 파드를 스케일링합니다
- **파드 Right Sizing**: 메트릭을 기반으로 파드를 동적으로 Right Sizing합니다
- **Istio 트래픽 지연**: 서비스 또는 데이터베이스가 과부하 상태일 때, 지연을 사용하여 과부하로부터 보호할 수 있습니다
- **Istio Rate Limits**: 서비스 또는 데이터베이스가 과부하 상태일 때, 속도 제한을 사용하여 과부하로부터 보호할 수 있습니다

<img src="https://wave-autoscale-marketplace-resources.s3.amazonaws.com/app-icons/aws.svg" alt="drawing" style="width:50px;"/>

**AWS**
- **EC2 자동 스케일링**: 메트릭을 기반으로 EC2 인스턴스를 스케일링합니다
- **ECS 자동 스케일링**: 메트릭을 기반으로 ECS 작업을 스케일링합니다
- **Lambda 동시성**: 최적의 성능과 효율성을 달성하기 위해 예약된 동시성과 프로비전된 동시성을 동적으로 조정합니다
- **EMR on EC2**: 온디맨드 또는 스팟 인스턴스, 단계 동시성 등을 설정합니다
- **AWS WAF v2**: 메트릭을 기반으로 WAF 규칙의 속도 제한을 동적으로 조정합니다

**Google Cloud**, **Azure**, **Cloudflare** 등에 대한 더 많은 사용 사례가 있습니다.

## 유사한 프로젝트 ###
다음은 몇 가지 독특한 기능을 제공하는 주목할 만한 프로젝트입니다:

- KEDA (Kubernetes Event-Driven Autoscaling): KEDA는 쿠버네티스 워크로드에 대한 이벤트 기반 자동 스케일링에 중점을 둔 주요 오픈 소스 프로젝트입니다. 다양한 이벤트 소스를 지원하며 쿠버네티스 배포와 원활하게 작동하도록 설계되었습니다. [KEDA에 대해 더 알아보세요.](https://keda.sh/)

## 설립자 ##
[<img src="https://github.com/STCLab-Inc/wave-autoscale/assets/114452/2dd6b2b4-1a96-4684-aa74-a2dd8bf70bad" width=100 />](https://cloud.stclab.com)

Wave Autoscale은 [STCLab Inc](https://cloud.stclab.com)에 의해 설립되었습니다. 이 회사는 고객이 인프라/서버 비용을 제어하고, 다운타임을 줄이고, 그들의 경험을 향상시키는 가치를 더 추가함으로써 신뢰할 수 있는 웹 트래픽 및 리소스 관리를 제공하는 데 목표를 두고 있는 스타트업입니다.

“STC”는 “Surfing The Wave of Change”의 약자로, 변화의 파도에 대비한 사람만이 파도를 타고 즐길 수 있다는 의미입니다.

## 라이센스
Wave Autoscale은 [Apache License 2.0](https://github.com/STCLab-Inc/wave-autoscale/blob/main/LICENSE)에 따라 라이센스가 부여됩니다.
