mod instantiate;
mod execute;
mod query;

use cosmwasm_std::{
    DepsMut, Env, MessageInfo, StdResult, Response, entry_point, Binary, Deps
};
use cw2::set_contract_version;

use crate::msg::{TutorielInstantiateMsg, TutorielExecuteMsg, TutorielQueryMsg, TutorielMigrateMsg};
use crate::error::TutorielError;

use crate::contract::instantiate::_instantiate;
use crate::contract::execute::_execute;
use crate::contract::query::_query;

const CONTRACT_NAME: &str = "crates.io:contrat_tutoriel"; // contrat_tutoriel : Vient du Cargo.toml
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: TutorielInstantiateMsg) -> Result<Response, TutorielError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    _instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: TutorielExecuteMsg) -> Result<Response, TutorielError> {
    _execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: TutorielQueryMsg) -> StdResult<Binary> {
    _query(deps, env, msg)
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: TutorielMigrateMsg) -> Result<Response, TutorielError> {
    Ok(Response::default())
}