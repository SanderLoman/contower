// //! Available RPC methods types and ids.

// use fixed_hash::construct_fixed_hash;
// use regex::bytes::Regex;
// use serde::Serialize;
// use ssz::Encode;
// use ssz_derive::{Decode, Encode};
// use ssz_types::{
//     typenum::{U1024, U256},
//     VariableList,
// };
// use std::marker::PhantomData;
// use std::ops::Deref;
// use std::sync::Arc;
// use strum::IntoStaticStr;
// use superstruct::superstruct;

// /// Maximum number of blocks in a single request.
// pub type MaxRequestBlocks = U1024;
// pub const MAX_REQUEST_BLOCKS: u64 = 1024;

// /// Maximum length of error message.
// pub type MaxErrorLen = U256;
// pub const MAX_ERROR_LEN: u64 = 256;

// /// Wrapper over SSZ List to represent error message in rpc responses.
// #[derive(Debug, Clone)]
// pub struct ErrorType(pub VariableList<u8, MaxErrorLen>);

// impl From<String> for ErrorType {
//     fn from(s: String) -> Self {
//         Self(VariableList::from(s.as_bytes().to_vec()))
//     }
// }

// impl From<&str> for ErrorType {
//     fn from(s: &str) -> Self {
//         Self(VariableList::from(s.as_bytes().to_vec()))
//     }
// }

// impl Deref for ErrorType {
//     type Target = VariableList<u8, MaxErrorLen>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl ToString for ErrorType {
//     fn to_string(&self) -> String {
//         #[allow(clippy::invalid_regex)]
//         let re = Regex::new("\\p{C}").expect("Regex is valid");
//         String::from_utf8_lossy(&re.replace_all(self.0.deref(), &b""[..])).to_string()
//     }
// }
// construct_fixed_hash! {
//     /// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
//     #[cfg_attr(feature = "scale-info", derive(TypeInfo))]
//     pub struct H256(32);
// }
// // Used for StatusMessage struct
// pub struct Epoch(u64);
// pub type Hash256 = H256;
// pub struct Slot(u64);
// /* Request/Response data structures for RPC methods */

// /* Requests */

// /// The STATUS request/response handshake message.
// #[derive(Encode, Decode, Clone, Debug, PartialEq)]
// pub struct StatusMessage {
//     /// The fork version of the chain we are broadcasting.
//     pub fork_digest: [u8; 4],

//     /// Latest finalized root.
//     pub finalized_root: Hash256,

//     /// Latest finalized epoch.
//     pub finalized_epoch: Epoch,

//     /// The latest block root.
//     pub head_root: Hash256,

//     /// The slot associated with the latest block root.
//     pub head_slot: Slot,
// }

// /// The PING request/response message.
// #[derive(Encode, Decode, Clone, Debug, PartialEq)]
// pub struct Ping {
//     /// The metadata sequence number.
//     pub data: u64,
// }

// /// The METADATA request structure.
// #[superstruct(
//     variants(V1, V2),
//     variant_attributes(derive(Clone, Debug, PartialEq, Serialize),)
// )]
// #[derive(Clone, Debug, PartialEq)]
// pub struct MetadataRequest {
//     _phantom_data: PhantomData,
// }

// impl MetadataRequest {
//     pub fn new_v1() -> Self {
//         Self::V1(MetadataRequestV1 {
//             _phantom_data: PhantomData,
//         })
//     }

//     pub fn new_v2() -> Self {
//         Self::V2(MetadataRequestV2 {
//             _phantom_data: PhantomData,
//         })
//     }
// }

// /// The reason given for a `Goodbye` message.
// ///
// /// Note: any unknown `u64::into(n)` will resolve to `Goodbye::Unknown` for any unknown `n`,
// /// however `GoodbyeReason::Unknown.into()` will go into `0_u64`. Therefore de-serializing then
// /// re-serializing may not return the same bytes.
// #[derive(Debug, Clone, PartialEq)]
// pub enum GoodbyeReason {
//     /// This node has shutdown.
//     ClientShutdown = 1,

//     /// Incompatible networks.
//     IrrelevantNetwork = 2,

//     /// Error/fault in the RPC.
//     Fault = 3,

//     /// Teku uses this code for not being able to verify a network.
//     UnableToVerifyNetwork = 128,

//     /// The node has too many connected peers.
//     TooManyPeers = 129,

//     /// Scored poorly.
//     BadScore = 250,

//     /// The peer is banned
//     Banned = 251,

//     /// The IP address the peer is using is banned.
//     BannedIP = 252,

//     /// Unknown reason.
//     Unknown = 0,
// }

// impl From<u64> for GoodbyeReason {
//     fn from(id: u64) -> GoodbyeReason {
//         match id {
//             1 => GoodbyeReason::ClientShutdown,
//             2 => GoodbyeReason::IrrelevantNetwork,
//             3 => GoodbyeReason::Fault,
//             128 => GoodbyeReason::UnableToVerifyNetwork,
//             129 => GoodbyeReason::TooManyPeers,
//             250 => GoodbyeReason::BadScore,
//             251 => GoodbyeReason::Banned,
//             252 => GoodbyeReason::BannedIP,
//             _ => GoodbyeReason::Unknown,
//         }
//     }
// }

// impl From<GoodbyeReason> for u64 {
//     fn from(reason: GoodbyeReason) -> u64 {
//         reason as u64
//     }
// }

// impl ssz::Encode for GoodbyeReason {
//     fn is_ssz_fixed_len() -> bool {
//         <u64 as ssz::Encode>::is_ssz_fixed_len()
//     }

//     fn ssz_fixed_len() -> usize {
//         <u64 as ssz::Encode>::ssz_fixed_len()
//     }

//     fn ssz_bytes_len(&self) -> usize {
//         0_u64.ssz_bytes_len()
//     }

//     fn ssz_append(&self, buf: &mut Vec<u8>) {
//         let conv: u64 = self.clone().into();
//         conv.ssz_append(buf)
//     }
// }

// impl ssz::Decode for GoodbyeReason {
//     fn is_ssz_fixed_len() -> bool {
//         <u64 as ssz::Decode>::is_ssz_fixed_len()
//     }

//     fn ssz_fixed_len() -> usize {
//         <u64 as ssz::Decode>::ssz_fixed_len()
//     }

//     fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
//         u64::from_ssz_bytes(bytes).map(|n| n.into())
//     }
// }

// /// Request a number of beacon block roots from a peer.
// #[superstruct(
//     variants(V1, V2),
//     variant_attributes(derive(Encode, Decode, Clone, Debug, PartialEq))
// )]
// #[derive(Clone, Debug, PartialEq)]
// pub struct BlocksByRangeRequest {
//     /// The starting slot to request blocks.
//     pub start_slot: u64,

//     /// The number of blocks from the start slot.
//     pub count: u64,
// }

// impl BlocksByRangeRequest {
//     /// The default request is V2
//     pub fn new(start_slot: u64, count: u64) -> Self {
//         Self::V2(BlocksByRangeRequestV2 { start_slot, count })
//     }

//     pub fn new_v1(start_slot: u64, count: u64) -> Self {
//         Self::V1(BlocksByRangeRequestV1 { start_slot, count })
//     }
// }

// /// Request a number of beacon block roots from a peer.
// #[superstruct(
//     variants(V1, V2),
//     variant_attributes(derive(Encode, Decode, Clone, Debug, PartialEq))
// )]
// #[derive(Clone, Debug, PartialEq)]
// pub struct OldBlocksByRangeRequest {
//     /// The starting slot to request blocks.
//     pub start_slot: u64,

//     /// The number of blocks from the start slot.
//     pub count: u64,

//     /// The step increment to receive blocks.
//     ///
//     /// A value of 1 returns every block.
//     /// A value of 2 returns every second block.
//     /// A value of 3 returns every third block and so on.
//     pub step: u64,
// }

// impl OldBlocksByRangeRequest {
//     /// The default request is V2
//     pub fn new(start_slot: u64, count: u64, step: u64) -> Self {
//         Self::V2(OldBlocksByRangeRequestV2 {
//             start_slot,
//             count,
//             step,
//         })
//     }

//     pub fn new_v1(start_slot: u64, count: u64, step: u64) -> Self {
//         Self::V1(OldBlocksByRangeRequestV1 {
//             start_slot,
//             count,
//             step,
//         })
//     }
// }

// /// Request a number of beacon block bodies from a peer.
// #[superstruct(variants(V1, V2), variant_attributes(derive(Clone, Debug, PartialEq)))]
// #[derive(Clone, Debug, PartialEq)]
// pub struct BlocksByRootRequest {
//     /// The list of beacon block bodies being requested.
//     pub block_roots: VariableList<Hash256, MaxRequestBlocks>,
// }

// impl BlocksByRootRequest {
//     pub fn new(block_roots: VariableList<Hash256, MaxRequestBlocks>) -> Self {
//         Self::V2(BlocksByRootRequestV2 { block_roots })
//     }

//     pub fn new_v1(block_roots: VariableList<Hash256, MaxRequestBlocks>) -> Self {
//         Self::V1(BlocksByRootRequestV1 { block_roots })
//     }
// }

// /* RPC Handling and Grouping */
// // Collection of enums and structs used by the Codecs to encode/decode RPC messages

// #[derive(Debug, Clone, PartialEq)]
// pub enum RPCResponse {
//     /// A HELLO message.
//     Status(StatusMessage),

//     /// A response to a get BLOCKS_BY_RANGE request. A None response signifies the end of the
//     /// batch.
//     BlocksByRange(Arc<_>),

//     /// A response to a get BLOCKS_BY_ROOT request.
//     BlocksByRoot(Arc<_>),

//     /// A response to a get LIGHTCLIENT_BOOTSTRAP request.
//     LightClientBootstrap(_),

//     /// A PONG response to a PING request.
//     Pong(Ping),

//     /// A response to a META_DATA request.
//     MetaData(_),
// }

// /// Indicates which response is being terminated by a stream termination response.
// #[derive(Debug, Clone)]
// pub enum ResponseTermination {
//     /// Blocks by range stream termination.
//     BlocksByRange,

//     /// Blocks by root stream termination.
//     BlocksByRoot,
// }

// /// The structured response containing a result/code indicating success or failure
// /// and the contents of the response
// #[derive(Debug, Clone)]
// pub enum RPCCodedResponse {
//     /// The response is a successful.
//     Success(RPCResponse),

//     Error(RPCResponseErrorCode, ErrorType),

//     /// Received a stream termination indicating which response is being terminated.
//     StreamTermination(ResponseTermination),
// }

// /// Request a light_client_bootstrap for lightclients peers.
// #[derive(Encode, Decode, Clone, Debug, PartialEq)]
// pub struct LightClientBootstrapRequest {
//     pub root: Hash256,
// }

// /// The code assigned to an erroneous `RPCResponse`.
// #[derive(Debug, Clone, Copy, PartialEq, IntoStaticStr)]
// #[strum(serialize_all = "snake_case")]
// pub enum RPCResponseErrorCode {
//     RateLimited,
//     InvalidRequest,
//     ServerError,
//     /// Error spec'd to indicate that a peer does not have blocks on a requested range.
//     ResourceUnavailable,
//     Unknown,
// }

// impl RPCCodedResponse {
//     /// Used to encode the response in the codec.
//     pub fn as_u8(&self) -> Option<u8> {
//         match self {
//             RPCCodedResponse::Success(_) => Some(0),
//             RPCCodedResponse::Error(code, _) => Some(code.as_u8()),
//             RPCCodedResponse::StreamTermination(_) => None,
//         }
//     }

//     /// Tells the codec whether to decode as an RPCResponse or an error.
//     pub fn is_response(response_code: u8) -> bool {
//         matches!(response_code, 0)
//     }

//     /// Builds an RPCCodedResponse from a response code and an ErrorMessage
//     pub fn from_error(response_code: u8, err: ErrorType) -> Self {
//         let code = match response_code {
//             1 => RPCResponseErrorCode::InvalidRequest,
//             2 => RPCResponseErrorCode::ServerError,
//             3 => RPCResponseErrorCode::ResourceUnavailable,
//             139 => RPCResponseErrorCode::RateLimited,
//             _ => RPCResponseErrorCode::Unknown,
//         };
//         RPCCodedResponse::Error(code, err)
//     }

//     /// Specifies which response allows for multiple chunks for the stream handler.
//     pub fn multiple_responses(&self) -> bool {
//         match self {
//             RPCCodedResponse::Success(resp) => match resp {
//                 RPCResponse::Status(_) => false,
//                 RPCResponse::BlocksByRange(_) => true,
//                 RPCResponse::BlocksByRoot(_) => true,
//                 RPCResponse::Pong(_) => false,
//                 RPCResponse::MetaData(_) => false,
//                 RPCResponse::LightClientBootstrap(_) => false,
//             },
//             RPCCodedResponse::Error(_, _) => true,
//             // Stream terminations are part of responses that have chunks
//             RPCCodedResponse::StreamTermination(_) => true,
//         }
//     }

//     /// Returns true if this response always terminates the stream.
//     pub fn close_after(&self) -> bool {
//         !matches!(self, RPCCodedResponse::Success(_))
//     }
// }

// impl RPCResponseErrorCode {
//     fn as_u8(&self) -> u8 {
//         match self {
//             RPCResponseErrorCode::InvalidRequest => 1,
//             RPCResponseErrorCode::ServerError => 2,
//             RPCResponseErrorCode::ResourceUnavailable => 3,
//             RPCResponseErrorCode::Unknown => 255,
//             RPCResponseErrorCode::RateLimited => 139,
//         }
//     }
// }

// use crate::rpc::protocol::Protocol;
// impl RPCResponse {
//     pub fn protocol(&self) -> Protocol {
//         match self {
//             RPCResponse::Status(_) => Protocol::Status,
//             RPCResponse::BlocksByRange(_) => Protocol::BlocksByRange,
//             RPCResponse::BlocksByRoot(_) => Protocol::BlocksByRoot,
//             RPCResponse::Pong(_) => Protocol::Ping,
//             RPCResponse::MetaData(_) => Protocol::MetaData,
//             RPCResponse::LightClientBootstrap(_) => Protocol::LightClientBootstrap,
//         }
//     }
// }

// impl std::fmt::Display for RPCResponseErrorCode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let repr = match self {
//             RPCResponseErrorCode::InvalidRequest => "The request was invalid",
//             RPCResponseErrorCode::ResourceUnavailable => "Resource unavailable",
//             RPCResponseErrorCode::ServerError => "Server error occurred",
//             RPCResponseErrorCode::Unknown => "Unknown error occurred",
//             RPCResponseErrorCode::RateLimited => "Rate limited",
//         };
//         f.write_str(repr)
//     }
// }

// impl std::fmt::Display for StatusMessage {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Status Message: Fork Digest: {:?}, Finalized Root: {}, Finalized Epoch: {}, Head Root: {}, Head Slot: {}", self.fork_digest, self.finalized_root, self.finalized_epoch, self.head_root, self.head_slot)
//     }
// }

// impl std::fmt::Display for RPCResponse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             RPCResponse::Status(status) => write!(f, "{}", status),
//             RPCResponse::BlocksByRange(block) => {
//                 write!(f, "BlocksByRange: Block slot: {}", block.slot())
//             }
//             RPCResponse::BlocksByRoot(block) => {
//                 write!(f, "BlocksByRoot: Block slot: {}", block.slot())
//             }
//             RPCResponse::Pong(ping) => write!(f, "Pong: {}", ping.data),
//             RPCResponse::MetaData(metadata) => write!(f, "Metadata: {}", metadata.seq_number()),
//             RPCResponse::LightClientBootstrap(bootstrap) => {
//                 write!(f, "LightClientBootstrap Slot: {}", bootstrap.header.slot)
//             }
//         }
//     }
// }

// impl std::fmt::Display for RPCCodedResponse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             RPCCodedResponse::Success(res) => write!(f, "{}", res),
//             RPCCodedResponse::Error(code, err) => write!(f, "{}: {}", code, err.to_string()),
//             RPCCodedResponse::StreamTermination(_) => write!(f, "Stream Termination"),
//         }
//     }
// }

// impl std::fmt::Display for GoodbyeReason {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             GoodbyeReason::ClientShutdown => write!(f, "Client Shutdown"),
//             GoodbyeReason::IrrelevantNetwork => write!(f, "Irrelevant Network"),
//             GoodbyeReason::Fault => write!(f, "Fault"),
//             GoodbyeReason::UnableToVerifyNetwork => write!(f, "Unable to verify network"),
//             GoodbyeReason::TooManyPeers => write!(f, "Too many peers"),
//             GoodbyeReason::BadScore => write!(f, "Bad Score"),
//             GoodbyeReason::Banned => write!(f, "Banned"),
//             GoodbyeReason::BannedIP => write!(f, "BannedIP"),
//             GoodbyeReason::Unknown => write!(f, "Unknown Reason"),
//         }
//     }
// }

// impl std::fmt::Display for BlocksByRangeRequest {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Start Slot: {}, Count: {}",
//             self.start_slot(),
//             self.count()
//         )
//     }
// }

// impl std::fmt::Display for OldBlocksByRangeRequest {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Start Slot: {}, Count: {}, Step: {}",
//             self.start_slot(),
//             self.count(),
//             self.step()
//         )
//     }
// }

// impl slog::KV for StatusMessage {
//     fn serialize(
//         &self,
//         record: &slog::Record,
//         serializer: &mut dyn slog::Serializer,
//     ) -> slog::Result {
//         use slog::Value;
//         serializer.emit_arguments("fork_digest", &format_args!("{:?}", self.fork_digest))?;
//         Value::serialize(&self.finalized_epoch, record, "finalized_epoch", serializer)?;
//         serializer.emit_arguments("finalized_root", &format_args!("{}", self.finalized_root))?;
//         Value::serialize(&self.head_slot, record, "head_slot", serializer)?;
//         serializer.emit_arguments("head_root", &format_args!("{}", self.head_root))?;
//         slog::Result::Ok(())
//     }
// }