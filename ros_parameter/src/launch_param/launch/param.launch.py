from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    param_node_01 = Node(
        package = "example_parameters_rclcpp",
        namespace = "rclcpp",
        executable = "parameters_basic",
        parameters = [
            {"rcl_log_level": 40}
        ]
    )
    param_node_02 = Node(
        package = "example_parameters_rclpy",   
        namespace = "rclpy",
        executable = "parameters_basic",
        parameters = [
            {"rcl_log_level": 40}
        ]
    )
    launch_description = LaunchDescription([param_node_01, param_node_02])
    return launch_description

