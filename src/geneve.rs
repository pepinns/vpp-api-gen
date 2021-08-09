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
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(geneve_add_del_tunnel_976693b5)] 
pub struct GeneveAddDelTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub local_address : Address, 
	pub remote_address : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_next_index : u32, 
	pub vni : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(geneve_add_del_tunnel_reply_5383d31f)] 
pub struct GeneveAddDelTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(geneve_add_del_tunnel2_8c2a9999)] 
pub struct GeneveAddDelTunnel2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub local_address : Address, 
	pub remote_address : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_next_index : u32, 
	pub vni : u32, 
	pub l3_mode : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(geneve_add_del_tunnel2_reply_5383d31f)] 
pub struct GeneveAddDelTunnel2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(geneve_tunnel_dump_f9e6675e)] 
pub struct GeneveTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(geneve_tunnel_details_e27e2748)] 
pub struct GeneveTunnelDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub src_address : Address, 
	pub dst_address : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_next_index : u32, 
	pub vni : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_set_geneve_bypass_65247409)] 
pub struct SwInterfaceSetGeneveBypass { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : bool, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_set_geneve_bypass_reply_e8d4e804)] 
pub struct SwInterfaceSetGeneveBypassReply { 
	pub context : u32, 
	pub retval : i32, 
} 
