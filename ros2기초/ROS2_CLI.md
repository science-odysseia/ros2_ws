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

### 3. 정보조회 명령어

#### 1) ros2 pkg

```bash
ros2 pkg list
ros2 pkg executables turtlesim
ros2 pkg prefix turtlesim
ros2 pkg xml turtlesim
```






