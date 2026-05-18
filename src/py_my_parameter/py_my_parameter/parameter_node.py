import rclpy
from rclpy.node import Node
from rcl_interfaces.msg import ParameterType
from rcl_interfaces.msg import SetParametersResult

class ParameterNode(Node):
    def __init__(self):
        super().__init__('parameter_node')
        
        self.declare_parameter('my_param', 'default_value')
        
        param_value = self.get_parameter('my_param').get_parameter_value().string_value
        self.get_logger().info(f'Initial "my_param": {param_value}')
        
        self.timer = self.create_timer(2.0, self.timer_callback)
        
        self.add_on_set_parameters_callback(self.parameter_callback)

    def timer_callback(self):
        current_value = self.get_parameter('my_param').get_parameter_value().string_value
        self.get_logger().info(f'Current "my_param": {current_value}')
        
    def parameter_callback(self, params):
        for param in params:
            if param.name == 'my_param':
                self.get_logger().info(f'Parameter "my_param" updated to: {param.value}')
       
        return SetParametersResult(successful=True)
        
def main(args=None):
    rclpy.init(args=args)
    node = ParameterNode()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()
    
if __name__ == '__main__':
    main()