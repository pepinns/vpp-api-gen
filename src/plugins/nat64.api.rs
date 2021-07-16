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
pub struct NatTimeouts { 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
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
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IfStatusFlags { 
	 IF_STATUS_API_FLAG_ADMIN_UP=1, 
	 IF_STATUS_API_FLAG_LINK_UP=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum MtuProto { 
	 MTU_PROTO_API_L3=0, 
	 MTU_PROTO_API_IP4=1, 
	 MTU_PROTO_API_IP6=2, 
	 MTU_PROTO_API_MPLS=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LinkDuplex { 
	 LINK_DUPLEX_API_UNKNOWN=0, 
	 LINK_DUPLEX_API_HALF=1, 
	 LINK_DUPLEX_API_FULL=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum SubIfFlags { 
	 SUB_IF_API_FLAG_NO_TAGS=1, 
	 SUB_IF_API_FLAG_ONE_TAG=2, 
	 SUB_IF_API_FLAG_TWO_TAGS=4, 
	 SUB_IF_API_FLAG_DOT1AD=8, 
	 SUB_IF_API_FLAG_EXACT_MATCH=16, 
	 SUB_IF_API_FLAG_DEFAULT=32, 
	 SUB_IF_API_FLAG_OUTER_VLAN_ID_ANY=64, 
	 SUB_IF_API_FLAG_INNER_VLAN_ID_ANY=128, 
	 SUB_IF_API_FLAG_MASK_VNET=254, 
	 SUB_IF_API_FLAG_DOT1AH=256, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum RxMode { 
	 RX_MODE_API_UNKNOWN=0, 
	 RX_MODE_API_POLLING=1, 
	 RX_MODE_API_INTERRUPT=2, 
	 RX_MODE_API_ADAPTIVE=3, 
	 RX_MODE_API_DEFAULT=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IfType { 
	 IF_API_TYPE_HARDWARE=0, 
	 IF_API_TYPE_SUB=1, 
	 IF_API_TYPE_P2P=2, 
	 IF_API_TYPE_PIPE=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum Direction { 
	 RX=0, 
	 TX=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum NatLogLevel { 
	 NAT_LOG_NONE=0, 
	 NAT_LOG_ERROR=1, 
	 NAT_LOG_WARNING=2, 
	 NAT_LOG_NOTICE=3, 
	 NAT_LOG_INFO=4, 
	 NAT_LOG_DEBUG=5, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum NatConfigFlags { 
	 NAT_IS_NONE=0, 
	 NAT_IS_TWICE_NAT=1, 
	 NAT_IS_SELF_TWICE_NAT=2, 
	 NAT_IS_OUT2IN_ONLY=4, 
	 NAT_IS_ADDR_ONLY=8, 
	 NAT_IS_OUTSIDE=16, 
	 NAT_IS_INSIDE=32, 
	 NAT_IS_STATIC=64, 
	 NAT_IS_EXT_HOST_VALID=128, 
} 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
pub type InterfaceIndex=u32; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64PluginEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub bib_buckets : u32, 
	pub bib_memory_size : u32, 
	pub st_buckets : u32, 
	pub st_memory_size : u32, 
	pub enable : bool, 
} 
impl Nat64PluginEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_plugin_enable_disable_45948b90") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64PluginEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat64PluginEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_plugin_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64SetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
impl Nat64SetTimeouts { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_set_timeouts_d4746b16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64SetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat64SetTimeoutsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_set_timeouts_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64GetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat64GetTimeouts { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_get_timeouts_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64GetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
impl Nat64GetTimeoutsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_get_timeouts_reply_3c4df4e1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelPoolAddrRange { 
	pub client_index : u32, 
	pub context : u32, 
	pub start_addr : Ip4Address, 
	pub end_addr : Ip4Address, 
	pub vrf_id : u32, 
	pub is_add : bool, 
} 
impl Nat64AddDelPoolAddrRange { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_pool_addr_range_21234ef3") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelPoolAddrRangeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat64AddDelPoolAddrRangeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_pool_addr_range_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64PoolAddrDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat64PoolAddrDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_pool_addr_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64PoolAddrDetails { 
	pub context : u32, 
	pub address : Ip4Address, 
	pub vrf_id : u32, 
} 
impl Nat64PoolAddrDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_pool_addr_details_9bb99cdb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub flags : NatConfigFlags, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Nat64AddDelInterface { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_interface_f3699b83") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat64AddDelInterfaceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_interface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64InterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat64InterfaceDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_interface_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64InterfaceDetails { 
	pub context : u32, 
	pub flags : NatConfigFlags, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Nat64InterfaceDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_interface_details_5d286289") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelStaticBib { 
	pub client_index : u32, 
	pub context : u32, 
	pub i_addr : Ip6Address, 
	pub o_addr : Ip4Address, 
	pub i_port : u16, 
	pub o_port : u16, 
	pub vrf_id : u32, 
	pub proto : u8, 
	pub is_add : bool, 
} 
impl Nat64AddDelStaticBib { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_static_bib_90fae58a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelStaticBibReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat64AddDelStaticBibReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_static_bib_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64BibDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub proto : u8, 
} 
impl Nat64BibDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_bib_dump_cfcb6b75") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64BibDetails { 
	pub context : u32, 
	pub i_addr : Ip6Address, 
	pub o_addr : Ip4Address, 
	pub i_port : u16, 
	pub o_port : u16, 
	pub vrf_id : u32, 
	pub proto : u8, 
	pub flags : NatConfigFlags, 
	pub ses_num : u32, 
} 
impl Nat64BibDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_bib_details_62c8541d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64StDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub proto : u8, 
} 
impl Nat64StDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_st_dump_cfcb6b75") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64StDetails { 
	pub context : u32, 
	pub il_addr : Ip6Address, 
	pub ol_addr : Ip4Address, 
	pub il_port : u16, 
	pub ol_port : u16, 
	pub ir_addr : Ip6Address, 
	pub or_addr : Ip4Address, 
	pub r_port : u16, 
	pub vrf_id : u32, 
	pub proto : u8, 
} 
impl Nat64StDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_st_details_c770d620") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelPrefix { 
	pub client_index : u32, 
	pub context : u32, 
	pub prefix : Ip6Prefix, 
	pub vrf_id : u32, 
	pub is_add : bool, 
} 
impl Nat64AddDelPrefix { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_prefix_727b2f4c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelPrefixReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat64AddDelPrefixReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_prefix_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64PrefixDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat64PrefixDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_prefix_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64PrefixDetails { 
	pub context : u32, 
	pub prefix : Ip6Prefix, 
	pub vrf_id : u32, 
} 
impl Nat64PrefixDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_prefix_details_20568de3") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelInterfaceAddr { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Nat64AddDelInterfaceAddr { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_interface_addr_47d6e753") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat64AddDelInterfaceAddrReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat64AddDelInterfaceAddrReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat64_add_del_interface_addr_reply_e8d4e804") 
	 } 
} 
