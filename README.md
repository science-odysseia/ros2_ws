### ROS2 설치법(첨부파일 활용)
[ROS2_installation](ROS_installation.md)

### 본 레포지토리 다운로드

```bash
git clone https://github.com/science-odysseia/ROS2_WS.git
```

# ROS2 Tutorial

## 개념정리

### 1) 노드(Node) : 프로그램 1개.

    카메라 처리 프로그램
    라이다 처리 프로그램
    모터 제어 프로그램

전부 각각 하나의 노드

### 2) 패키지(Package) : 노드들과 관련 파일들을 묶어놓은 **프로젝트 폴더**.

```bash
package/
├── node.py
├── subscriber.py
├── package.xml
├── setup.py
```

위 구조처럼 보통 패키지 안에는

    노드 코드
    launch 파일
    설정파일
    package.xml
    setup.py

와 같은 것들이 들어간다.

노드 하나를 실행하기 위해 필요한 파일들 모두가 들어간다고 생각하면 편하다.

### 3) ros2 run    VS    ros2 launch

#### 1. ros2 run

```bash
ros2 run my_pkg my_node
```

특정 노드 1개만 실행시킬 때 사용.

위 코드의 경우 my_pkg 패키지 안에 있는 my_node 프로그램을 실행시키겠다는 의미.

ros2는 파일의 형식에 구애받지 않으므로 my_node는 py일수도, c일수도, cpp일수도, 그 외 다른 것일 수도 있다.

#### 2. ros2 launch
```bash
ros2 launch my_pkg test_launch.py
```

(`my_pkg` : 패키지 이름, `test_launch.py` : 실행파일)

여러 노드 + 설정을 한 번에 실행.

launch 파일 안에 적힌:

    여러 노드
    파라미터
    namespace
    remap

등을 한꺼번에 실행한다.

### 4) 패키지 생성(pkg create)

일반적으로 ROS2에서는 패키지 생성을 워크스페이스/src에 생성한다.

(필수는 아니지만 이렇게 하지 않으면 뒤에서 나오는 빌드 과정 시 ROS2가 패키지를 찾지 못해 직접 옮겨주는 과정이 필요하다.)

```bash
cd ~/ros2_ws/src
```
로 이동 후

```bash
# python 파일의 패키지를 만들고 싶을 때
ros2 pkg create my_pkg --build-type ament_python
```

또는

```bash
# c 또는 cpp 파일의 패키지를 만들고 싶을 때
ros2 pkg create my_pkg --build-type ament_cmake
```

형식으로 패키지를 생성할 수 있다.

노드 이름까지 같이 지정해서 생성하고 싶다면 --node-name 옵션을 추가로 붙여주면 된다.

my_node의 노드를 포함한 my_pkg 패키지 만들기

```bash
# python
ros2 pkg create --build-type ament_python --node-name my_node my_pkg
```
```bash
# c/cpp
ros2 pkg create --build-type ament_python --node-name my_node my_pkg
```

### 5) 빌드(Build)

ROS2 명령어를 통해 실행될 수 있게 패키지를 정리/설치하는 과정.

예를 들어

    my_node.py
    my_node.cpp
    package.xml
    setup.py

이런 상태로는 ROS2가 

    어디 있는지
    실행파일 이름이 뭔지
    의존성이 뭔지

파악할 수 없다.

그래서 build 작업을 통해 패키지를 설치/정리해준다.

**빌드는 반드시 워크스페이스 폴더 위치에서 해준다.**

일반적으로 빌드를 하면 워크스페이스 아래 아래와 같은 폴더들이 생긴다.

    ros2_ws/
    ├── build/
    ├── install/
    ├── log/
    ├── src/

`src` : 원본 코드

`build` : 빌드 중간파일

`install` : 실제 실행에 사용되는 결과물. 결과적으로 이 안의 프로그램들이 실행된다.

`log` : 빌드 로그(기록).

빌드 명령어는 아래와 같다.

```bash
colcon build
```

하지만 아래 명령어를 훨씬 더 많이 사용한다.

```bash
colcon build --symlink-install 
```

이유는 일반 빌드 명령어 `colcon build`의 경우 install 폴더 안의 복사본으로 실행이 되는데,

이러면 원본 파일을 수정해도 실행에 반영이 되지 않아 `colcon build`를 다시 해줘야 하는 번거로움이 생긴다.

`--symlink-install`이라는 옵션을 추가하면 install 폴더가 원본을 링크만 하여

수정내용이 재빌드 없이 바로 반영되는 경우가 많이 위 명령어를 더 많이 사용한다.

만약 특정 패키지만 빌드하고 싶은 경우 --packages-select 옵션을 사용해 빌드할 수 있다.

```bash
colcon build --symlink-install --packages-select my_pkg
```

### 6) 의존성 업데이트

의존성 업데이트 역시 **워크스페이스** 위치에서 한다.

```bash
cd ~/ros2_ws

sudo rosdep init      # 처음 한 번만
rosdep update         # 의존성 DB 업데이트
rosdep install --from-paths src -y --ignore-src
```

```bash
sudo rosdep init
```

rosdep을 처음 사용할 수 있게 초기 설정하는 명령어. 처음 한번만 실행해주면 된다.

간혹 아래 에러가 발생하는 경우는 이미 이 작업을 했다는 뜻으로 무시해주면 된다.

    ERROR: default sources list file already exists

```bash
rosdep update
```
의존성 목록 업데이트 명령어. sudo apt update처럼 비슷한 거로 보면 된다.

```bash
rosdep install --from-paths src --ignore-src
```

최신화된 의존성 목록을 실제로 업데이트 시키는 명령어.

`--from-paths src` : src 폴더 안의 모든 패키지를 검사하겠다는 의미. 즉 ~/ros2_ws/src 안의 package.xml들을 읽음.

`--ignore-src` : src 안에 이미 있는 패키지는 의존성 설치 대상에서 제외.

가령 아래와 같은 구조에서

```bash
ros2_ws/
└── src/
    ├── my_robot_pkg/
    └── custom_sensor_pkg/
```

my_robot_pkg/package.xml중 일부가 아래와 같이 되어있었다고 하면

```md
<depend>custom_sensor_pkg</depend>
<depend>rclpy</depend>
<depend>std_msgs</depend>
```

    custom_sensor_pkg → src 안에 이미 있으니까 설치 안 함
    rclpy             → 시스템 패키지로 필요하면 설치
    std_msgs          → 시스템 패키지로 필요하면 설치


---
## 통신 방식

ROS2에는 3가지 통신 방식이 있다. 이를 표로 정리하면 아래와 같다.

| 통신 방식 | 사용하는 인터페이스 파일 |
| :-----: | -------------- |
| Topic   | .msg           |
| Service | .srv           |
| Action  | .action        |

### 1) Topic

가장 기본적인 통신.

Publisher가 데이터를 발행(Publish)하면 Subscriber가 이를 구독(Subscribe)하는 방식이다.

주로 센서 데이터 전송(카메라 영상, LiDAR 데이터 등)에 사용된다.

단방향 통신이며 `.msg` 형식을 사용한다.

`msg`의 양식은 아래 예시처럼 <변수타입> <변수이름>으로 작성한다.

    float32 temperature
    string robot_name
    bool is_charging

### 2) Service

요청/응답 방식의 통신이다.

질문 → 답변 형태이며 주로 1회성 작업에 사용된다.

Client가 요청(Request)을 보내면 Server가 응답(Response)을 반환한다.

`.srv` 형식을 사용한다.

`srv`의 양식은 아래와 같이 \-\-\- 를 기준으로 윗쪽을 request(요청), 아래쪽을 response(응답)으로 사용한다.

    int64 a
    int64 b
    ---
    int64 sum

### 3) Action

오래 걸리는 작업에 사용되는 통신 방식이다.

Goal(목표) → Feedback(진행상황) → Result(최종결과) 구조를 가진다.

중간 피드백 및 작업 취소가 가능하다.

`.action` 형식을 사용한다.

`action`의 양식은 `srv`와 비슷하게 \-\-\-로 구분되는 구조를 지니지만, 3부분으로 나뉘며, 위에서부터 순서대로 Goal/Result/Feedback으로 구분된다.

    int32 order
    ---
    int32[] sequence
    ---
    int32[] sequence

## 인터페이스 패키지

msg / srv / action 파일들을 모아놓은 패키지

.msg, .srv, .action 파일을 실제 Python/C++에서 import 가능한 코드로 변환하는 작업을 

rosidl_generate_interfaces()가 해주는데, 

이것이 CMakeLists.txt 기반이기 때문에

인터페이스 패키지를 빌드할 때에는 ament_python이 아닌 `ament_cmake`로 빌드해준다.



aaaaaaasssssssdddddd