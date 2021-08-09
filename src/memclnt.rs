/*
   Autogenerated Data, Do not Edit! 
   Author: @felixfaisal 
*/
#![allow(non_camel_case_types)]
use vpp_macros::Message; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
// Implementation for module_version 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ModuleVersion { 
	pub major : u32, 
	pub minor : u32, 
	pub patch : u32, 
	pub name : String, 
} 
// Implementation for message_table_entry 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MessageTableEntry { 
	pub index : u16, 
	pub name : String, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(memclnt_create_9c5e1c2f)] 
pub struct MemclntCreate { 
	pub context : u32, 
	pub ctx_quota : i32, 
	pub input_queue : u64, 
	pub name : FixedSizeString<U64>, 
	pub api_versions : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(memclnt_create_reply_42ec4560)] 
pub struct MemclntCreateReply { 
	pub context : u32, 
	pub response : i32, 
	pub handle : u64, 
	pub index : u32, 
	pub message_table : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(memclnt_delete_7e1c04e3)] 
pub struct MemclntDelete { 
	pub index : u32, 
	pub handle : u64, 
	pub do_cleanup : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(memclnt_delete_reply_3d3b6312)] 
pub struct MemclntDeleteReply { 
	pub response : i32, 
	pub handle : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(rx_thread_exit_c3a3a452)] 
pub struct RxThreadExit { 
	pub dummy : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(memclnt_rx_thread_suspend_c3a3a452)] 
pub struct MemclntRxThreadSuspend { 
	pub dummy : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(memclnt_read_timeout_c3a3a452)] 
pub struct MemclntReadTimeout { 
	pub dummy : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(rpc_call_7e8a2c95)] 
pub struct RpcCall { 
	pub client_index : u32, 
	pub context : u32, 
	pub function : u64, 
	pub multicast : u8, 
	pub need_barrier_sync : u8, 
	pub send_reply : u8, 
	pub data_len : u32, 
	pub data : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(rpc_call_reply_e8d4e804)] 
pub struct RpcCallReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(get_first_msg_id_ebf79a66)] 
pub struct GetFirstMsgId { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(get_first_msg_id_reply_7d337472)] 
pub struct GetFirstMsgIdReply { 
	pub context : u32, 
	pub retval : i32, 
	pub first_msg_id : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(api_versions_51077d14)] 
pub struct ApiVersions { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(api_versions_reply_5f0d99d6)] 
pub struct ApiVersionsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub api_versions : ModuleVersion, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(trace_plugin_msg_ids_f476d3ce)] 
pub struct TracePluginMsgIds { 
	pub client_index : u32, 
	pub context : u32, 
	pub plugin_name : FixedSizeString<U128>, 
	pub first_msg_id : u16, 
	pub last_msg_id : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sockclnt_create_455fb9c4)] 
pub struct SockclntCreate { 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sockclnt_create_reply_35166268)] 
pub struct SockclntCreateReply { 
	pub client_index : u32, 
	pub context : u32, 
	pub response : i32, 
	pub index : u32, 
	pub count : u16, 
	pub message_table : MessageTableEntry, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sockclnt_delete_8ac76db6)] 
pub struct SockclntDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sockclnt_delete_reply_8f38b1ee)] 
pub struct SockclntDeleteReply { 
	pub context : u32, 
	pub response : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sock_init_shm_51646d92)] 
pub struct SockInitShm { 
	pub client_index : u32, 
	pub context : u32, 
	pub requested_size : u32, 
	pub nitems : u8, 
	pub configs : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sock_init_shm_reply_e8d4e804)] 
pub struct SockInitShmReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(memclnt_keepalive_51077d14)] 
pub struct MemclntKeepalive { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(memclnt_keepalive_reply_e8d4e804)] 
pub struct MemclntKeepaliveReply { 
	pub context : u32, 
	pub retval : i32, 
} 
