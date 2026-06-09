## ROS2 CLI

### 0. ros2 -h

ros2 도움말.

사실상 바로 아래 CLI 기본구조에서의 `<command>` 리스트를 보기 위한 코드라고 보면 된다.

```bash
ros2 -h
```

실행하면 command에 대한 설명이 나온다.

### 1. ROS2 CLI 기본구조

    ros2 <command> <verb> [options] [arguments]

( 순서는 명령어에 따라 바뀔 수도 있다. (물론 너무 뒤바뀌면 에러나는 명령어들도 있다.) )

```bash
ros2 topic pub /cmd_vel geometry_msgs/msg/Twist "{linear: {x: 1.0}, angular: {z: 0.5}}" --once
```

`<command>` : topic (토픽 명령어)

`<verb>` : pub (토픽을 발행하겠다.)

`[arguments]` : `/cmd_vel`, `geometry_msgs/msg/Twist`, `"{linear: {x: 1.0}, angular: {z: 0.5}}"` (순서대로 토픽 이름, 토픽 타입, 전송 데이터)

`[options]` : --once (한번만)

\#\# 참고 ( --ros-args 옵션 )

▪ -r __ns:=사용할 네임스페이스

▪ -r __node:=변경할 노드 이름

▪ -r 본래의 토픽/서비스/액션명:=변경할 이름

▪ -p 파라미터 이름:=변경할 파라미터 값

▪ --params-file 파라미터 파일


### 2. 실행 명령어

#### 1) ros2 run

노드 1개 실행명령어.

```bash
ros2 run turtlesim turtlesim_node
```

#### 2) ros2 launch

런치파일 실행명령어.

```bash
ros2 launch demo_nodes_cpp talker_listener.launch.py
```

### 3. ros2 pkg

패키지 관련 명령어.

```bash
ros2 pkg create
ros2 pkg list
ros2 pkg executables turtlesim
ros2 pkg prefix turtlesim
ros2 pkg xml turtlesim
```

|verb|의미|
|:---:|:---:|
|create|패키지 생성|
|list|패키지 목록 출력|
|executables|실행 파일 목록 출력|
|prefix|지정 패키지의 저장 위치 출력|
|xml|지정 패키지의 패키지 정보 파일(xml) 내용 출력|

prefix의 경우 보통 /opt/ros/humble 로 뜨면 그 아래 share 폴더 안에 있는 경우가 많음.

### 4. ros2 node

노드 관련 명령어.

```bash
ros2 node list
ros2 node info /turtlesim
```

|verb|의미|
|:---:|:---:|
|list|실행 중인 모든 노드 목록 출력|
|info|지정한 노드의 정보 출력|

### 5. 3가지 통신 명령어

#### 1) ros2 topic

토픽 관련 명령어.

```bash
ros2 topic list
ros2 topic list -t
ros2 topic info
ros2 topic echo
ros2 topic hz 
ros2 topic bw 
ros2 topic pub
ros2 topic type
ros2 topic find
ros2 topic delay
```

|verb|의미|
|:---:|:---:|
|pub|토픽 발행|
|list|토픽 목록 출력 (`-t` : 타입도 같이 출력하라는 뜻의 option) |
|info|지정 토픽의 정보 출력|
|echo|지정 토픽의 데이터 출력|
|type|지정 토픽의 인터페이스 타입 출력|
|hz, bw|토픽의 주기, 대역폭 출력|
|find|지정 메시지 타입을 사용하는 토픽 찾기|
|delay|지정 토픽의 Publisher → Subscriber까지 걸리는 지연시간(Latency) 측정|

#### 2) ros2 service

서비스 관련 명령어.

```bash
ros2 service call
ros2 service find
ros2 service list
ros2 service type
```
|verb|의미|
|:---:|:---:|
|call|서비스 요청 전송|
|find|지정 타입을 사용하는 서비스 검색|
|list|서비스 목록 출력|
|type|지정 서비스의 인터페이스 타입 출력|

#### 3) ros2 action

액션 관련 명령어.

```bash
ros2 action info
ros2 action list
ros2 action send_goal
```
|verb|의미|
|:---:|:---:|
|info|지정 액션의 정보 출력|
|list|액션 목록 출력|
|send_goal|지정 액션에 목표 전송|

### 6. ros2 interface

인터페이스(msg, srv, action) 관련 명령어.

```bash
ros2 interface list
ros2 interface package
ros2 interface packages
ros2 interface proto
ros2 interface show
```

|verb|의미|
|:---:|:---:|
|list|모든 인터페이스 목록 출력|
|package|지정 패키지의 인터페이스 목록 출력|
|packages|인터페이스를 가진 패키지 목록 출력|
|proto|지정 인터페이스의 기본 입력 형태 출력|
|show|지정 인터페이스 내용(구조) 출력|

`show`는 내용 전체를 출력하고,

`proto`는 바로 붙여넣어 사용가능한 예시 데이터 형식을 출력한다.


### 7. ros2 param

파라미터 관련 명령어.

```bash
ros2 param delete
ros2 param describe	
ros2 param dump
ros2 param get
ros2 param list
ros2 param set
```

|verb|의미|
|:---:|:---:|
|list|파라미터 목록 출력|
|get|지정 파라미터 값 읽기|
|set|지정 파라미터 값 변경|
|describe|지정 파라미터 정보 출력|
|dump|지정 파라미터를 yaml 파일로 저장|
|delete|지정 파라미터 삭제|

### 8. ros2 bag

녹화 관련 명령어.

```bash
ros2 bag record
ros2 bag play	
ros2 bag info
```

|verb|의미|
|:---:|:---:|
|record|지정 토픽 녹화(`-a` : 이 옵션만 붙이면 모든 토픽 녹화)|
|play|녹화했던 지정된 토픽 재생|
|info|저장된 rosbag 정보 출력|

### 9. ros2 multicast

네트워크 확인 명령어

```bash
ros2 multicast receive              # 수신대기
ros2 multicast send                 # 테스트 패킷 전송
```

### 10. ros2 daemon

ROS2 데몬 관리 명령어

daemon이란 사용자 눈에 안 보이는 상태로 백그라운드에서 계속 실행되는 서비스 프로그램이다.

```bash
ros2 daemon start                   # daemon 실행
ros2 daemon stop                    # daemon 종료
ros2 daemon status                  # daemon 실행상태여부(running, not running) 출력
```

### 11. ros2 doctor

ROS2 환경 점검 

```bash
ros2 doctor                         # 기본 진단
ros2 doctor hello                   # 네트워크 연결 확인정보.
ros2 doctor --report                # ros2 doctor -r 과 동일한 명령어. 상세 리포트 출력.
ros2 doctor -rf                     # report-fail. 실패 항목 출력.
ros2 doctor -iw                     # include-warning. 경고 항목 출력.
```

### 12. ros2 wtf

ros2 doctor와 동일한 명령어.

심지어 hello, -r, -rf, -iw도 똑같이 적용된다.

(Weird Thing Finder의 약자. ~~What The Fxxx 아님~~)

### 13. ROS2 환경변수

#### 1) RMW_IMPLEMENTATION

ROS는 DDS 통신 미들웨어를 이용해 통신을 한다.

ROS1은 기본적으로 Master 노드가 있고, 서로가 이 Master 노드를 이용해 찾기 때문에

이 Master 노드가 문제가 생기면 시스템 전체에 영향이 가는 문제점이 있었다.

ROS2는 Master 노드 개념이 없다.

대신 DDS라는 미들웨어가 노드, 토픽, 서비스 등을 발견하고 연결해주는 역할을 해준다.

DDS의 종류에는 FastDDS, CycloneDDS, ConnextDDS 등이 있는데,

RMW_IMPLEMENTATION 환경변수가 이 중 무엇을 사용하는지를 알려준다.

```bash
echo $RMW_IMPLEMENTATION                            # DDS 확인(출력)
export RMW_IMPLEMENTATION=rmw_cyclonedds_cpp        # DDS 종류 설정 또는 변경
```
#### 2) ROS_DOMAIN_ID

기본적으로 이 ROS_DOMAIN_ID라는 환경 변수를 기준으로 통신을 하며

이 환경변수값이 같은 시스템 또는 터미널끼리만 통신이 가능하다.

```bash
echo $ROS_DOMAIN_ID             # ROS_DOMAIN_ID 번호 확인(출력)
export ROS_DOMAIN_ID=30         # ROS_DOMAIN_ID 설정 또는 변경
```

#### 3) ROS_LOCALHOST_ONLY

'로컬 통신만 허용'의 여부

```bash
export ROS_LOCALHOST_ONLY=1     # 1이면 내 PC 안에서만, 0이면 네트워크 통신 허용.
```

#### 4) ROS 배포판 정보

```bash
echo $ROS_VERSION               # 2. ROS2이면 2가, ROS1이면 1이 나온다.
echo $ROS_DISTRO                # Humble. ROS2 배포판 출력.
```

Ros2 응용

[ROS2 응용](../ros2응용/심화개념.md)
