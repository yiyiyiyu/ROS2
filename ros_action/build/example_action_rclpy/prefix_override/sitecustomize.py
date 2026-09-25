import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/yi/ros2/ROS2/ros_action/install/example_action_rclpy'
