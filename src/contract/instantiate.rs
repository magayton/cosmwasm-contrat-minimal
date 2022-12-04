use crate::error::TutorielError;
use crate::msg::TutorielInstantiateMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn _instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: TutorielInstantiateMsg,
) -> Result<Response, TutorielError> {
    Ok(Response::new().add_attribute("Instantiate", "Instantiate OK"))
}