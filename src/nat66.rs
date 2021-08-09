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
use crate::ip_types::*; 
use crate::interface_types::*; 
use crate::nat_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(nat66_add_del_interface_f3699b83)] 
pub struct Nat66AddDelInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub flags : NatConfigFlags, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(nat66_add_del_interface_reply_e8d4e804)] 
pub struct Nat66AddDelInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(nat66_interface_dump_51077d14)] 
pub struct Nat66InterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(nat66_interface_details_5d286289)] 
pub struct Nat66InterfaceDetails { 
	pub context : u32, 
	pub flags : NatConfigFlags, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(nat66_add_del_static_mapping_fb64e50b)] 
pub struct Nat66AddDelStaticMapping { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub local_ip_address : Ip6Address, 
	pub external_ip_address : Ip6Address, 
	pub vrf_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(nat66_add_del_static_mapping_reply_e8d4e804)] 
pub struct Nat66AddDelStaticMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(nat66_static_mapping_dump_51077d14)] 
pub struct Nat66StaticMappingDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(nat66_static_mapping_details_5c568448)] 
pub struct Nat66StaticMappingDetails { 
	pub context : u32, 
	pub local_ip_address : Ip6Address, 
	pub external_ip_address : Ip6Address, 
	pub vrf_id : u32, 
	pub total_bytes : u64, 
	pub total_pkts : u64, 
} 
