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
use crate::interface_types::*; 
use crate::ethernet_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(af_packet_create_a190415f)] 
pub struct AfPacketCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub hw_addr : MacAddress, 
	pub use_random_hw_addr : bool, 
	pub host_if_name : FixedSizeString<U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(af_packet_create_reply_5383d31f)] 
pub struct AfPacketCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(af_packet_delete_863fa648)] 
pub struct AfPacketDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub host_if_name : FixedSizeString<U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(af_packet_delete_reply_e8d4e804)] 
pub struct AfPacketDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(af_packet_set_l4_cksum_offload_319cd5c8)] 
pub struct AfPacketSetL4CksumOffload { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub set : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(af_packet_set_l4_cksum_offload_reply_e8d4e804)] 
pub struct AfPacketSetL4CksumOffloadReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(af_packet_dump_51077d14)] 
pub struct AfPacketDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(af_packet_details_58c7c042)] 
pub struct AfPacketDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub host_if_name : FixedSizeString<U64>, 
} 
