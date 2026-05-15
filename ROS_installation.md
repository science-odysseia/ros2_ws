## ROS 설치법

필독!!!!!

여기서는 **ROS2 Humble** 버전을 사용할 예정이며

이는 **Ubuntu 22.04** 버전에서 잘 작동하니

반드시 Ubuntu 버전을 아래 명령어로 확인하고 올바른 Ubuntu버전에서 설치할 것.

```bash
lsb_release -a
```

### 1. git 설치

먼저 아래 명령어로 git 설치하기

```bash
sudo apt update
sudo apt install git
```

### 2. 설치파일 권한 부여 및 실행

설치파일이 있는 경로(ROS_install_files)로 이동한 후

아래 명령어로 ros2-humble-desktop-main.sh의 실행 권한을 부여하고

실행하여 설치한다.

```bash
cd ROS2_install_files

chmod +x ros2-humble-desktop-main.sh
./ros2-humble-desktop-main.sh
```

### 3. 잘 설치되었는지 테스트

아래 명령어를 실행하여 다음과 같은 결과가 나오면 성공이다.

```bash
ros2 run examples_rclcpp_minimal_publisher publisher_member_function
```

    [INFO] [1778684689.876385965] [minimal_publisher]: Publishing: 'Hello, world! 0'
    [INFO] [1778684690.376696258] [minimal_publisher]: Publishing: 'Hello, world! 1'
    [INFO] [1778684690.876942436] [minimal_publisher]: Publishing: 'Hello, world! 2'
    [INFO] [1778684691.377209470] [minimal_publisher]: Publishing: 'Hello, world! 3'
    [INFO] [1778684691.877194819] [minimal_publisher]: Publishing: 'Hello, world! 4'
    [INFO] [1778684692.376982036] [minimal_publisher]: Publishing: 'Hello, world! 5'
    [INFO] [1778684692.876624893] [minimal_publisher]: Publishing: 'Hello, world! 6'
    [INFO] [1778684693.376218854] [minimal_publisher]: Publishing: 'Hello, world! 7'
    [INFO] [1778684693.875762615] [minimal_publisher]: Publishing: 'Hello, world! 8'
    [INFO] [1778684694.375013315] [minimal_publisher]: Publishing: 'Hello, world! 9'
    [INFO] [1778684694.874105445] [minimal_publisher]: Publishing: 'Hello, world! 10'

```bash
printenv ROS_DISTRO
```
위 명령어도 실행했을 때 humble이 뜨면 성공.