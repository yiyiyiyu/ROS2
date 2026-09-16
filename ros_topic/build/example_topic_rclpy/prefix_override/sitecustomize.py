import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/yunzhou/node/ROS2/ros_topic/install/example_topic_rclpy'
