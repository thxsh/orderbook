use crate::{
    msg::{Header, OrderbookEvent, OrderbookIbcMessage},
    Orderbook, OrderbookError, ORDERBOOK_ID,
};
use abstract_adapter::sdk::AbstractResponse;
use abstract_adapter::std::ibc::ModuleIbcMsg;
use cosmwasm_std::{from_json, Binary, DepsMut, Env};

use crate::{contract::OrderbookResult, handlers::execute::route_msg};

pub fn ibc_handler(
    deps: DepsMut,
    _env: Env,
    mut app: Orderbook,
    module_info: ModuleIbcInfo,
    ibc_msg: Binary,
) -> OrderbookResult {
    // Assert IBC sender was the server
    if module_info.module.id().ne(IBCMAIL_SERVER_ID) {
        return Err(ServerError::UnauthorizedIbcModule(module_info.clone()));
    };

    let server_msg: OrderbookIbcMessage = from_json(msg)?;

    match server_msg {
        ServerIbcMessage::RouteMessage {
            msg,
            header,
            metadata,
        } => {
            let msgs = route_message(
                deps,
                &env,
                &mut app,
                &TruncatedChainId::new(&env),
                header,
                metadata,
                msg,
            )?;

            Ok(app
                .response("module_ibc")
                .add_attribute("method", "route")
                .add_submessages(msgs))
        }
        _ => Err(ServerError::UnauthorizedIbcMessage {}),
    }
}
