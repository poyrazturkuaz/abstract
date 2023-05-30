use crate::contract::{DexAdapter, DexResult};
use abstract_core::objects::fee::UsageFee;
use crate::msg::DexInstantiateMsg;
use crate::state::SWAP_FEE;
use abstract_sdk::AccountVerification;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    adapter: DexAdapter,
    msg: DexInstantiateMsg,
) -> DexResult {
    let recipient = adapter
        .account_registry(deps.as_ref())
        .proxy_address(msg.recipient_account)?;
    let fee = UsageFee::new(deps.api, msg.swap_fee, recipient)?;
    SWAP_FEE.save(deps.storage, &fee)?;
    Ok(Response::default())
}
