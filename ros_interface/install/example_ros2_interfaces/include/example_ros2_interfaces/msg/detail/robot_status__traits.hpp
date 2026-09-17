// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from example_ros2_interfaces:msg/RobotStatus.idl
// generated code does not contain a copyright notice

#ifndef EXAMPLE_ROS2_INTERFACES__MSG__DETAIL__ROBOT_STATUS__TRAITS_HPP_
#define EXAMPLE_ROS2_INTERFACES__MSG__DETAIL__ROBOT_STATUS__TRAITS_HPP_

#include "example_ros2_interfaces/msg/detail/robot_status__struct.hpp"
#include <rosidl_runtime_cpp/traits.hpp>
#include <stdint.h>
#include <type_traits>

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<example_ros2_interfaces::msg::RobotStatus>()
{
  return "example_ros2_interfaces::msg::RobotStatus";
}

template<>
inline const char * name<example_ros2_interfaces::msg::RobotStatus>()
{
  return "example_ros2_interfaces/msg/RobotStatus";
}

template<>
struct has_fixed_size<example_ros2_interfaces::msg::RobotStatus>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<example_ros2_interfaces::msg::RobotStatus>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<example_ros2_interfaces::msg::RobotStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // EXAMPLE_ROS2_INTERFACES__MSG__DETAIL__ROBOT_STATUS__TRAITS_HPP_
