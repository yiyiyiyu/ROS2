// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from example_ros2_interfaces:srv/MoveRobot.idl
// generated code does not contain a copyright notice

#ifndef EXAMPLE_ROS2_INTERFACES__SRV__DETAIL__MOVE_ROBOT__TRAITS_HPP_
#define EXAMPLE_ROS2_INTERFACES__SRV__DETAIL__MOVE_ROBOT__TRAITS_HPP_

#include "example_ros2_interfaces/srv/detail/move_robot__struct.hpp"
#include <rosidl_runtime_cpp/traits.hpp>
#include <stdint.h>
#include <type_traits>

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<example_ros2_interfaces::srv::MoveRobot_Request>()
{
  return "example_ros2_interfaces::srv::MoveRobot_Request";
}

template<>
inline const char * name<example_ros2_interfaces::srv::MoveRobot_Request>()
{
  return "example_ros2_interfaces/srv/MoveRobot_Request";
}

template<>
struct has_fixed_size<example_ros2_interfaces::srv::MoveRobot_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<example_ros2_interfaces::srv::MoveRobot_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<example_ros2_interfaces::srv::MoveRobot_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<example_ros2_interfaces::srv::MoveRobot_Response>()
{
  return "example_ros2_interfaces::srv::MoveRobot_Response";
}

template<>
inline const char * name<example_ros2_interfaces::srv::MoveRobot_Response>()
{
  return "example_ros2_interfaces/srv/MoveRobot_Response";
}

template<>
struct has_fixed_size<example_ros2_interfaces::srv::MoveRobot_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<example_ros2_interfaces::srv::MoveRobot_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<example_ros2_interfaces::srv::MoveRobot_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<example_ros2_interfaces::srv::MoveRobot>()
{
  return "example_ros2_interfaces::srv::MoveRobot";
}

template<>
inline const char * name<example_ros2_interfaces::srv::MoveRobot>()
{
  return "example_ros2_interfaces/srv/MoveRobot";
}

template<>
struct has_fixed_size<example_ros2_interfaces::srv::MoveRobot>
  : std::integral_constant<
    bool,
    has_fixed_size<example_ros2_interfaces::srv::MoveRobot_Request>::value &&
    has_fixed_size<example_ros2_interfaces::srv::MoveRobot_Response>::value
  >
{
};

template<>
struct has_bounded_size<example_ros2_interfaces::srv::MoveRobot>
  : std::integral_constant<
    bool,
    has_bounded_size<example_ros2_interfaces::srv::MoveRobot_Request>::value &&
    has_bounded_size<example_ros2_interfaces::srv::MoveRobot_Response>::value
  >
{
};

template<>
struct is_service<example_ros2_interfaces::srv::MoveRobot>
  : std::true_type
{
};

template<>
struct is_service_request<example_ros2_interfaces::srv::MoveRobot_Request>
  : std::true_type
{
};

template<>
struct is_service_response<example_ros2_interfaces::srv::MoveRobot_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // EXAMPLE_ROS2_INTERFACES__SRV__DETAIL__MOVE_ROBOT__TRAITS_HPP_
