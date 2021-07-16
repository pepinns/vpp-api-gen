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
pub struct IgmpGroup { 
	pub filter : FilterMode, 
	pub n_srcs : u8, 
	pub sw_if_index : InterfaceIndex, 
	pub gaddr : Ip4Address, 
	pub saddrs : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GroupPrefix { 
	pub typ : GroupPrefixType, 
	pub prefix : Prefix, 
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
#[repr(u32)]
pub enum FilterMode { 
	 EXCLUDE=0, 
	 INCLUDE=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum GroupPrefixType { 
	 ASM=0, 
	 SSM=1, 
} 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
pub type InterfaceIndex=u32; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpListen { 
	pub client_index : u32, 
	pub context : u32, 
	pub group : IgmpGroup, 
} 
impl IgmpListen { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_listen_3f93a51a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpListenReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IgmpListenReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_listen_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
	pub mode : u8, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IgmpEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_enable_disable_b1edfb96") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IgmpEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpProxyDeviceAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub add : u8, 
	pub vrf_id : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IgmpProxyDeviceAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_proxy_device_add_del_0b9be9ce") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpProxyDeviceAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IgmpProxyDeviceAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_proxy_device_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpProxyDeviceAddDelInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub add : bool, 
	pub vrf_id : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IgmpProxyDeviceAddDelInterface { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_proxy_device_add_del_interface_1a9ec24a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpProxyDeviceAddDelInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IgmpProxyDeviceAddDelInterfaceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_proxy_device_add_del_interface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IgmpDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub saddr : Ip4Address, 
	pub gaddr : Ip4Address, 
} 
impl IgmpDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_details_52f12a89") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpClearInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IgmpClearInterface { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_clear_interface_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpClearInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IgmpClearInterfaceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_clear_interface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantIgmpEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : u32, 
	pub pid : u32, 
} 
impl WantIgmpEvents { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_igmp_events_cfaccc1f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantIgmpEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl WantIgmpEventsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_igmp_events_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpEvent { 
	pub sw_if_index : InterfaceIndex, 
	pub filter : FilterMode, 
	pub saddr : Ip4Address, 
	pub gaddr : Ip4Address, 
} 
impl IgmpEvent { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_event_d7696eaf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpGroupPrefixSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub gp : GroupPrefix, 
} 
impl IgmpGroupPrefixSet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_group_prefix_set_d4f20ac5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpGroupPrefixSetReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IgmpGroupPrefixSetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_group_prefix_set_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpGroupPrefixDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IgmpGroupPrefixDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_group_prefix_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IgmpGroupPrefixDetails { 
	pub context : u32, 
	pub gp : GroupPrefix, 
} 
impl IgmpGroupPrefixDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("igmp_group_prefix_details_c3b3c526") 
	 } 
} 
