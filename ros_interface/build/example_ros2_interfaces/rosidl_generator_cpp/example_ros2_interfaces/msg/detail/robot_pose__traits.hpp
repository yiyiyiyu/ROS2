// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from example_ros2_interfaces:msg/RobotPose.idl
// generated code does not contain a copyright notice

#ifndef EXAMPLE_ROS2_INTERFACES__MSG__DETAIL__ROBOT_POSE__TRAITS_HPP_
#define EXAMPLE_ROS2_INTERFACES__MSG__DETAIL__ROBOT_POSE__TRAITS_HPP_

#include "example_ros2_interfaces/msg/detail/robot_pose__struct.hpp"
#include <rosidl_runtime_cpp/traits.hpp>
#include <stdint.h>
#include <type_traits>

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose__traits.hpp"

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<example_ros2_interfaces::msg::RobotPose>()
{
  return "example_ros2_interfaces::msg::RobotPose";
}

template<>
inline const char * name<example_ros2_interfaces::msg::RobotPose>()
{
  return "example_ros2_interfaces/msg/RobotPose";
}

template<>
struct has_fixed_size<example_ros2_interfaces::msg::RobotPose>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::Pose>::value> {};

template<>
struct has_bounded_size<example_ros2_interfaces::msg::RobotPose>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::Pose>::value> {};

template<>
struct is_message<example_ros2_interfaces::msg::RobotPose>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // EXAMPLE_ROS2_INTERFACES__MSG__DETAIL__ROBOT_POSE__TRAITS_HPP_
