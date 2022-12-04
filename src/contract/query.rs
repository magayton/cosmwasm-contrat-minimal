use crate::msg::TutorielQueryMsg;
use cosmwasm_std::{Deps, Env, StdResult, Binary};

pub fn _query(
    _deps: Deps,
    _env: Env,
    _msg: TutorielQueryMsg,
) -> StdResult<Binary> {
    Ok(Binary::default())
}