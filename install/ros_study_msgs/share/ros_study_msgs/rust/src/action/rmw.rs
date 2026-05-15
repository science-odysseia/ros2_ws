
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_Goal() -> *const std::ffi::c_void;
}

#[link(name = "ros_study_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_study_msgs__action__MyAction_Goal__init(msg: *mut MyAction_Goal) -> bool;
    fn ros_study_msgs__action__MyAction_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MyAction_Goal>, size: usize) -> bool;
    fn ros_study_msgs__action__MyAction_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MyAction_Goal>);
    fn ros_study_msgs__action__MyAction_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MyAction_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MyAction_Goal>) -> bool;
}

// Corresponds to ros_study_msgs__action__MyAction_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyAction_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub go: f32,

}



impl Default for MyAction_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_study_msgs__action__MyAction_Goal__init(&mut msg as *mut _) {
        panic!("Call to ros_study_msgs__action__MyAction_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MyAction_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MyAction_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MyAction_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "ros_study_msgs/action/MyAction_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_Goal() }
  }
}


#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_Result() -> *const std::ffi::c_void;
}

#[link(name = "ros_study_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_study_msgs__action__MyAction_Result__init(msg: *mut MyAction_Result) -> bool;
    fn ros_study_msgs__action__MyAction_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MyAction_Result>, size: usize) -> bool;
    fn ros_study_msgs__action__MyAction_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MyAction_Result>);
    fn ros_study_msgs__action__MyAction_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MyAction_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MyAction_Result>) -> bool;
}

// Corresponds to ros_study_msgs__action__MyAction_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyAction_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: f32,

}



impl Default for MyAction_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_study_msgs__action__MyAction_Result__init(&mut msg as *mut _) {
        panic!("Call to ros_study_msgs__action__MyAction_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MyAction_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MyAction_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MyAction_Result where Self: Sized {
  const TYPE_NAME: &'static str = "ros_study_msgs/action/MyAction_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_Result() }
  }
}


#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "ros_study_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_study_msgs__action__MyAction_Feedback__init(msg: *mut MyAction_Feedback) -> bool;
    fn ros_study_msgs__action__MyAction_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MyAction_Feedback>, size: usize) -> bool;
    fn ros_study_msgs__action__MyAction_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MyAction_Feedback>);
    fn ros_study_msgs__action__MyAction_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MyAction_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MyAction_Feedback>) -> bool;
}

// Corresponds to ros_study_msgs__action__MyAction_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyAction_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub str: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for MyAction_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_study_msgs__action__MyAction_Feedback__init(&mut msg as *mut _) {
        panic!("Call to ros_study_msgs__action__MyAction_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MyAction_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MyAction_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MyAction_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "ros_study_msgs/action/MyAction_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_Feedback() }
  }
}


#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "ros_study_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_study_msgs__action__MyAction_FeedbackMessage__init(msg: *mut MyAction_FeedbackMessage) -> bool;
    fn ros_study_msgs__action__MyAction_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MyAction_FeedbackMessage>, size: usize) -> bool;
    fn ros_study_msgs__action__MyAction_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MyAction_FeedbackMessage>);
    fn ros_study_msgs__action__MyAction_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MyAction_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MyAction_FeedbackMessage>) -> bool;
}

// Corresponds to ros_study_msgs__action__MyAction_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyAction_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MyAction_Feedback,

}



impl Default for MyAction_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_study_msgs__action__MyAction_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to ros_study_msgs__action__MyAction_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MyAction_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MyAction_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MyAction_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "ros_study_msgs/action/MyAction_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_FeedbackMessage() }
  }
}




#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "ros_study_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_study_msgs__action__MyAction_SendGoal_Request__init(msg: *mut MyAction_SendGoal_Request) -> bool;
    fn ros_study_msgs__action__MyAction_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MyAction_SendGoal_Request>, size: usize) -> bool;
    fn ros_study_msgs__action__MyAction_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MyAction_SendGoal_Request>);
    fn ros_study_msgs__action__MyAction_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MyAction_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MyAction_SendGoal_Request>) -> bool;
}

// Corresponds to ros_study_msgs__action__MyAction_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyAction_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MyAction_Goal,

}



impl Default for MyAction_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_study_msgs__action__MyAction_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to ros_study_msgs__action__MyAction_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MyAction_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MyAction_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MyAction_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ros_study_msgs/action/MyAction_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_SendGoal_Request() }
  }
}


#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "ros_study_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_study_msgs__action__MyAction_SendGoal_Response__init(msg: *mut MyAction_SendGoal_Response) -> bool;
    fn ros_study_msgs__action__MyAction_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MyAction_SendGoal_Response>, size: usize) -> bool;
    fn ros_study_msgs__action__MyAction_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MyAction_SendGoal_Response>);
    fn ros_study_msgs__action__MyAction_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MyAction_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MyAction_SendGoal_Response>) -> bool;
}

// Corresponds to ros_study_msgs__action__MyAction_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyAction_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MyAction_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_study_msgs__action__MyAction_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to ros_study_msgs__action__MyAction_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MyAction_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MyAction_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MyAction_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ros_study_msgs/action/MyAction_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_SendGoal_Response() }
  }
}


#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "ros_study_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_study_msgs__action__MyAction_GetResult_Request__init(msg: *mut MyAction_GetResult_Request) -> bool;
    fn ros_study_msgs__action__MyAction_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MyAction_GetResult_Request>, size: usize) -> bool;
    fn ros_study_msgs__action__MyAction_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MyAction_GetResult_Request>);
    fn ros_study_msgs__action__MyAction_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MyAction_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MyAction_GetResult_Request>) -> bool;
}

// Corresponds to ros_study_msgs__action__MyAction_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyAction_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MyAction_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_study_msgs__action__MyAction_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to ros_study_msgs__action__MyAction_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MyAction_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MyAction_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MyAction_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ros_study_msgs/action/MyAction_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_GetResult_Request() }
  }
}


#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "ros_study_msgs__rosidl_generator_c")]
extern "C" {
    fn ros_study_msgs__action__MyAction_GetResult_Response__init(msg: *mut MyAction_GetResult_Response) -> bool;
    fn ros_study_msgs__action__MyAction_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MyAction_GetResult_Response>, size: usize) -> bool;
    fn ros_study_msgs__action__MyAction_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MyAction_GetResult_Response>);
    fn ros_study_msgs__action__MyAction_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MyAction_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MyAction_GetResult_Response>) -> bool;
}

// Corresponds to ros_study_msgs__action__MyAction_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyAction_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MyAction_Result,

}



impl Default for MyAction_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros_study_msgs__action__MyAction_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to ros_study_msgs__action__MyAction_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MyAction_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros_study_msgs__action__MyAction_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MyAction_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MyAction_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ros_study_msgs/action/MyAction_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros_study_msgs__action__MyAction_GetResult_Response() }
  }
}






#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ros_study_msgs__action__MyAction_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to ros_study_msgs__action__MyAction_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MyAction_SendGoal;

impl rosidl_runtime_rs::Service for MyAction_SendGoal {
    type Request = MyAction_SendGoal_Request;
    type Response = MyAction_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ros_study_msgs__action__MyAction_SendGoal() }
    }
}




#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ros_study_msgs__action__MyAction_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to ros_study_msgs__action__MyAction_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MyAction_GetResult;

impl rosidl_runtime_rs::Service for MyAction_GetResult {
    type Request = MyAction_GetResult_Request;
    type Response = MyAction_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ros_study_msgs__action__MyAction_GetResult() }
    }
}


