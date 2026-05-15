// generated from rosidl_typesupport_cpp/resource/idl__type_support.cpp.em
// with input from my_robot_interfaces:action/SwitchControl.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
#include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
#include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_Goal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_Goal_type_support_ids_t;

static const _SwitchControl_Goal_type_support_ids_t _SwitchControl_Goal_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_Goal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_Goal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_Goal_type_support_symbol_names_t _SwitchControl_Goal_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_Goal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_Goal)),
  }
};

typedef struct _SwitchControl_Goal_type_support_data_t
{
  void * data[2];
} _SwitchControl_Goal_type_support_data_t;

static _SwitchControl_Goal_type_support_data_t _SwitchControl_Goal_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_Goal_message_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_Goal_message_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_Goal_message_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_Goal_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SwitchControl_Goal_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_Goal_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_Goal>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_Goal_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_Goal)() {
  return get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_Goal>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_Result_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_Result_type_support_ids_t;

static const _SwitchControl_Result_type_support_ids_t _SwitchControl_Result_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_Result_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_Result_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_Result_type_support_symbol_names_t _SwitchControl_Result_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_Result)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_Result)),
  }
};

typedef struct _SwitchControl_Result_type_support_data_t
{
  void * data[2];
} _SwitchControl_Result_type_support_data_t;

static _SwitchControl_Result_type_support_data_t _SwitchControl_Result_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_Result_message_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_Result_message_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_Result_message_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_Result_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SwitchControl_Result_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_Result_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_Result>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_Result_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_Result)() {
  return get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_Result>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_Feedback_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_Feedback_type_support_ids_t;

static const _SwitchControl_Feedback_type_support_ids_t _SwitchControl_Feedback_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_Feedback_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_Feedback_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_Feedback_type_support_symbol_names_t _SwitchControl_Feedback_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_Feedback)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_Feedback)),
  }
};

typedef struct _SwitchControl_Feedback_type_support_data_t
{
  void * data[2];
} _SwitchControl_Feedback_type_support_data_t;

static _SwitchControl_Feedback_type_support_data_t _SwitchControl_Feedback_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_Feedback_message_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_Feedback_message_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_Feedback_message_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_Feedback_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SwitchControl_Feedback_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_Feedback_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_Feedback>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_Feedback_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_Feedback)() {
  return get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_Feedback>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_SendGoal_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_SendGoal_Request_type_support_ids_t;

static const _SwitchControl_SendGoal_Request_type_support_ids_t _SwitchControl_SendGoal_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_SendGoal_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_SendGoal_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_SendGoal_Request_type_support_symbol_names_t _SwitchControl_SendGoal_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_SendGoal_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_SendGoal_Request)),
  }
};

typedef struct _SwitchControl_SendGoal_Request_type_support_data_t
{
  void * data[2];
} _SwitchControl_SendGoal_Request_type_support_data_t;

static _SwitchControl_SendGoal_Request_type_support_data_t _SwitchControl_SendGoal_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_SendGoal_Request_message_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_SendGoal_Request_message_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_SendGoal_Request_message_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_SendGoal_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SwitchControl_SendGoal_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_SendGoal_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_SendGoal_Request>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_SendGoal_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_SendGoal_Request)() {
  return get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_SendGoal_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_SendGoal_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_SendGoal_Response_type_support_ids_t;

static const _SwitchControl_SendGoal_Response_type_support_ids_t _SwitchControl_SendGoal_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_SendGoal_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_SendGoal_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_SendGoal_Response_type_support_symbol_names_t _SwitchControl_SendGoal_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_SendGoal_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_SendGoal_Response)),
  }
};

typedef struct _SwitchControl_SendGoal_Response_type_support_data_t
{
  void * data[2];
} _SwitchControl_SendGoal_Response_type_support_data_t;

static _SwitchControl_SendGoal_Response_type_support_data_t _SwitchControl_SendGoal_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_SendGoal_Response_message_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_SendGoal_Response_message_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_SendGoal_Response_message_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_SendGoal_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SwitchControl_SendGoal_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_SendGoal_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_SendGoal_Response>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_SendGoal_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_SendGoal_Response)() {
  return get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_SendGoal_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_SendGoal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_SendGoal_type_support_ids_t;

static const _SwitchControl_SendGoal_type_support_ids_t _SwitchControl_SendGoal_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_SendGoal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_SendGoal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_SendGoal_type_support_symbol_names_t _SwitchControl_SendGoal_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_SendGoal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_SendGoal)),
  }
};

typedef struct _SwitchControl_SendGoal_type_support_data_t
{
  void * data[2];
} _SwitchControl_SendGoal_type_support_data_t;

static _SwitchControl_SendGoal_type_support_data_t _SwitchControl_SendGoal_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_SendGoal_service_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_SendGoal_service_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_SendGoal_service_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_SendGoal_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t SwitchControl_SendGoal_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_SendGoal_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<my_robot_interfaces::action::SwitchControl_SendGoal>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_SendGoal_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_SendGoal)() {
  return ::rosidl_typesupport_cpp::get_service_type_support_handle<my_robot_interfaces::action::SwitchControl_SendGoal>();
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_GetResult_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_GetResult_Request_type_support_ids_t;

static const _SwitchControl_GetResult_Request_type_support_ids_t _SwitchControl_GetResult_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_GetResult_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_GetResult_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_GetResult_Request_type_support_symbol_names_t _SwitchControl_GetResult_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_GetResult_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_GetResult_Request)),
  }
};

typedef struct _SwitchControl_GetResult_Request_type_support_data_t
{
  void * data[2];
} _SwitchControl_GetResult_Request_type_support_data_t;

static _SwitchControl_GetResult_Request_type_support_data_t _SwitchControl_GetResult_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_GetResult_Request_message_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_GetResult_Request_message_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_GetResult_Request_message_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_GetResult_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SwitchControl_GetResult_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_GetResult_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_GetResult_Request>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_GetResult_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_GetResult_Request)() {
  return get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_GetResult_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_GetResult_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_GetResult_Response_type_support_ids_t;

static const _SwitchControl_GetResult_Response_type_support_ids_t _SwitchControl_GetResult_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_GetResult_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_GetResult_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_GetResult_Response_type_support_symbol_names_t _SwitchControl_GetResult_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_GetResult_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_GetResult_Response)),
  }
};

typedef struct _SwitchControl_GetResult_Response_type_support_data_t
{
  void * data[2];
} _SwitchControl_GetResult_Response_type_support_data_t;

static _SwitchControl_GetResult_Response_type_support_data_t _SwitchControl_GetResult_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_GetResult_Response_message_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_GetResult_Response_message_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_GetResult_Response_message_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_GetResult_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SwitchControl_GetResult_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_GetResult_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_GetResult_Response>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_GetResult_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_GetResult_Response)() {
  return get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_GetResult_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_GetResult_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_GetResult_type_support_ids_t;

static const _SwitchControl_GetResult_type_support_ids_t _SwitchControl_GetResult_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_GetResult_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_GetResult_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_GetResult_type_support_symbol_names_t _SwitchControl_GetResult_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_GetResult)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_GetResult)),
  }
};

typedef struct _SwitchControl_GetResult_type_support_data_t
{
  void * data[2];
} _SwitchControl_GetResult_type_support_data_t;

static _SwitchControl_GetResult_type_support_data_t _SwitchControl_GetResult_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_GetResult_service_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_GetResult_service_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_GetResult_service_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_GetResult_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t SwitchControl_GetResult_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_GetResult_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<my_robot_interfaces::action::SwitchControl_GetResult>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_GetResult_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_GetResult)() {
  return ::rosidl_typesupport_cpp::get_service_type_support_handle<my_robot_interfaces::action::SwitchControl_GetResult>();
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _SwitchControl_FeedbackMessage_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_FeedbackMessage_type_support_ids_t;

static const _SwitchControl_FeedbackMessage_type_support_ids_t _SwitchControl_FeedbackMessage_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SwitchControl_FeedbackMessage_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SwitchControl_FeedbackMessage_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SwitchControl_FeedbackMessage_type_support_symbol_names_t _SwitchControl_FeedbackMessage_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, my_robot_interfaces, action, SwitchControl_FeedbackMessage)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, my_robot_interfaces, action, SwitchControl_FeedbackMessage)),
  }
};

typedef struct _SwitchControl_FeedbackMessage_type_support_data_t
{
  void * data[2];
} _SwitchControl_FeedbackMessage_type_support_data_t;

static _SwitchControl_FeedbackMessage_type_support_data_t _SwitchControl_FeedbackMessage_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SwitchControl_FeedbackMessage_message_typesupport_map = {
  2,
  "my_robot_interfaces",
  &_SwitchControl_FeedbackMessage_message_typesupport_ids.typesupport_identifier[0],
  &_SwitchControl_FeedbackMessage_message_typesupport_symbol_names.symbol_name[0],
  &_SwitchControl_FeedbackMessage_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SwitchControl_FeedbackMessage_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_FeedbackMessage_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_FeedbackMessage>()
{
  return &::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_FeedbackMessage_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl_FeedbackMessage)() {
  return get_message_type_support_handle<my_robot_interfaces::action::SwitchControl_FeedbackMessage>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

#include "action_msgs/msg/goal_status_array.hpp"
#include "action_msgs/srv/cancel_goal.hpp"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_runtime_c/action_type_support_struct.h"
#include "rosidl_typesupport_cpp/action_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

static rosidl_action_type_support_t SwitchControl_action_type_support_handle = {
  NULL, NULL, NULL, NULL, NULL};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace my_robot_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_action_type_support_t *
get_action_type_support_handle<my_robot_interfaces::action::SwitchControl>()
{
  using ::my_robot_interfaces::action::rosidl_typesupport_cpp::SwitchControl_action_type_support_handle;
  // Thread-safe by always writing the same values to the static struct
  SwitchControl_action_type_support_handle.goal_service_type_support = get_service_type_support_handle<::my_robot_interfaces::action::SwitchControl::Impl::SendGoalService>();
  SwitchControl_action_type_support_handle.result_service_type_support = get_service_type_support_handle<::my_robot_interfaces::action::SwitchControl::Impl::GetResultService>();
  SwitchControl_action_type_support_handle.cancel_service_type_support = get_service_type_support_handle<::my_robot_interfaces::action::SwitchControl::Impl::CancelGoalService>();
  SwitchControl_action_type_support_handle.feedback_message_type_support = get_message_type_support_handle<::my_robot_interfaces::action::SwitchControl::Impl::FeedbackMessage>();
  SwitchControl_action_type_support_handle.status_message_type_support = get_message_type_support_handle<::my_robot_interfaces::action::SwitchControl::Impl::GoalStatusMessage>();
  return &SwitchControl_action_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_action_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__ACTION_SYMBOL_NAME(rosidl_typesupport_cpp, my_robot_interfaces, action, SwitchControl)() {
  return ::rosidl_typesupport_cpp::get_action_type_support_handle<my_robot_interfaces::action::SwitchControl>();
}

#ifdef __cplusplus
}
#endif
