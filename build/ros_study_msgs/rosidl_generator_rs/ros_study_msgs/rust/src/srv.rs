#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to ros_study_msgs__srv__MySrv_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MySrv_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub req: f32,

}



impl Default for MySrv_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MySrv_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MySrv_Request {
  type RmwMsg = super::srv::rmw::MySrv_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        req: msg.req,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      req: msg.req,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      req: msg.req,
    }
  }
}


// Corresponds to ros_study_msgs__srv__MySrv_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MySrv_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: f32,

}



impl Default for MySrv_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MySrv_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MySrv_Response {
  type RmwMsg = super::srv::rmw::MySrv_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}






#[link(name = "ros_study_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ros_study_msgs__srv__MySrv() -> *const std::ffi::c_void;
}

// Corresponds to ros_study_msgs__srv__MySrv
#[allow(missing_docs, non_camel_case_types)]
pub struct MySrv;

impl rosidl_runtime_rs::Service for MySrv {
    type Request = MySrv_Request;
    type Response = MySrv_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ros_study_msgs__srv__MySrv() }
    }
}


