use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};
use cw20::Cw20ReceiveMsg;
use cw_asset::Asset;

use crate::contract::{Orderbook, OrderbookResult};

pub fn receive_handler(
    deps: DepsMut,
    _env: Env,
    mut info: MessageInfo,
    module: Orderbook,
    msg: Cw20ReceiveMsg,
) -> OrderbookResult {
    let Cw20ReceiveMsg {
        sender,
        amount,
        msg: _,
    } = msg;

    let receipt = Asset::cw20(info.sender, amount);

    println!(
        "receive_handler: sender: {:?}, amount: {:?}, receipt: {:?}",
        sender, amount, receipt
    );

    info.sender = deps.api.addr_validate(&sender)?;
    // crate::handlers::execute::limit_order(deps, env, info, module, Some(receipt))

    Ok(module.response("receive"))
}
