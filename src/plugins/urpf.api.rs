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
pub struct FibMplsLabel { 
	pub is_uniform : u8, 
	pub label : u32, 
	pub ttl : u8, 
	pub exp : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FibPathNh { 
	pub address : AddressUnion, 
	pub via_label : u32, 
	pub obj_id : u32, 
	pub classify_table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FibPath { 
	pub sw_if_index : u32, 
	pub table_id : u32, 
	pub rpf_id : u32, 
	pub weight : u8, 
	pub preference : u8, 
	pub typ : FibPathType, 
	pub flags : FibPathFlags, 
	pub proto : FibPathNhProto, 
	pub nh : FibPathNh, 
	pub n_labels : u8, 
	pub label_stack : FibMplsLabel, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
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
pub enum FibPathNhProto { 
	 FIB_API_PATH_NH_PROTO_IP4=0, 
	 FIB_API_PATH_NH_PROTO_IP6=1, 
	 FIB_API_PATH_NH_PROTO_MPLS=2, 
	 FIB_API_PATH_NH_PROTO_ETHERNET=3, 
	 FIB_API_PATH_NH_PROTO_BIER=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum FibPathFlags { 
	 FIB_API_PATH_FLAG_NONE=0, 
	 FIB_API_PATH_FLAG_RESOLVE_VIA_ATTACHED=1, 
	 FIB_API_PATH_FLAG_RESOLVE_VIA_HOST=2, 
	 FIB_API_PATH_FLAG_POP_PW_CW=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum FibPathType { 
	 FIB_API_PATH_TYPE_NORMAL=0, 
	 FIB_API_PATH_TYPE_LOCAL=1, 
	 FIB_API_PATH_TYPE_DROP=2, 
	 FIB_API_PATH_TYPE_UDP_ENCAP=3, 
	 FIB_API_PATH_TYPE_BIER_IMP=4, 
	 FIB_API_PATH_TYPE_ICMP_UNREACH=5, 
	 FIB_API_PATH_TYPE_ICMP_PROHIBIT=6, 
	 FIB_API_PATH_TYPE_SOURCE_LOOKUP=7, 
	 FIB_API_PATH_TYPE_DVR=8, 
	 FIB_API_PATH_TYPE_INTERFACE_RX=9, 
	 FIB_API_PATH_TYPE_CLASSIFY=10, 
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
pub enum UrpfMode { 
	 URPF_API_MODE_OFF=1, 
	 URPF_API_MODE_LOOSE=2, 
	 URPF_API_MODE_STRICT=3, 
} 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
pub type InterfaceIndex=u32; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct UrpfUpdate { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_input : bool, 
	pub mode : UrpfMode, 
	pub af : AddressFamily, 
	pub sw_if_index : InterfaceIndex, 
} 
impl UrpfUpdate { 
	 pub fn get_message_id() -> String { 
	 	 String::from("urpf_update_2bf8a77c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct UrpfUpdateReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl UrpfUpdateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("urpf_update_reply_e8d4e804") 
	 } 
} 
