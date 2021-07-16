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
pub struct Srv6SidList { 
	pub num_sids : u8, 
	pub weight : u32, 
	pub sids : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Srv6SidListWithSlIndex { 
	pub num_sids : u8, 
	pub weight : u32, 
	pub sl_index : u32, 
	pub sids : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
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
pub enum SrPolicyOp { 
	 SR_POLICY_OP_API_NONE=0, 
	 SR_POLICY_OP_API_ADD=1, 
	 SR_POLICY_OP_API_DEL=2, 
	 SR_POLICY_OP_API_MOD=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum SrBehavior { 
	 SR_BEHAVIOR_API_END=1, 
	 SR_BEHAVIOR_API_X=2, 
	 SR_BEHAVIOR_API_T=3, 
	 SR_BEHAVIOR_API_D_FIRST=4, 
	 SR_BEHAVIOR_API_DX2=5, 
	 SR_BEHAVIOR_API_DX6=6, 
	 SR_BEHAVIOR_API_DX4=7, 
	 SR_BEHAVIOR_API_DT6=8, 
	 SR_BEHAVIOR_API_DT4=9, 
	 SR_BEHAVIOR_API_LAST=10, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum SrSteer { 
	 SR_STEER_API_L2=2, 
	 SR_STEER_API_IPV4=4, 
	 SR_STEER_API_IPV6=6, 
} 
pub type InterfaceIndex=u32; 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrLocalsidAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_del : bool, 
	pub localsid : Ip6Address, 
	pub end_psp : bool, 
	pub behavior : SrBehavior, 
	pub sw_if_index : InterfaceIndex, 
	pub vlan_index : u32, 
	pub fib_table : u32, 
	pub nh_addr : Address, 
} 
impl SrLocalsidAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_localsid_add_del_26fa3309") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrLocalsidAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SrLocalsidAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_localsid_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPolicyAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub bsid_addr : Ip6Address, 
	pub weight : u32, 
	pub is_encap : bool, 
	pub is_spray : bool, 
	pub fib_table : u32, 
	pub sids : Srv6SidList, 
} 
impl SrPolicyAdd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policy_add_ec79ee6a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPolicyAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SrPolicyAddReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policy_add_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPolicyMod { 
	pub client_index : u32, 
	pub context : u32, 
	pub bsid_addr : Ip6Address, 
	pub sr_policy_index : u32, 
	pub fib_table : u32, 
	pub operation : SrPolicyOp, 
	pub sl_index : u32, 
	pub weight : u32, 
	pub sids : Srv6SidList, 
} 
impl SrPolicyMod { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policy_mod_e531a102") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPolicyModReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SrPolicyModReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policy_mod_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPolicyDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub bsid_addr : Ip6Address, 
	pub sr_policy_index : u32, 
} 
impl SrPolicyDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policy_del_cb4d48d5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPolicyDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SrPolicyDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policy_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrSetEncapSource { 
	pub client_index : u32, 
	pub context : u32, 
	pub encaps_source : Ip6Address, 
} 
impl SrSetEncapSource { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_set_encap_source_d3bad5e1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrSetEncapSourceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SrSetEncapSourceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_set_encap_source_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrSetEncapHopLimit { 
	pub client_index : u32, 
	pub context : u32, 
	pub hop_limit : u8, 
} 
impl SrSetEncapHopLimit { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_set_encap_hop_limit_aa75d7d0") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrSetEncapHopLimitReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SrSetEncapHopLimitReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_set_encap_hop_limit_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrSteeringAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_del : bool, 
	pub bsid_addr : Ip6Address, 
	pub sr_policy_index : u32, 
	pub table_id : u32, 
	pub prefix : Prefix, 
	pub sw_if_index : InterfaceIndex, 
	pub traffic_type : SrSteer, 
} 
impl SrSteeringAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_steering_add_del_3711dace") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrSteeringAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SrSteeringAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_steering_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrLocalsidsDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl SrLocalsidsDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_localsids_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrLocalsidsDetails { 
	pub context : u32, 
	pub addr : Ip6Address, 
	pub end_psp : bool, 
	pub behavior : SrBehavior, 
	pub fib_table : u32, 
	pub vlan_index : u32, 
	pub xconnect_nh_addr : Address, 
	pub xconnect_iface_or_vrf_table : u32, 
} 
impl SrLocalsidsDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_localsids_details_6a6c0265") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPoliciesDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl SrPoliciesDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policies_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPoliciesDetails { 
	pub context : u32, 
	pub bsid : Ip6Address, 
	pub is_spray : bool, 
	pub is_encap : bool, 
	pub fib_table : u32, 
	pub num_sid_lists : u8, 
	pub sid_lists : Srv6SidList, 
} 
impl SrPoliciesDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policies_details_07ec2d93") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPoliciesWithSlIndexDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl SrPoliciesWithSlIndexDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policies_with_sl_index_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrPoliciesWithSlIndexDetails { 
	pub context : u32, 
	pub bsid : Ip6Address, 
	pub is_spray : bool, 
	pub is_encap : bool, 
	pub fib_table : u32, 
	pub num_sid_lists : u8, 
	pub sid_lists : Srv6SidListWithSlIndex, 
} 
impl SrPoliciesWithSlIndexDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_policies_with_sl_index_details_ca2e9bc8") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrSteeringPolDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl SrSteeringPolDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_steering_pol_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SrSteeringPolDetails { 
	pub context : u32, 
	pub traffic_type : SrSteer, 
	pub fib_table : u32, 
	pub prefix : Prefix, 
	pub sw_if_index : InterfaceIndex, 
	pub bsid : Ip6Address, 
} 
impl SrSteeringPolDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sr_steering_pol_details_1c1ee786") 
	 } 
} 
