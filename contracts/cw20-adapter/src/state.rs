use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const LAUNCHPAD: Item<Addr> = Item::new("avida-launchpad");
