use cosmwasm_std::{Addr, Empty, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub admin: Addr, // Admin to allow revoking of names, could be a DAO, MS
    pub payment_token_address: Addr, // The address of the tokens used for payments
    pub payment_amount_to_register_name: Uint128, // Micro units we have to pay
}

pub const CONFIG: Item<Config> = Item::new("config");

/// Maps representing the one-to-one relationship of names, allows
/// two way look ups.
pub const DAO_TO_NAME: Map<Addr, String> = Map::new("dao_to_name");
pub const NAME_TO_DAO: Map<String, Addr> = Map::new("name_to_dao");

/// Maps for names which are reserved for later user, e.g. RAW, JUNO, or
/// anything of importance that could cause issues.
pub const RESERVED_NAMES: Map<String, Empty> = Map::new("reserved_names");
