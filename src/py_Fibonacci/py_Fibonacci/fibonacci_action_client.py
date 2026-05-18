import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient

from action_tutorials_interfaces.action import Fibonacci

class FibonacciActionClient(Node):
    def __init__(self):
        super().__init__('fibonacci_action_client')
        self._client = ActionClient(self, Fibonacci, 'fibonacci')

    def send_goal(self, order):
        self._client.wait_for_server()
        
        goal_msg = Fibonacci.Goal()
        goal_msg.order = order
        self._client.send_goal_async(goal_msg, feedback_callback=self.feedback_callback
                                     ).add_done_callback(self.goal_response_callback)

    def feedback_callback(self, feedback):
        self.get_logger().info(f'Received feedback: {feedback.feedback.partial_sequence}')
        
    def goal_response_callback(self, future):
        goal_handle = future.result()
        
        if not goal_handle.accepted:
            self.get_logger().info('Goal rejected')
            return
        
        self.get_logger().info('Goal accepted')
        
        goal_handle.get_result_async().add_done_callback(self.get_result_callback)
        
    def get_result_callback(self, future):
        result = future.result().result
        self.get_logger().info(f'Result: {result.sequence}')
        
        rclpy.shutdown()
        
def main(args=None):
    rclpy.init(args=args)
    
    client = FibonacciActionClient()
    client.send_goal(10)
    
    rclpy.spin(client)