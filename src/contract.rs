#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use ibc_core::client::types::Height;
use ibc_core::primitives::proto::Any;
use ibc_proto::ibc::lightclients::wasm::v1::ClientState;
use prost::Message;

use crate::error::ContractError;
use crate::msg::{
    CheckForMisbehaviourResult, ExecuteMsg, InstantiateMsg, QueryMsg, StatusResult, SudoMsg,
    TimestampAtHeightResult, UpdateStateResult,
};
use crate::state::{consensus_db_key, HOST_CLIENT_STATE_KEY};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let client_state = ClientState {
        checksum: msg.checksum.into(),
        data: msg.client_state.clone().into(),
        latest_height: Some(ibc_proto::ibc::core::client::v1::Height {
            revision_number: 0,
            revision_height: 1,
        }),
    };

    let client_state_any = Any {
        type_url: "/ibc.lightclients.wasm.v1.ClientState".to_string(),
        value: client_state.encode_to_vec(),
    };

    deps.storage.set(
        HOST_CLIENT_STATE_KEY.as_bytes(),
        client_state_any.encode_to_vec().as_slice(),
    );

    let height = Height::new(0, 1).unwrap();
    deps.storage.set(
        consensus_db_key(&height).as_bytes(),
        msg.consensus_state.as_slice(),
    );

    Ok(Response::default())
}

#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    let result = match msg {
        SudoMsg::VerifyMembership(_) => verify_membership()?,
        SudoMsg::VerifyNonMembership(_) => verify_non_membership()?,
        SudoMsg::UpdateState(_) => update_state()?,
        SudoMsg::UpdateStateOnMisbehaviour(_) => unimplemented!(),
        SudoMsg::VerifyUpgradeAndUpdateState(_) => todo!(),
        SudoMsg::MigrateClientStore(_) => unimplemented!(),
    };

    Ok(Response::default().set_data(result))
}

pub fn verify_membership() -> Result<Binary, ContractError> {
    Ok(to_json_binary(&Ok::<(), ()>(()))?)
}

pub fn verify_non_membership() -> Result<Binary, ContractError> {
    Ok(to_json_binary(&Ok::<(), ()>(()))?)
}

pub fn update_state() -> Result<Binary, ContractError> {
    Ok(to_json_binary(&UpdateStateResult { heights: vec![] })?)
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[entry_point]
pub fn query(_deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::VerifyClientMessage(_) => verify_client_message(),
        QueryMsg::CheckForMisbehaviour(_) => check_for_misbehaviour(),
        QueryMsg::TimestampAtHeight(_) => timestamp_at_height(env),
        QueryMsg::Status(_) => status(),
        QueryMsg::ExportMetadata(_) => export_metadata(),
    }
}

pub fn verify_client_message() -> Result<Binary, ContractError> {
    Ok(to_json_binary(&Ok::<(), ()>(()))?)
}

pub fn check_for_misbehaviour() -> Result<Binary, ContractError> {
    Ok(to_json_binary(&CheckForMisbehaviourResult {
        found_misbehaviour: false,
    })?)
}

pub fn timestamp_at_height(env: Env) -> Result<Binary, ContractError> {
    let now = env.block.time.nanos();
    Ok(to_json_binary(&TimestampAtHeightResult { timestamp: now })?)
}

pub fn status() -> Result<Binary, ContractError> {
    Ok(to_json_binary(&StatusResult {
        status: "Active".to_string(),
    })?)
}

pub fn export_metadata() -> Result<Binary, ContractError> {
    Ok(to_json_binary(&Ok::<(), ()>(()))?)
}

#[cfg(test)]
mod tests {}
