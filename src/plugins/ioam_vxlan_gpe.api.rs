/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Address { 
	pub af : AddressFamily, 
	pub un : AddressUnion, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Prefix { 
	pub address : Address, 
	pub len : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip4AddressAndMask { 
	pub addr : Ip4Address, 
	pub mask : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6AddressAndMask { 
	pub addr : Ip6Address, 
	pub mask : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Mprefix { 
	pub af : AddressFamily, 
	pub grp_address_length : u16, 
	pub grp_address : AddressUnion, 
	pub src_address : AddressUnion, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip6Prefix { 
	pub address : Ip6Address, 
	pub len : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ip4Prefix { 
	pub address : Ip4Address, 
	pub len : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PrefixMatcher { 
	pub le : u8, 
	pub ge : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum AddressFamily { 
	 ADDRESS_IP4=0, 
	 ADDRESS_IP6=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpFeatureLocation { 
	 IP_API_FEATURE_INPUT=0, 
	 IP_API_FEATURE_OUTPUT=1, 
	 IP_API_FEATURE_LOCAL=2, 
	 IP_API_FEATURE_PUNT=3, 
	 IP_API_FEATURE_DROP=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpEcn { 
	 IP_API_ECN_NONE=0, 
	 IP_API_ECN_ECT0=1, 
	 IP_API_ECN_ECT1=2, 
	 IP_API_ECN_CE=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpDscp { 
	 IP_API_DSCP_CS0=0, 
	 IP_API_DSCP_CS1=8, 
	 IP_API_DSCP_AF11=10, 
	 IP_API_DSCP_AF12=12, 
	 IP_API_DSCP_AF13=14, 
	 IP_API_DSCP_CS2=16, 
	 IP_API_DSCP_AF21=18, 
	 IP_API_DSCP_AF22=20, 
	 IP_API_DSCP_AF23=22, 
	 IP_API_DSCP_CS3=24, 
	 IP_API_DSCP_AF31=26, 
	 IP_API_DSCP_AF32=28, 
	 IP_API_DSCP_AF33=30, 
	 IP_API_DSCP_CS4=32, 
	 IP_API_DSCP_AF41=34, 
	 IP_API_DSCP_AF42=36, 
	 IP_API_DSCP_AF43=38, 
	 IP_API_DSCP_CS5=40, 
	 IP_API_DSCP_EF=46, 
	 IP_API_DSCP_CS6=48, 
	 IP_API_DSCP_CS7=50, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpProto { 
	 IP_API_PROTO_HOPOPT=0, 
	 IP_API_PROTO_ICMP=1, 
	 IP_API_PROTO_IGMP=2, 
	 IP_API_PROTO_TCP=6, 
	 IP_API_PROTO_UDP=17, 
	 IP_API_PROTO_GRE=47, 
	 IP_API_PROTO_ESP=50, 
	 IP_API_PROTO_AH=51, 
	 IP_API_PROTO_ICMP6=58, 
	 IP_API_PROTO_EIGRP=88, 
	 IP_API_PROTO_OSPF=89, 
	 IP_API_PROTO_SCTP=132, 
	 IP_API_PROTO_RESERVED=255, 
} 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamEnable { 
	pub client_index : u32, 
	pub context : u32, 
	pub id : u16, 
	pub trace_ppc : u8, 
	pub pow_enable : bool, 
	pub trace_enable : bool, 
} 
impl VxlanGpeIoamEnable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_enable_2481bef7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamEnableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VxlanGpeIoamEnableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_enable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub id : u16, 
} 
impl VxlanGpeIoamDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_disable_6b16a45e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VxlanGpeIoamDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamVniEnable { 
	pub client_index : u32, 
	pub context : u32, 
	pub vni : u32, 
	pub local : Address, 
	pub remote : Address, 
} 
impl VxlanGpeIoamVniEnable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_vni_enable_997161fb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamVniEnableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VxlanGpeIoamVniEnableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_vni_enable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamVniDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub vni : u32, 
	pub local : Address, 
	pub remote : Address, 
} 
impl VxlanGpeIoamVniDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_vni_disable_997161fb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamVniDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VxlanGpeIoamVniDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_vni_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamTransitEnable { 
	pub client_index : u32, 
	pub context : u32, 
	pub outer_fib_index : u32, 
	pub dst_addr : Address, 
} 
impl VxlanGpeIoamTransitEnable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_transit_enable_553f5b7b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamTransitEnableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VxlanGpeIoamTransitEnableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_transit_enable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamTransitDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub outer_fib_index : u32, 
	pub dst_addr : Address, 
} 
impl VxlanGpeIoamTransitDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_transit_disable_553f5b7b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeIoamTransitDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VxlanGpeIoamTransitDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vxlan_gpe_ioam_transit_disable_reply_e8d4e804") 
	 } 
} 
