/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CryptoSwSchedulerSetWorker { 
	pub client_index : u32, 
	pub context : u32, 
	pub worker_index : u32, 
	pub crypto_enable : bool, 
} 
impl CryptoSwSchedulerSetWorker { 
	 pub fn get_message_id() -> String { 
	 	 String::from("crypto_sw_scheduler_set_worker_b4274502") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CryptoSwSchedulerSetWorkerReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl CryptoSwSchedulerSetWorkerReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("crypto_sw_scheduler_set_worker_reply_e8d4e804") 
	 } 
} 
