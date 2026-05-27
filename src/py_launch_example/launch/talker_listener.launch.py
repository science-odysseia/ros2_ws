from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    talker_node = Node(
        package='py_launch_example',
        executable='talker_node',
        name='talker'
    )

    listener_node = Node(
        package='py_launch_example',
        executable='listener_node',
        name='listener'
    )

    return LaunchDescription([
        talker_node,
        listener_node
    ])