//! Registry of type URLs associated with various protobuf types defined in
//! this crate.

// TODO(tarcieri): leverage first-class support for type URLs in prost?
// See: https://github.com/tokio-rs/prost/issues/299

use crate::{osmosis, cosmos_sdk_proto::traits::TypeUrl};

 
impl TypeUrl for osmosis::gamm::v1beta1::Pool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Pool";
}
 
impl TypeUrl for osmosis::poolincentives::v1beta1::UpdatePoolIncentivesProposal {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.UpdatePoolIncentivesProposal";
}
