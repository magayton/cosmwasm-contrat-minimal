use crate::error::TutorielError;
use crate::msg::TutorielExecuteMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn _execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: TutorielExecuteMsg,
) -> Result<Response, TutorielError> {
    Ok(Response::new().add_attribute("Execute", "Execute OK"))
}