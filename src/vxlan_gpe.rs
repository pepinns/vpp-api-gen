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
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(vxlan_gpe_add_del_tunnel_7c6da6ae)] 
pub struct VxlanGpeAddDelTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub local : Address, 
	pub remote : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_vrf_id : u32, 
	pub protocol : IpProto, 
	pub vni : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(vxlan_gpe_add_del_tunnel_reply_5383d31f)] 
pub struct VxlanGpeAddDelTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(vxlan_gpe_tunnel_dump_f9e6675e)] 
pub struct VxlanGpeTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(vxlan_gpe_tunnel_details_57712346)] 
pub struct VxlanGpeTunnelDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local : Address, 
	pub remote : Address, 
	pub vni : u32, 
	pub protocol : IpProto, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_vrf_id : u32, 
	pub is_ipv6 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_set_vxlan_gpe_bypass_65247409)] 
pub struct SwInterfaceSetVxlanGpeBypass { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : bool, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(sw_interface_set_vxlan_gpe_bypass_reply_e8d4e804)] 
pub struct SwInterfaceSetVxlanGpeBypassReply { 
	pub context : u32, 
	pub retval : i32, 
} 
