use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct TutorielInstantiateMsg {}

#[cw_serde]
pub enum TutorielExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum TutorielQueryMsg {}

#[cw_serde]
pub struct TutorielMigrateMsg {}