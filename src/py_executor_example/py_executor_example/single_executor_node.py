import time
import rclpy
from rclpy.node import Node
from rclpy.executors import SingleThreadedExecutor


class SingleExecutorNode(Node):

    def __init__(self):
        super().__init__('single_executor_node')
        self.timer1 = self.create_timer(1.0, self.timer1_callback)
        self.timer2 = self.create_timer(1.0, self.timer2_callback)

    def timer1_callback(self):
        self.get_logger().info('timer1 시작 - 3초 작업')
        time.sleep(3)
        self.get_logger().info('timer1 종료')

    def timer2_callback(self):
        self.get_logger().info('timer2 실행')


def main(args=None):
    rclpy.init(args=args)
    node = SingleExecutorNode()
    executor = SingleThreadedExecutor()
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