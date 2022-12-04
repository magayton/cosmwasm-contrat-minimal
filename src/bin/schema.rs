use cosmwasm_schema::write_api;
use contrat_tutoriel::msg::{TutorielInstantiateMsg, TutorielExecuteMsg, TutorielQueryMsg, TutorielMigrateMsg};

fn main() {
    write_api!{
        instantiate: TutorielInstantiateMsg,
        execute: TutorielExecuteMsg,
        query: TutorielQueryMsg,
        migrate: TutorielMigrateMsg,
    }
}