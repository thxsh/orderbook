use crate::{
    contract::{Orderbook, OrderbookResult},
    msg::OrderbookInstantiateMsg,
    state::{Config, CONFIG},
};

use abstract_app::objects::ans_host::AnsHost;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _module: Orderbook,
    msg: OrderbookInstantiateMsg,
) -> OrderbookResult {
    let config: Config = Config {};
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}
