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
|info|토픽의 정보 출력|
|echo|토픽의 데이터 출력|
|type|토픽의 타입 출력|
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
|find|지정된 타입을 사용하는 서비스 검색|
|list|서비스 목록 출력|
|type|지정된 서비스의 타입 출력|

#### 3) ros2 action

액션 관련 명령어.

```bash
ros2 action info
ros2 action list
ros2 action send_goal
```
|verb|의미|
|:---:|:---:|
|info|지정된 액션 정보 출력|
|list|액션 목록 출력|
|send_goal|지정된 액션에 목표 전송|

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
|package|지정된 패키지의 인터페이스 목록 출력|
|packages|인터페이스를 가진 패키지 목록 출력|
|proto|지정된 인터페이스의 기본 입력 형태 출력|
|show|지정된 인터페이스 내용(구조) 출력|

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
|get|지정된 파라미터 값 읽기|
|set|지정된 파라미터 값 변경|
|describe|지정된 파라미터 정보 출력|
|dump|파라미터를 yaml 파일로 저장|
|delete|파라미터 삭제|





