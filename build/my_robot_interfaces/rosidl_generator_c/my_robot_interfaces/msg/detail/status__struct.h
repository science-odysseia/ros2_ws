// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from my_robot_interfaces:msg/Status.idl
// generated code does not contain a copyright notice

#ifndef MY_ROBOT_INTERFACES__MSG__DETAIL__STATUS__STRUCT_H_
#define MY_ROBOT_INTERFACES__MSG__DETAIL__STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'robot_name'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/Status in the package my_robot_interfaces.
typedef struct my_robot_interfaces__msg__Status
{
  int32_t battery_level;
  bool is_charging;
  rosidl_runtime_c__String robot_name;
} my_robot_interfaces__msg__Status;

// Struct for a sequence of my_robot_interfaces__msg__Status.
typedef struct my_robot_interfaces__msg__Status__Sequence
{
  my_robot_interfaces__msg__Status * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} my_robot_interfaces__msg__Status__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MY_ROBOT_INTERFACES__MSG__DETAIL__STATUS__STRUCT_H_
