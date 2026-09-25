
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_Goal() -> *const std::ffi::c_void;
}

#[link(name = "robot_control_interfaces__rosidl_generator_c")]
extern "C" {
    fn robot_control_interfaces__action__MoveRobot_Goal__init(msg: *mut MoveRobot_Goal) -> bool;
    fn robot_control_interfaces__action__MoveRobot_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Goal>, size: usize) -> bool;
    fn robot_control_interfaces__action__MoveRobot_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Goal>);
    fn robot_control_interfaces__action__MoveRobot_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Goal>) -> bool;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f32,

}



impl Default for MoveRobot_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robot_control_interfaces__action__MoveRobot_Goal__init(&mut msg as *mut _) {
        panic!("Call to robot_control_interfaces__action__MoveRobot_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "robot_control_interfaces/action/MoveRobot_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_Goal() }
  }
}


#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_Result() -> *const std::ffi::c_void;
}

#[link(name = "robot_control_interfaces__rosidl_generator_c")]
extern "C" {
    fn robot_control_interfaces__action__MoveRobot_Result__init(msg: *mut MoveRobot_Result) -> bool;
    fn robot_control_interfaces__action__MoveRobot_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Result>, size: usize) -> bool;
    fn robot_control_interfaces__action__MoveRobot_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Result>);
    fn robot_control_interfaces__action__MoveRobot_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Result>) -> bool;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: f32,

}



impl Default for MoveRobot_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robot_control_interfaces__action__MoveRobot_Result__init(&mut msg as *mut _) {
        panic!("Call to robot_control_interfaces__action__MoveRobot_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_Result where Self: Sized {
  const TYPE_NAME: &'static str = "robot_control_interfaces/action/MoveRobot_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_Result() }
  }
}


#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "robot_control_interfaces__rosidl_generator_c")]
extern "C" {
    fn robot_control_interfaces__action__MoveRobot_Feedback__init(msg: *mut MoveRobot_Feedback) -> bool;
    fn robot_control_interfaces__action__MoveRobot_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Feedback>, size: usize) -> bool;
    fn robot_control_interfaces__action__MoveRobot_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Feedback>);
    fn robot_control_interfaces__action__MoveRobot_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Feedback>) -> bool;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u32,

}

impl MoveRobot_Feedback {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_MOVEING: u32 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_STOP: u32 = 4;

}


impl Default for MoveRobot_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robot_control_interfaces__action__MoveRobot_Feedback__init(&mut msg as *mut _) {
        panic!("Call to robot_control_interfaces__action__MoveRobot_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "robot_control_interfaces/action/MoveRobot_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_Feedback() }
  }
}


#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "robot_control_interfaces__rosidl_generator_c")]
extern "C" {
    fn robot_control_interfaces__action__MoveRobot_FeedbackMessage__init(msg: *mut MoveRobot_FeedbackMessage) -> bool;
    fn robot_control_interfaces__action__MoveRobot_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_FeedbackMessage>, size: usize) -> bool;
    fn robot_control_interfaces__action__MoveRobot_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_FeedbackMessage>);
    fn robot_control_interfaces__action__MoveRobot_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_FeedbackMessage>) -> bool;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MoveRobot_Feedback,

}



impl Default for MoveRobot_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robot_control_interfaces__action__MoveRobot_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to robot_control_interfaces__action__MoveRobot_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "robot_control_interfaces/action/MoveRobot_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_FeedbackMessage() }
  }
}




#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "robot_control_interfaces__rosidl_generator_c")]
extern "C" {
    fn robot_control_interfaces__action__MoveRobot_SendGoal_Request__init(msg: *mut MoveRobot_SendGoal_Request) -> bool;
    fn robot_control_interfaces__action__MoveRobot_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_SendGoal_Request>, size: usize) -> bool;
    fn robot_control_interfaces__action__MoveRobot_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_SendGoal_Request>);
    fn robot_control_interfaces__action__MoveRobot_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_SendGoal_Request>) -> bool;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MoveRobot_Goal,

}



impl Default for MoveRobot_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robot_control_interfaces__action__MoveRobot_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to robot_control_interfaces__action__MoveRobot_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robot_control_interfaces/action/MoveRobot_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_SendGoal_Request() }
  }
}


#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "robot_control_interfaces__rosidl_generator_c")]
extern "C" {
    fn robot_control_interfaces__action__MoveRobot_SendGoal_Response__init(msg: *mut MoveRobot_SendGoal_Response) -> bool;
    fn robot_control_interfaces__action__MoveRobot_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_SendGoal_Response>, size: usize) -> bool;
    fn robot_control_interfaces__action__MoveRobot_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_SendGoal_Response>);
    fn robot_control_interfaces__action__MoveRobot_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_SendGoal_Response>) -> bool;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MoveRobot_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robot_control_interfaces__action__MoveRobot_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to robot_control_interfaces__action__MoveRobot_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robot_control_interfaces/action/MoveRobot_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_SendGoal_Response() }
  }
}


#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "robot_control_interfaces__rosidl_generator_c")]
extern "C" {
    fn robot_control_interfaces__action__MoveRobot_GetResult_Request__init(msg: *mut MoveRobot_GetResult_Request) -> bool;
    fn robot_control_interfaces__action__MoveRobot_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_GetResult_Request>, size: usize) -> bool;
    fn robot_control_interfaces__action__MoveRobot_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_GetResult_Request>);
    fn robot_control_interfaces__action__MoveRobot_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_GetResult_Request>) -> bool;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MoveRobot_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robot_control_interfaces__action__MoveRobot_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to robot_control_interfaces__action__MoveRobot_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robot_control_interfaces/action/MoveRobot_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_GetResult_Request() }
  }
}


#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "robot_control_interfaces__rosidl_generator_c")]
extern "C" {
    fn robot_control_interfaces__action__MoveRobot_GetResult_Response__init(msg: *mut MoveRobot_GetResult_Response) -> bool;
    fn robot_control_interfaces__action__MoveRobot_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_GetResult_Response>, size: usize) -> bool;
    fn robot_control_interfaces__action__MoveRobot_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_GetResult_Response>);
    fn robot_control_interfaces__action__MoveRobot_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_GetResult_Response>) -> bool;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MoveRobot_Result,

}



impl Default for MoveRobot_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robot_control_interfaces__action__MoveRobot_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to robot_control_interfaces__action__MoveRobot_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robot_control_interfaces__action__MoveRobot_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robot_control_interfaces/action/MoveRobot_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robot_control_interfaces__action__MoveRobot_GetResult_Response() }
  }
}






#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robot_control_interfaces__action__MoveRobot_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveRobot_SendGoal;

impl rosidl_runtime_rs::Service for MoveRobot_SendGoal {
    type Request = MoveRobot_SendGoal_Request;
    type Response = MoveRobot_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robot_control_interfaces__action__MoveRobot_SendGoal() }
    }
}




#[link(name = "robot_control_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robot_control_interfaces__action__MoveRobot_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to robot_control_interfaces__action__MoveRobot_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveRobot_GetResult;

impl rosidl_runtime_rs::Service for MoveRobot_GetResult {
    type Request = MoveRobot_GetResult_Request;
    type Response = MoveRobot_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robot_control_interfaces__action__MoveRobot_GetResult() }
    }
}


