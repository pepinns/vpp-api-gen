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
pub struct LocalLocator { 
	pub sw_if_index : InterfaceIndex, 
	pub priority : u8, 
	pub weight : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct RemoteLocator { 
	pub priority : u8, 
	pub weight : u8, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nsh { 
	pub spi : u32, 
	pub si : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Eid { 
	pub typ : EidType, 
	pub address : EidAddress, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct HmacKey { 
	pub id : HmacKeyId, 
	pub key : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneL2ArpEntry { 
	pub mac : MacAddress, 
	pub ip4 : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneNdpEntry { 
	pub mac : MacAddress, 
	pub ip6 : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAdjacency { 
	pub reid : Eid, 
	pub leid : Eid, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union eid_address { 
	 prefix : Prefix, 
	 mac : MacAddress, 
	 nsh : Nsh, 
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
pub enum EidType { 
	 EID_TYPE_API_PREFIX=0, 
	 EID_TYPE_API_MAC=1, 
	 EID_TYPE_API_NSH=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum HmacKeyId { 
	 KEY_ID_API_HMAC_NO_KEY=0, 
	 KEY_ID_API_HMAC_SHA_1_96=1, 
	 KEY_ID_API_HMAC_SHA_256_128=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum OneMapMode { 
	 ONE_MAP_MODE_API_DST_ONLY=0, 
	 ONE_MAP_MODE_API_SRC_DST=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum OneFilter { 
	 ONE_FILTER_API_ALL=0, 
	 ONE_FILTER_API_LOCAL=1, 
	 ONE_FILTER_API_REMOTE=2, 
} 
pub type InterfaceIndex=u32; 
pub type MacAddress=[u8;6]; 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelLocatorSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<U64>, 
	pub locator_num : u32, 
	pub locators : LocalLocator, 
} 
impl OneAddDelLocatorSet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_locator_set_6fcd6471") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelLocatorSetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ls_index : u32, 
} 
impl OneAddDelLocatorSetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_locator_set_reply_b6666db4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelLocator { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<U64>, 
	pub sw_if_index : InterfaceIndex, 
	pub priority : u8, 
	pub weight : u8, 
} 
impl OneAddDelLocator { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_locator_af4d8f13") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelLocatorReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelLocatorReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_locator_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelLocalEid { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub eid : Eid, 
	pub locator_set_name : FixedSizeString<U64>, 
	pub vni : u32, 
	pub key : HmacKey, 
} 
impl OneAddDelLocalEid { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_local_eid_21f573bd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelLocalEidReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelLocalEidReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_local_eid_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapRegisterSetTtl { 
	pub client_index : u32, 
	pub context : u32, 
	pub ttl : u32, 
} 
impl OneMapRegisterSetTtl { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_register_set_ttl_dd59f1f3") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapRegisterSetTtlReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneMapRegisterSetTtlReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_register_set_ttl_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneMapRegisterTtl { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneMapRegisterTtl { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_map_register_ttl_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneMapRegisterTtlReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ttl : u32, 
} 
impl ShowOneMapRegisterTtlReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_map_register_ttl_reply_fa83dd66") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelMapServer { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ip_address : Address, 
} 
impl OneAddDelMapServer { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_map_server_6598ea7c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelMapServerReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelMapServerReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_map_server_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelMapResolver { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ip_address : Address, 
} 
impl OneAddDelMapResolver { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_map_resolver_6598ea7c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelMapResolverReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelMapResolverReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_map_resolver_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
impl OneEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_enable_disable_c264d7bf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneNshSetLocatorSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ls_name : FixedSizeString<U64>, 
} 
impl OneNshSetLocatorSet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_nsh_set_locator_set_486e2b76") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneNshSetLocatorSetReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneNshSetLocatorSetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_nsh_set_locator_set_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OnePitrSetLocatorSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ls_name : FixedSizeString<U64>, 
} 
impl OnePitrSetLocatorSet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_pitr_set_locator_set_486e2b76") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OnePitrSetLocatorSetReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OnePitrSetLocatorSetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_pitr_set_locator_set_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneUsePetr { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Address, 
	pub is_add : bool, 
} 
impl OneUsePetr { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_use_petr_9e141831") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneUsePetrReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneUsePetrReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_use_petr_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneUsePetr { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneUsePetr { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_use_petr_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneUsePetrReply { 
	pub context : u32, 
	pub retval : i32, 
	pub status : bool, 
	pub ip_address : Address, 
} 
impl ShowOneUsePetrReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_use_petr_reply_10e744a6") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneRlocProbeState { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneRlocProbeState { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_rloc_probe_state_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneRlocProbeStateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
impl ShowOneRlocProbeStateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_rloc_probe_state_reply_f15abb16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneRlocProbeEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
impl OneRlocProbeEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_rloc_probe_enable_disable_c264d7bf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneRlocProbeEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneRlocProbeEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_rloc_probe_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapRegisterEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
impl OneMapRegisterEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_register_enable_disable_c264d7bf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapRegisterEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneMapRegisterEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_register_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneMapRegisterState { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneMapRegisterState { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_map_register_state_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneMapRegisterStateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
impl ShowOneMapRegisterStateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_map_register_state_reply_f15abb16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapRequestMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub mode : OneMapMode, 
} 
impl OneMapRequestMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_request_mode_ffa5d2f5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapRequestModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneMapRequestModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_request_mode_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneMapRequestMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneMapRequestMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_map_request_mode_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneMapRequestModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub mode : OneMapMode, 
} 
impl ShowOneMapRequestModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_map_request_mode_reply_d41f3c1d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelRemoteMapping { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub is_src_dst : bool, 
	pub del_all : bool, 
	pub vni : u32, 
	pub action : u8, 
	pub deid : Eid, 
	pub seid : Eid, 
	pub rloc_num : u32, 
	pub rlocs : RemoteLocator, 
} 
impl OneAddDelRemoteMapping { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_remote_mapping_fae8ed77") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelRemoteMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelRemoteMappingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_remote_mapping_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelL2ArpEntry { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub bd : u32, 
	pub entry : OneL2ArpEntry, 
} 
impl OneAddDelL2ArpEntry { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_l2_arp_entry_33209078") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelL2ArpEntryReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelL2ArpEntryReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_l2_arp_entry_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneL2ArpEntriesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub bd : u32, 
} 
impl OneL2ArpEntriesGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_l2_arp_entries_get_4d418cf4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneL2ArpEntriesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub entries : OneL2ArpEntry, 
} 
impl OneL2ArpEntriesGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_l2_arp_entries_get_reply_b0a47bbe") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelNdpEntry { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub bd : u32, 
	pub entry : OneNdpEntry, 
} 
impl OneAddDelNdpEntry { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_ndp_entry_d1629a2f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelNdpEntryReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelNdpEntryReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_ndp_entry_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneNdpEntriesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub bd : u32, 
} 
impl OneNdpEntriesGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_ndp_entries_get_4d418cf4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneNdpEntriesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub entries : OneNdpEntry, 
} 
impl OneNdpEntriesGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_ndp_entries_get_reply_0bd34161") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneSetTransportProtocol { 
	pub client_index : u32, 
	pub context : u32, 
	pub protocol : u8, 
} 
impl OneSetTransportProtocol { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_set_transport_protocol_07b6b85f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneSetTransportProtocolReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneSetTransportProtocolReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_set_transport_protocol_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneGetTransportProtocol { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneGetTransportProtocol { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_get_transport_protocol_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneGetTransportProtocolReply { 
	pub context : u32, 
	pub retval : i32, 
	pub protocol : u8, 
} 
impl OneGetTransportProtocolReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_get_transport_protocol_reply_62a28eb3") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneNdpBdGet { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneNdpBdGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_ndp_bd_get_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneNdpBdGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub bridge_domains : u32, 
} 
impl OneNdpBdGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_ndp_bd_get_reply_221ac888") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneL2ArpBdGet { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneL2ArpBdGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_l2_arp_bd_get_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneL2ArpBdGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub bridge_domains : u32, 
} 
impl OneL2ArpBdGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_l2_arp_bd_get_reply_221ac888") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelAdjacency { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : u8, 
	pub vni : u32, 
	pub reid : Eid, 
	pub leid : Eid, 
} 
impl OneAddDelAdjacency { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_adjacency_e48e7afe") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelAdjacencyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelAdjacencyReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_adjacency_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelMapRequestItrRlocs { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<U64>, 
} 
impl OneAddDelMapRequestItrRlocs { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_map_request_itr_rlocs_6be88e45") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAddDelMapRequestItrRlocsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneAddDelMapRequestItrRlocsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_add_del_map_request_itr_rlocs_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEidTableAddDelMap { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub vni : u32, 
	pub dp_table : u32, 
	pub is_l2 : bool, 
} 
impl OneEidTableAddDelMap { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_eid_table_add_del_map_9481416b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEidTableAddDelMapReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneEidTableAddDelMapReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_eid_table_add_del_map_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneLocatorDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub ls_index : u32, 
	pub ls_name : FixedSizeString<U64>, 
	pub is_index_set : bool, 
} 
impl OneLocatorDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_locator_dump_9b11076c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneLocatorDetails { 
	pub context : u32, 
	pub local : u8, 
	pub sw_if_index : InterfaceIndex, 
	pub ip_address : Address, 
	pub priority : u8, 
	pub weight : u8, 
} 
impl OneLocatorDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_locator_details_c0c4c2a7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneLocatorSetDetails { 
	pub context : u32, 
	pub ls_index : u32, 
	pub ls_name : FixedSizeString<U64>, 
} 
impl OneLocatorSetDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_locator_set_details_5b33a105") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneLocatorSetDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub filter : OneFilter, 
} 
impl OneLocatorSetDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_locator_set_dump_71190768") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEidTableDetails { 
	pub context : u32, 
	pub locator_set_index : u32, 
	pub action : u8, 
	pub is_local : bool, 
	pub is_src_dst : bool, 
	pub vni : u32, 
	pub deid : Eid, 
	pub seid : Eid, 
	pub ttl : u32, 
	pub authoritative : u8, 
	pub key : HmacKey, 
} 
impl OneEidTableDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_eid_table_details_4bc32e3a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEidTableDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub eid_set : bool, 
	pub vni : u32, 
	pub eid : Eid, 
	pub filter : OneFilter, 
} 
impl OneEidTableDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_eid_table_dump_95151038") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAdjacenciesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub adjacencies : OneAdjacency, 
} 
impl OneAdjacenciesGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_adjacencies_get_reply_a8ed89a5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneAdjacenciesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub vni : u32, 
} 
impl OneAdjacenciesGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_adjacencies_get_8d1f2fe9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEidTableMapDetails { 
	pub context : u32, 
	pub vni : u32, 
	pub dp_table : u32, 
} 
impl OneEidTableMapDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_eid_table_map_details_0b6859e2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEidTableMapDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_l2 : bool, 
} 
impl OneEidTableMapDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_eid_table_map_dump_d6cf0c3d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEidTableVniDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneEidTableVniDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_eid_table_vni_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEidTableVniDetails { 
	pub context : u32, 
	pub vni : u32, 
} 
impl OneEidTableVniDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_eid_table_vni_details_64abc01e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapResolverDetails { 
	pub context : u32, 
	pub ip_address : Address, 
} 
impl OneMapResolverDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_resolver_details_82a09deb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapResolverDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneMapResolverDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_resolver_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapServerDetails { 
	pub context : u32, 
	pub ip_address : Address, 
} 
impl OneMapServerDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_server_details_82a09deb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapServerDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneMapServerDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_server_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneStatus { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneStatus { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_status_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneStatusReply { 
	pub context : u32, 
	pub retval : i32, 
	pub feature_status : bool, 
	pub gpe_status : bool, 
} 
impl ShowOneStatusReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_status_reply_961bb25b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneGetMapRequestItrRlocs { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneGetMapRequestItrRlocs { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_get_map_request_itr_rlocs_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneGetMapRequestItrRlocsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub locator_set_name : FixedSizeString<U64>, 
} 
impl OneGetMapRequestItrRlocsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_get_map_request_itr_rlocs_reply_76580f3a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneNshMapping { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneNshMapping { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_nsh_mapping_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneNshMappingReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_set : bool, 
	pub locator_set_name : FixedSizeString<U64>, 
} 
impl ShowOneNshMappingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_nsh_mapping_reply_46478c02") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOnePitr { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOnePitr { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_pitr_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOnePitrReply { 
	pub context : u32, 
	pub retval : i32, 
	pub status : bool, 
	pub locator_set_name : FixedSizeString<U64>, 
} 
impl ShowOnePitrReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_pitr_reply_a2d1a49f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneStatsDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneStatsDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_stats_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneStatsDetails { 
	pub context : u32, 
	pub vni : u32, 
	pub deid : Eid, 
	pub seid : Eid, 
	pub rloc : Address, 
	pub lloc : Address, 
	pub pkt_count : u32, 
	pub bytes : u32, 
} 
impl OneStatsDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_stats_details_ff6ef238") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneStatsFlush { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneStatsFlush { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_stats_flush_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneStatsFlushReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneStatsFlushReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_stats_flush_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneStatsEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
impl OneStatsEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_stats_enable_disable_c264d7bf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneStatsEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneStatsEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_stats_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneStatsEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneStatsEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_stats_enable_disable_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneStatsEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
impl ShowOneStatsEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_stats_enable_disable_reply_f15abb16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapRegisterFallbackThreshold { 
	pub client_index : u32, 
	pub context : u32, 
	pub value : u32, 
} 
impl OneMapRegisterFallbackThreshold { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_register_fallback_threshold_f7d4a475") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneMapRegisterFallbackThresholdReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneMapRegisterFallbackThresholdReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_map_register_fallback_threshold_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneMapRegisterFallbackThreshold { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ShowOneMapRegisterFallbackThreshold { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_map_register_fallback_threshold_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ShowOneMapRegisterFallbackThresholdReply { 
	pub context : u32, 
	pub retval : i32, 
	pub value : u32, 
} 
impl ShowOneMapRegisterFallbackThresholdReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("show_one_map_register_fallback_threshold_reply_c93a9113") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEnableDisableXtrMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
impl OneEnableDisableXtrMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_enable_disable_xtr_mode_c264d7bf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEnableDisableXtrModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneEnableDisableXtrModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_enable_disable_xtr_mode_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneShowXtrMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneShowXtrMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_show_xtr_mode_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneShowXtrModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
impl OneShowXtrModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_show_xtr_mode_reply_f15abb16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEnableDisablePetrMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
impl OneEnableDisablePetrMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_enable_disable_petr_mode_c264d7bf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEnableDisablePetrModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneEnableDisablePetrModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_enable_disable_petr_mode_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneShowPetrMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneShowPetrMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_show_petr_mode_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneShowPetrModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
impl OneShowPetrModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_show_petr_mode_reply_f15abb16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEnableDisablePitrMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
impl OneEnableDisablePitrMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_enable_disable_pitr_mode_c264d7bf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneEnableDisablePitrModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OneEnableDisablePitrModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_enable_disable_pitr_mode_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneShowPitrMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl OneShowPitrMode { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_show_pitr_mode_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OneShowPitrModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
impl OneShowPitrModeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("one_show_pitr_mode_reply_f15abb16") 
	 } 
} 
