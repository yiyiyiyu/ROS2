# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target robot_control_interfaces::robot_control_interfaces
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${robot_control_interfaces_TARGETS}.
if(robot_control_interfaces_TARGETS AND NOT TARGET robot_control_interfaces::robot_control_interfaces)
  add_library(robot_control_interfaces::robot_control_interfaces INTERFACE IMPORTED)
  set_target_properties(robot_control_interfaces::robot_control_interfaces PROPERTIES
    INTERFACE_LINK_LIBRARIES "${robot_control_interfaces_TARGETS}")
endif()
