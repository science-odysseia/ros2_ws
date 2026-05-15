// generated from rosidl_typesupport_c/resource/idl__type_support.cpp.em
// with input from my_robot_interfaces:action/SwitchControl.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "my_robot_interfaces/action/detail/switch_control__struct.h"
#include "my_robot_interfaces/action/detail/switch_control__type_support.h"
#include "rosidl_typesupport_c/identifier.h"
#include "rosidl_typesupport_c/message_type_support_dispatch.h"
#include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_c/visibility_control.h"
#include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_Goal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_Goal_type_support_ids_t;

static const _SwitchControl_Goal_type_support_ids_t _SwitchControl_Goal_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_Goal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_Goal)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_Goal_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_Goal)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_Goal_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_Result_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_Result_type_support_ids_t;

static const _SwitchControl_Result_type_support_ids_t _SwitchControl_Result_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_Result)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_Result)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_Result_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_Result)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_Result_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_Feedback_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_Feedback_type_support_ids_t;

static const _SwitchControl_Feedback_type_support_ids_t _SwitchControl_Feedback_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_Feedback)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_Feedback)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_Feedback_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_Feedback)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_Feedback_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_SendGoal_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_SendGoal_Request_type_support_ids_t;

static const _SwitchControl_SendGoal_Request_type_support_ids_t _SwitchControl_SendGoal_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_SendGoal_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_SendGoal_Request)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_SendGoal_Request_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_SendGoal_Request)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_SendGoal_Request_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_SendGoal_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_SendGoal_Response_type_support_ids_t;

static const _SwitchControl_SendGoal_Response_type_support_ids_t _SwitchControl_SendGoal_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_SendGoal_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_SendGoal_Response)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_SendGoal_Response_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_SendGoal_Response)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_SendGoal_Response_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
#include "rosidl_typesupport_c/service_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_SendGoal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_SendGoal_type_support_ids_t;

static const _SwitchControl_SendGoal_type_support_ids_t _SwitchControl_SendGoal_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_SendGoal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_SendGoal)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_SendGoal_service_typesupport_map),
  rosidl_typesupport_c__get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_SendGoal)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_SendGoal_service_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_GetResult_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_GetResult_Request_type_support_ids_t;

static const _SwitchControl_GetResult_Request_type_support_ids_t _SwitchControl_GetResult_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_GetResult_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_GetResult_Request)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_GetResult_Request_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_GetResult_Request)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_GetResult_Request_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_GetResult_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_GetResult_Response_type_support_ids_t;

static const _SwitchControl_GetResult_Response_type_support_ids_t _SwitchControl_GetResult_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_GetResult_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_GetResult_Response)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_GetResult_Response_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_GetResult_Response)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_GetResult_Response_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/service_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_GetResult_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_GetResult_type_support_ids_t;

static const _SwitchControl_GetResult_type_support_ids_t _SwitchControl_GetResult_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_GetResult)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_GetResult)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_GetResult_service_typesupport_map),
  rosidl_typesupport_c__get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_GetResult)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_GetResult_service_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__struct.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace my_robot_interfaces
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _SwitchControl_FeedbackMessage_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SwitchControl_FeedbackMessage_type_support_ids_t;

static const _SwitchControl_FeedbackMessage_type_support_ids_t _SwitchControl_FeedbackMessage_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
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
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, my_robot_interfaces, action, SwitchControl_FeedbackMessage)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, my_robot_interfaces, action, SwitchControl_FeedbackMessage)),
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
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SwitchControl_FeedbackMessage_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace my_robot_interfaces

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_FeedbackMessage)() {
  return &::my_robot_interfaces::action::rosidl_typesupport_c::SwitchControl_FeedbackMessage_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

#include "action_msgs/msg/goal_status_array.h"
#include "action_msgs/srv/cancel_goal.h"
#include "my_robot_interfaces/action/switch_control.h"
// already included above
// #include "my_robot_interfaces/action/detail/switch_control__type_support.h"

static rosidl_action_type_support_t _my_robot_interfaces__action__SwitchControl__typesupport_c;

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_action_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__ACTION_SYMBOL_NAME(
  rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl)()
{
  // Thread-safe by always writing the same values to the static struct
  _my_robot_interfaces__action__SwitchControl__typesupport_c.goal_service_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(
    rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_SendGoal)();
  _my_robot_interfaces__action__SwitchControl__typesupport_c.result_service_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(
    rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_GetResult)();
  _my_robot_interfaces__action__SwitchControl__typesupport_c.cancel_service_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(
    rosidl_typesupport_c, action_msgs, srv, CancelGoal)();
  _my_robot_interfaces__action__SwitchControl__typesupport_c.feedback_message_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c, my_robot_interfaces, action, SwitchControl_FeedbackMessage)();
  _my_robot_interfaces__action__SwitchControl__typesupport_c.status_message_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c, action_msgs, msg, GoalStatusArray)();

  return &_my_robot_interfaces__action__SwitchControl__typesupport_c;
}

#ifdef __cplusplus
}
#endif
