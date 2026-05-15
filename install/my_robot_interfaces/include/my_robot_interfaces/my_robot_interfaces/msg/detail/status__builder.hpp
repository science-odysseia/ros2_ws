// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from my_robot_interfaces:msg/Status.idl
// generated code does not contain a copyright notice

#ifndef MY_ROBOT_INTERFACES__MSG__DETAIL__STATUS__BUILDER_HPP_
#define MY_ROBOT_INTERFACES__MSG__DETAIL__STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "my_robot_interfaces/msg/detail/status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace my_robot_interfaces
{

namespace msg
{

namespace builder
{

class Init_Status_robot_name
{
public:
  explicit Init_Status_robot_name(::my_robot_interfaces::msg::Status & msg)
  : msg_(msg)
  {}
  ::my_robot_interfaces::msg::Status robot_name(::my_robot_interfaces::msg::Status::_robot_name_type arg)
  {
    msg_.robot_name = std::move(arg);
    return std::move(msg_);
  }

private:
  ::my_robot_interfaces::msg::Status msg_;
};

class Init_Status_is_charging
{
public:
  explicit Init_Status_is_charging(::my_robot_interfaces::msg::Status & msg)
  : msg_(msg)
  {}
  Init_Status_robot_name is_charging(::my_robot_interfaces::msg::Status::_is_charging_type arg)
  {
    msg_.is_charging = std::move(arg);
    return Init_Status_robot_name(msg_);
  }

private:
  ::my_robot_interfaces::msg::Status msg_;
};

class Init_Status_battery_level
{
public:
  Init_Status_battery_level()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Status_is_charging battery_level(::my_robot_interfaces::msg::Status::_battery_level_type arg)
  {
    msg_.battery_level = std::move(arg);
    return Init_Status_is_charging(msg_);
  }

private:
  ::my_robot_interfaces::msg::Status msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::my_robot_interfaces::msg::Status>()
{
  return my_robot_interfaces::msg::builder::Init_Status_battery_level();
}

}  // namespace my_robot_interfaces

#endif  // MY_ROBOT_INTERFACES__MSG__DETAIL__STATUS__BUILDER_HPP_
