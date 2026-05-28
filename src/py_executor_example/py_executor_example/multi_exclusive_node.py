import time
import rclpy
from rclpy.node import Node
from rclpy.executors import MultiThreadedExecutor
from rclpy.callback_groups import MutuallyExclusiveCallbackGroup

class MultiExecutorNode(Node):
    def __init__(self):
        super().__init__('multi_executor_node')
        self.callback_group1 = MutuallyExclusiveCallbackGroup()                 
        self.callback_group2 = MutuallyExclusiveCallbackGroup()                 

        self.timer1 = self.create_timer(
            1.0,
            self.timer1_callback,
            callback_group=self.callback_group1                                 
        )

        self.timer2 = self.create_timer(
            1.0,
            self.timer2_callback,
            callback_group=self.callback_group2                                 
        )

    def timer1_callback(self):
        self.get_logger().info('timer1 시작 - 3초 작업')
        time.sleep(3)
        self.get_logger().info('timer1 종료')

    def timer2_callback(self):
        self.get_logger().info('timer2 실행')


def main(args=None):
    rclpy.init(args=args)
    node = MultiExecutorNode()
    executor = MultiThreadedExecutor(num_threads=2)
    executor.add_node(node)
    try:
        executor.spin()
    except KeyboardInterrupt:
        pass
    finally:
        executor.shutdown()
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()