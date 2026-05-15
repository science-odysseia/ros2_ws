import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile
from ros_study_msgs.msg import MyMsg

class my_msg_test(Node):
    def __init__(self):
        super().__init__('my_msg_test')
        qos_profile = QoSProfile(depth=10)
        self.publisher_ = self.create_publisher(MyMsg, 'my_msg', qos_profile)
        timer_period = 0.5  # seconds
        self.timer = self.create_timer(timer_period, self.timer_callback)
        self.i = 0

    def timer_callback(self):
        msg = MyMsg()
        msg.num = float(self.i)
        self.publisher_.publish(msg)
        self.get_logger().info('Publishing: "%s"' % msg.num)
        self.i += 1

def main(args=None):
    rclpy.init(args=args)
    
    my_msg_test_node = my_msg_test()
    
    rclpy.spin(my_msg_test_node)
    
    my_msg_test_node.destroy_node()
    rclpy.shutdown()
    
if __name__ == '__main__':
    main()