
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_Goal() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__SwitchControl_Goal__init(msg: *mut SwitchControl_Goal) -> bool;
    fn my_robot_interfaces__action__SwitchControl_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Goal>, size: usize) -> bool;
    fn my_robot_interfaces__action__SwitchControl_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Goal>);
    fn my_robot_interfaces__action__SwitchControl_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchControl_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Goal>) -> bool;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchControl_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enable: bool,

}



impl Default for SwitchControl_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__SwitchControl_Goal__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__SwitchControl_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchControl_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchControl_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchControl_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/SwitchControl_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_Goal() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_Result() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__SwitchControl_Result__init(msg: *mut SwitchControl_Result) -> bool;
    fn my_robot_interfaces__action__SwitchControl_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Result>, size: usize) -> bool;
    fn my_robot_interfaces__action__SwitchControl_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Result>);
    fn my_robot_interfaces__action__SwitchControl_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchControl_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Result>) -> bool;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchControl_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SwitchControl_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__SwitchControl_Result__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__SwitchControl_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchControl_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchControl_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchControl_Result where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/SwitchControl_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_Result() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__SwitchControl_Feedback__init(msg: *mut SwitchControl_Feedback) -> bool;
    fn my_robot_interfaces__action__SwitchControl_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Feedback>, size: usize) -> bool;
    fn my_robot_interfaces__action__SwitchControl_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Feedback>);
    fn my_robot_interfaces__action__SwitchControl_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchControl_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_Feedback>) -> bool;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchControl_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_mode: rosidl_runtime_rs::String,

}



impl Default for SwitchControl_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__SwitchControl_Feedback__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__SwitchControl_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchControl_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchControl_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchControl_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/SwitchControl_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_Feedback() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__SwitchControl_FeedbackMessage__init(msg: *mut SwitchControl_FeedbackMessage) -> bool;
    fn my_robot_interfaces__action__SwitchControl_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_FeedbackMessage>, size: usize) -> bool;
    fn my_robot_interfaces__action__SwitchControl_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_FeedbackMessage>);
    fn my_robot_interfaces__action__SwitchControl_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchControl_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_FeedbackMessage>) -> bool;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchControl_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::SwitchControl_Feedback,

}



impl Default for SwitchControl_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__SwitchControl_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__SwitchControl_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchControl_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchControl_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchControl_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/SwitchControl_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_FeedbackMessage() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_Goal() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__Navigate_Goal__init(msg: *mut Navigate_Goal) -> bool;
    fn my_robot_interfaces__action__Navigate_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Navigate_Goal>, size: usize) -> bool;
    fn my_robot_interfaces__action__Navigate_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Navigate_Goal>);
    fn my_robot_interfaces__action__Navigate_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Navigate_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Navigate_Goal>) -> bool;
}

// Corresponds to my_robot_interfaces__action__Navigate_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigate_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_y: f64,

}



impl Default for Navigate_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__Navigate_Goal__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__Navigate_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Navigate_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Navigate_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Navigate_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/Navigate_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_Goal() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_Result() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__Navigate_Result__init(msg: *mut Navigate_Result) -> bool;
    fn my_robot_interfaces__action__Navigate_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Navigate_Result>, size: usize) -> bool;
    fn my_robot_interfaces__action__Navigate_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Navigate_Result>);
    fn my_robot_interfaces__action__Navigate_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Navigate_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Navigate_Result>) -> bool;
}

// Corresponds to my_robot_interfaces__action__Navigate_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigate_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for Navigate_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__Navigate_Result__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__Navigate_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Navigate_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Navigate_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Navigate_Result where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/Navigate_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_Result() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__Navigate_Feedback__init(msg: *mut Navigate_Feedback) -> bool;
    fn my_robot_interfaces__action__Navigate_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Navigate_Feedback>, size: usize) -> bool;
    fn my_robot_interfaces__action__Navigate_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Navigate_Feedback>);
    fn my_robot_interfaces__action__Navigate_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Navigate_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Navigate_Feedback>) -> bool;
}

// Corresponds to my_robot_interfaces__action__Navigate_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigate_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_y: f64,

}



impl Default for Navigate_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__Navigate_Feedback__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__Navigate_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Navigate_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Navigate_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Navigate_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/Navigate_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_Feedback() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__Navigate_FeedbackMessage__init(msg: *mut Navigate_FeedbackMessage) -> bool;
    fn my_robot_interfaces__action__Navigate_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Navigate_FeedbackMessage>, size: usize) -> bool;
    fn my_robot_interfaces__action__Navigate_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Navigate_FeedbackMessage>);
    fn my_robot_interfaces__action__Navigate_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Navigate_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Navigate_FeedbackMessage>) -> bool;
}

// Corresponds to my_robot_interfaces__action__Navigate_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigate_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Navigate_Feedback,

}



impl Default for Navigate_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__Navigate_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__Navigate_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Navigate_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Navigate_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Navigate_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/Navigate_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_FeedbackMessage() }
  }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__SwitchControl_SendGoal_Request__init(msg: *mut SwitchControl_SendGoal_Request) -> bool;
    fn my_robot_interfaces__action__SwitchControl_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_SendGoal_Request>, size: usize) -> bool;
    fn my_robot_interfaces__action__SwitchControl_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_SendGoal_Request>);
    fn my_robot_interfaces__action__SwitchControl_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchControl_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_SendGoal_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchControl_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::SwitchControl_Goal,

}



impl Default for SwitchControl_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__SwitchControl_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__SwitchControl_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchControl_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchControl_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchControl_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/SwitchControl_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_SendGoal_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__SwitchControl_SendGoal_Response__init(msg: *mut SwitchControl_SendGoal_Response) -> bool;
    fn my_robot_interfaces__action__SwitchControl_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_SendGoal_Response>, size: usize) -> bool;
    fn my_robot_interfaces__action__SwitchControl_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_SendGoal_Response>);
    fn my_robot_interfaces__action__SwitchControl_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchControl_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_SendGoal_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchControl_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for SwitchControl_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__SwitchControl_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__SwitchControl_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchControl_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchControl_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchControl_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/SwitchControl_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_SendGoal_Response() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__SwitchControl_GetResult_Request__init(msg: *mut SwitchControl_GetResult_Request) -> bool;
    fn my_robot_interfaces__action__SwitchControl_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_GetResult_Request>, size: usize) -> bool;
    fn my_robot_interfaces__action__SwitchControl_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_GetResult_Request>);
    fn my_robot_interfaces__action__SwitchControl_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchControl_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_GetResult_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchControl_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for SwitchControl_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__SwitchControl_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__SwitchControl_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchControl_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchControl_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchControl_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/SwitchControl_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_GetResult_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__SwitchControl_GetResult_Response__init(msg: *mut SwitchControl_GetResult_Response) -> bool;
    fn my_robot_interfaces__action__SwitchControl_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_GetResult_Response>, size: usize) -> bool;
    fn my_robot_interfaces__action__SwitchControl_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_GetResult_Response>);
    fn my_robot_interfaces__action__SwitchControl_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchControl_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchControl_GetResult_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchControl_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::SwitchControl_Result,

}



impl Default for SwitchControl_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__SwitchControl_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__SwitchControl_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchControl_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__SwitchControl_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchControl_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchControl_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/SwitchControl_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__SwitchControl_GetResult_Response() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__Navigate_SendGoal_Request__init(msg: *mut Navigate_SendGoal_Request) -> bool;
    fn my_robot_interfaces__action__Navigate_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Navigate_SendGoal_Request>, size: usize) -> bool;
    fn my_robot_interfaces__action__Navigate_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Navigate_SendGoal_Request>);
    fn my_robot_interfaces__action__Navigate_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Navigate_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Navigate_SendGoal_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__action__Navigate_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigate_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Navigate_Goal,

}



impl Default for Navigate_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__Navigate_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__Navigate_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Navigate_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Navigate_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Navigate_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/Navigate_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_SendGoal_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__Navigate_SendGoal_Response__init(msg: *mut Navigate_SendGoal_Response) -> bool;
    fn my_robot_interfaces__action__Navigate_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Navigate_SendGoal_Response>, size: usize) -> bool;
    fn my_robot_interfaces__action__Navigate_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Navigate_SendGoal_Response>);
    fn my_robot_interfaces__action__Navigate_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Navigate_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Navigate_SendGoal_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__action__Navigate_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigate_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Navigate_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__Navigate_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__Navigate_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Navigate_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Navigate_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Navigate_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/Navigate_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_SendGoal_Response() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__Navigate_GetResult_Request__init(msg: *mut Navigate_GetResult_Request) -> bool;
    fn my_robot_interfaces__action__Navigate_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Navigate_GetResult_Request>, size: usize) -> bool;
    fn my_robot_interfaces__action__Navigate_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Navigate_GetResult_Request>);
    fn my_robot_interfaces__action__Navigate_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Navigate_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Navigate_GetResult_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__action__Navigate_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigate_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Navigate_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__Navigate_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__Navigate_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Navigate_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Navigate_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Navigate_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/Navigate_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_GetResult_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__Navigate_GetResult_Response__init(msg: *mut Navigate_GetResult_Response) -> bool;
    fn my_robot_interfaces__action__Navigate_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Navigate_GetResult_Response>, size: usize) -> bool;
    fn my_robot_interfaces__action__Navigate_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Navigate_GetResult_Response>);
    fn my_robot_interfaces__action__Navigate_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Navigate_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Navigate_GetResult_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__action__Navigate_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigate_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Navigate_Result,

}



impl Default for Navigate_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__Navigate_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__Navigate_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Navigate_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__Navigate_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Navigate_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Navigate_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/Navigate_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__Navigate_GetResult_Response() }
  }
}






#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__SwitchControl_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct SwitchControl_SendGoal;

impl rosidl_runtime_rs::Service for SwitchControl_SendGoal {
    type Request = SwitchControl_SendGoal_Request;
    type Response = SwitchControl_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__SwitchControl_SendGoal() }
    }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__SwitchControl_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__action__SwitchControl_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct SwitchControl_GetResult;

impl rosidl_runtime_rs::Service for SwitchControl_GetResult {
    type Request = SwitchControl_GetResult_Request;
    type Response = SwitchControl_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__SwitchControl_GetResult() }
    }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__Navigate_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__action__Navigate_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Navigate_SendGoal;

impl rosidl_runtime_rs::Service for Navigate_SendGoal {
    type Request = Navigate_SendGoal_Request;
    type Response = Navigate_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__Navigate_SendGoal() }
    }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__Navigate_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__action__Navigate_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Navigate_GetResult;

impl rosidl_runtime_rs::Service for Navigate_GetResult {
    type Request = Navigate_GetResult_Request;
    type Response = Navigate_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__Navigate_GetResult() }
    }
}


