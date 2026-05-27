from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    talker_node = Node(
        package='py_launch_example',
        executable='talker_node',
        name='talker',
        namespace='robot1',
        remappings=[
            ('chatter', 'robot_chatter')
        ]
    )

    listener_node = Node(
        package='py_launch_example',
        executable='listener_node',
        name='listener',
        namespace='robot1',
        remappings=[
            ('chatter', 'robot_chatter')
        ]
    )

    return LaunchDescription([
        talker_node,
        listener_node
    ])