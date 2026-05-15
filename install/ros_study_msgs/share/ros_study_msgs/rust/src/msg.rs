#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to ros_study_msgs__msg__MyMsg

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub num: f32,

}



impl Default for MyMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MyMsg::default())
  }
}

impl rosidl_runtime_rs::Message for MyMsg {
  type RmwMsg = super::msg::rmw::MyMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        num: msg.num,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      num: msg.num,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      num: msg.num,
    }
  }
}


