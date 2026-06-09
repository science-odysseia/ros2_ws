#!/usr/bin/env python3
#
# Copyright 2019 ROBOTIS CO., LTD.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Authors: Joep Tool

import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node
from launch.conditions import UnlessCondition


def generate_launch_description():
    launch_file_dir = os.path.join(get_package_share_directory('turtlebot3_gazebo'), 'launch')
    pkg_gazebo_ros = get_package_share_directory('gazebo_ros')
    world = os.path.join(
        get_package_share_directory('turtlebot3_gazebo'),
        'worlds',
        'no_roof_small_warehouse.world'
    )

    gazebo_model_path = os.path.join(
        get_package_share_directory('turtlebot3_gazebo'),
        'models'
    )

    os.environ['GAZEBO_MODEL_PATH'] = f"{gazebo_model_path}:{os.environ.get('GAZEBO_MODEL_PATH', '')}"

    use_sim_time = LaunchConfiguration('use_sim_time', default='true')
    localization = LaunchConfiguration('localization', default='false')
    x_pose = LaunchConfiguration('x_pose', default='-2.0')
    y_pose = LaunchConfiguration('y_pose', default='-0.5')

    gzserver_cmd = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(pkg_gazebo_ros, 'launch', 'gzserver.launch.py')
        ),
        launch_arguments={'world': world}.items()
    )

    gzclient_cmd = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(pkg_gazebo_ros, 'launch', 'gzclient.launch.py')
        )
    )

    robot_state_publisher_cmd = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(launch_file_dir, 'robot_state_publisher.launch.py')
        ),
        launch_arguments={'use_sim_time': use_sim_time}.items()
    )

    spawn_turtlebot_cmd = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(launch_file_dir, 'spawn_turtlebot3.launch.py')
        ),
        launch_arguments={
            'use_sim_time': use_sim_time,
            'x_pose': x_pose,
            'y_pose': y_pose
        }.items()
    )

    parameters={
        'frame_id':'base_footprint',
        'odom_frame_id': 'odom',
        'use_sim_time':use_sim_time,
        'subscribe_depth':True,
        'subscribe_odom': True, 
        'use_action_for_goal':True,
        'Reg/Force3DoF':'true',
        'Grid/RayTracing':'true',
        'Grid/2D': 'true',
        'Grid/3D':'false',
        'Grid/CellSize': '0.1',
        'Grid/RangeMax':'5',
        'Grid/NormalsSegmentation':'false',
        'Grid/MaxGroundHeight':'0.05',
        'Grid/MaxObstacleHeight':'0.4',
        'Optimizer/GravitySigma':'0'
    }

    remappings=[
        ('rgb/image', '/camera/image_raw'),
        ('rgb/camera_info', '/camera/camera_info'),
        ('depth/image', '/camera/depth/image_raw')]

    rtabmap_node = Node(
        condition=UnlessCondition(localization),
        package='rtabmap_slam',
        executable='rtabmap',
        output='screen',
        parameters=[parameters],
        remappings=remappings,
        arguments=['-d'])

    ld = LaunchDescription()

    ld.add_action(gzserver_cmd)
    ld.add_action(gzclient_cmd)
    ld.add_action(robot_state_publisher_cmd)
    ld.add_action(spawn_turtlebot_cmd)
    ld.add_action(rtabmap_node)

    return ld
