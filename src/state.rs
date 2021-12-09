use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Valut {
    pub deposited: Uint128,
    pub issued: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Supply {
    pub deposited: Uint128,
    pub issued: Uint128,
}

// mock 1 gLSRV => ust price
pub const ORACEL_PRICE: Item<Uint128> = Item::new("oracel_price");

// user valuts
pub const VALUTS: Map<&Addr, Valut> = Map::new("valuts");
// sysytm status
pub const TOTAL_SUPPLY: Item<Supply> = Item::new("total_supply");

