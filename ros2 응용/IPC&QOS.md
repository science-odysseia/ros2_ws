## IPC(Intra-Process Communication)

ROS2는 여러 개의 노드(Node)를 사용하여 시스템을 구성한다.

그리고 일반적으로 노드가 서로 다른 프로세스에서 실행되면 DDS를 거치면서 데이터가 여러 번 복사된다.

![Two_nodes](../imgs/IPC1.png)

이 과정에서 메모리 사용량, CPU 사용량 등이 증가하고, 통신속도 또한 느려질 수 있다.

IPC는 **여러 노드를 하나의 프로세스 안에서 실행** 하여 DDS 복사를 최소화하는 기능이다.

![IPC](../imgs/IPC2.png)

여기서 사용되는 방식이 Zero-Copy이다.

Zero-Copy 방식의 데이터 흐름은 아래와 같다.

1. Publisher가 데이터 생성
2. 공유메모리에 저장
3. Subscriber는 주소만 전달받음

데이터가 한 번만 생성되고 주소만 참조해서 읽는 방식이므로 메모리를 아낄 수 있다.

## QOS(Quality of Service)

DDS 통신의 품질을 조절하는 옵션

### 1) Reliability(신뢰성)

1. `BEST_EFFORT` : 송신에 집중. 전송속도를 중시하여 네트워크에 따른 일부 데이터 유실 가능.
2. `RELIABLE` : 수신에 집중. 신뢰성을 중시하여 유실 발생시 재전송을 통한 수신 보장.

예시

```python
qos_profile = QoSProfile(reliability = QoSReliabilityPolicy.BEST_EFFORT)
``` 

### 2) History(저장 개수)

1. `KEEP_LAST` : 정해진 사이즈(depth 옵션)만큼 데이터를 보관
2. `KEEP_ALL` : 모든 데이터 보관(최대 사이즈는 DDS 벤더마다 다름)
   
예시

```python
qos_profile = QoSProfile(history = QoSHistoryPolicy.KEEP_LAST, depth = 10)
```

### 3) Durability(과거 데이터 전달 여부)

1. `TRANSIENT_LOCAL` : 구독이 생성되기 전 데이터도 보관.(퍼블리셔만 적용가능)
2. `VOLATILE` : 그 반대. 구독이 생성되기 전 데이터 무효.

예시

```python
qos_profile = QoSProfile(durabilty = QoSDurabilityPolicy.TRANSIENT_LOCAL)
```

### 4) Deadline(주기 확인)

정해진 주기 내 데이터의 송수신이 없는경우 이벤트 함수 실행.

`deadline = Duration(t)` : 주기(초)

예시

```python
qos_profile = QoSProfile(depth = 10, deadline = Duration(0.1))
```

### 5) Lifespan(데이터 수명)

정해진 주기 내 수신되는 데이터만 인정.

`lifespan = Duration(t)` : 주기(초)

예시

```python
qos_profile = QoSProfile(lifespan = Duration(0.01))
```

### 6) Liveliness(생존 확인)

정해진 주기 내 노드 또는 토픽의 생사여부 확인.

1. `liveliness` : 어떤 방식(자동 또는 매뉴얼 등)으로 확인할지 지정하는 옵션. AUTOMATIC, MANUAL_BY_NODE, MANUAL_BY_TOPIC 3가지중 하나이다.
2. `liveliness_lease_duration` : Liveliness를 확인하는 주기

예시

```python
qos_profile = QoSProfile(
    liveliness = AUTOMATIC,
    liveliness_lease_duration = Duration(0.1))
```
