## Gazebo

가상 환경에서 로봇을 테스트하는 기술

1. 물리 엔진 (ODE, Bullet, DART)
2. 센서 시뮬레이션 (LiDAR, Camera 등)
3. 3D 렌더링
4. 로봇 모델 시뮬레이션

### Gazebo 구성요소

1. World : 환경 정의 파일
2. Robot Model : 로봇 정의 파일(URDF, SDF)
3. Plugin : Gazebo 기능확장.
4. Launch File : Gazebo와 로봇을 한 번에 실행하기 위한 파일

### URDF vs SDF

URDF : 로봇 자체 정의. Gazebo에서 사용시 변환 필요

SDF : World 전체 정의, 센서, 플러그인, 조명, 물리엔진 등 까지 포함 가능.

Gazebo는 SDF를 직접 사용 가능.

Gazebo 명령어
``` bash
sudo apt install ros-humble-gazebo-ros-pkgs     # 설치
gazebo                                          # 실행
gazebo my_world.sdf                             # my_world 실행
gazebo --verbose                                # 디버깅모드
```

### SLAM(Simultaneous Localization And Mapping)

위치 추정과 지도를 동시에 생성

`Chicken and Egg Problem` : 

Localization(위치추정)을 하려면 지도가 필요하고,

Mapping(지도제작)을 하려면 위치가 필요하다.

둘 다 없으므로 서로 추정하면서 해결해간다.

사용기술

Localization : 현재 위치 추정

Mapping : 주변 환경에 대한 지도 새엉

Sensor Fusion : 여러 센서 데이터를 통합하여 정확도 향상.

동작순서

1. Initialization : 초기화. 센서 데이터 수집.
2. Simultaneous Estimation : 위치 추정 + 지도 생성
3. Loop Closure : 이전 방문한 장소 재인식
4. Graph Optimization : 전체 맵 최적화

### Loop Closure

odometry는 주로 바퀴의 회전 정도(Wheel Encoder)를 기준으로 측정하기 때문에,

바퀴의 미끄러짐이나 로봇의 회전 오차 등으로 인해 누적 오차(Drift)가 생긴다.

로봇이 코스를 한 바퀴 돈다고 가정해보면

SLAM이 추정한 경로는 조금씩 틀어질 수밖에 없다.

그래서 이전의 이미지와 특징점을 매칭하여 유사도가 높으면 같은 지점으로 판단하여

현재 위치 = 예전 위치로 강제로 연결해주는 작업이다.

### Kidnapped Robot Problem

로봇이 갑작스럽게 다른 위치로 이동(납치)되었을 때 

자신의 위치를 파악하지 못하는 상황

1. 사람이 로봇을 들어서 옮김
2. 로봇이 부딪혀서 실제 위치가 밀림
3. 바퀴 미끄러짐 때문에 오도메트리 오차가 커짐
4. 센서 오류 발생
5. 로봇을 껐다 켰는데 초기 위치 추정이 틀림
6. 비슷하게 생긴 공간이 많아서 잘못된 위치로 착각

등의 이유로 Kidnapped Robot Problem이 발생한다.

주요 해결책

1. 좋은 초기 위치 추정 제공
2. LiDAR + IMU + Odometry 같은 Sensor Fusion 사용
3. LiDAR scan matching 품질 향상
4. Loop Closure 및 Submap 파라미터 조정
5. Global Localization Trigger 사용

### GMapping vs Cartographer

GMapping : Ros에서 오래전부터 사용된 2D LiDAR SLAM 알고리즘.

주로 ROS1에서 많이 사용하며, 저사양 하드웨어에서도 비교적 가볍게 동작하지만,

Odometry 품질에 민감하다.

Cartographer : Google이 개발한 SLAM 알고리즘

ROS1, ROS2 모두 사용가능하며, 2D, 3D SLAM을 모두 지원한다.

오차 보정 능력이 비교적 좋고, 넓은 공간, 복잡한 환경에서 더 유리하다.

### NAV

Navigation. 로봇이 현재 위치에서 목표까지 스스로 이동하도록 하는 기술

SLAM이 지도 만들기 + 내 위치 추정 이라면,

NAV는 만들어진 지도 위에서 목표 지점까지 경로를 계획 및 이동하는 기술.

사용 기술

1. Path Planning : 목표 지점까지 최적의 경로를 찾는 과정
2. Behavior Tree : 로봇의 동작을 트리 형태로 구성하여 유동적으로 관리하고 제어하는 알고리즘
3. Trajectory Tracking : 계획된 경로를 따라 로봇을 제어하는 기술

### Nav2

ROS2에서 사용하는 자율주행 네비게이션 프레임워크

1. Planner Server : Dijkstra나 A*(경로 탐색 알고리즘)등을 사용하여 로봇 위치에서 목표 지점까지의 최적 경로 계산
2. Controller Server : DWA 알고리즘을 사용하여 경로를 따라가기 위해 실제로 어떻게 움직일지를 계산(cmd_vel 생성)
3. Smoother Server : Planner가 생성한 경로를 입력으로 받아, 로봇 주변 환경 정보를 나타내는 costmap을 기반으로 경로를 더 부드럽게 변경
4. Recovery Server : 네비게이션 실패 시 clear costmap이나 회전, 후진 등의 복구 동작을 실행

DWA(Dynamic Window Approach) : 

로봇의 동역학 제약(속도, 가속도)을 고려하여 최적의 속도 명령을 선택하는 알고리즘.

Costmap : Map에 비용을 표시한 것. 장애물과 가까울수록 고비용.


