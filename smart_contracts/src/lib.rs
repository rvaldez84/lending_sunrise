#![no_std]
use fungible_token_io::{FTAction, FTEvent, IoFungibleToken};
use gstd::{collections::HashMap, errors::Result, msg, prelude::*, ActorId};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

static mut STATE: Option<HashMap<ActorId, String>> = None;

fn state_mut() -> &'static mut HashMap<ActorId, String> {
    let state = unsafe { STATE.as_mut() };
    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init() {
    unsafe { STATE = Some(HashMap::new()) }
}

#[no_mangle]
extern "C" fn handle() {
    handle_state().expect("Execution Error")
}

fn handle_state() -> Result<()> {
    let payload = msg::load()?;
    if let ActionLending::DepositToPool = payload {
        msg::reply(EventLending::DepositDone, 0)?;
    }

    if let ActionLending::RedeemDeposit = payload {
        msg::reply(EventLending::RedeemDone, 0)?;
    }

    Ok(())
}

#[no_mangle]
extern "C" fn state() {
    let state: Vec<_> = state_mut().iter().map(|(k, v)| (*k, v.clone())).collect();
    msg::reply(state, 0).expect("fail to send the state");
}
