from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    param_node_01 = Node(
        package = "example_parameters_rclcpp",
        executable = "parameters_basic",
    )
    launch_description = LaunchDescription([param_node_01])
    return launch_description

