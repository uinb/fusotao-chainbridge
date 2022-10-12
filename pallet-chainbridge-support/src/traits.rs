use crate::ResourceId;
use codec::{Decode, EncodeLike};
use frame_support::dispatch::DispatchError;
use frame_support::dispatch::DispatchResult;
use sp_runtime::traits::Dispatchable;
use sp_std::prelude::*;

pub trait Agent<AccountId> {
    type Origin;
    type Message: EncodeLike + Decode + Dispatchable;

    /// bind the origin to an appchain account without private key
    /// function RegisterInterchainAccount(counterpartyPortId: Identifier, connectionID: Identifier) returns (nil)
    fn register_agent(origin: Self::Origin) -> Result<AccountId, DispatchError>;

    /// function AuthenticateTx(msgs []Any, connectionId string, portId string) returns (error)
    fn authenticate_tx(origin: Self::Origin, msg: Self::Message) -> Result<(), DispatchError>;

    /// function ExecuteTx(sourcePort: Identifier, channel Channel, msgs []Any) returns (resultString, error)
    fn execute_tx(origin: Self::Origin, msg: Self::Message) -> DispatchResult;
}

/// A trait handling asset ID and name
pub trait AssetIdResourceIdProvider<AssetId> {
    type Err;

    fn try_get_asset_id(resource_id: ResourceId) -> Result<AssetId, Self::Err>;
}
