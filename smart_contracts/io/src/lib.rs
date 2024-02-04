#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{debug, prelude::*, ActorId};

/*
    List the 2 more important functions in a lending platform

    DepositToPool - Send tokens from user to lending pool
    RedeemDeposit - Request the tokens from lending pool
*/

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum ActionLending {
    DepositToPool,
    RedeemDeposit,
}

/*
    List all the possible coins to be applied as collateral,
    USDC is the first proposal but this will change depending
    the coins supported by Vara Network
*/

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum LendingColateral {
    USDC,
}

/*
    Trigger an event depending the action executed

    DepositDone - The pool is having control over the user deposit
    RedeemDone - The user has the borrowed money + interest
*/
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum EventLending {
    DepositDone,
    RedeemDone,
}

/*
    Set the kind of interest rate every lending will be requested
*/

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum InterestKind {
    Fixed,
    Variable,
}

/*
    Set the different durations for a lending
*/

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum LendingDuration {
    Monthly,
    Yearly,
    Daily,
}

/*
    Metadata to understand the information the contract will share with other contracts or front-end
*/
pub struct LendingMetadata;

impl Metadata for LendingMetadata {
    type Init = ();
    type Handle = InOut<ActionLending, EventLending>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<LendingState>;
}

/*
    Every lending must have different attributes to describe the lending

    lender : The person that lends the tokens
    borrower : The persons that ask tokens and give collateral tokens
    kind : Describe the kind of interes the lending has
    lender_rate : the rate will gain the lender
    borrower_rate : the rate will pay the borrower
    platform_commision : The commision that the borrower will pay and receives the platform
    amount : The total amount of tokens to request
    colateral: The coin the actual borrower has and set as collateral
*/

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub struct LendingStateItem {
    borrower: ActorId,
    kind: InterestKind,
    lender_rate: i32,
    borrower_rate: i32,
    platform_comision: i8,
    amount: i32,
    collateral: LendingColateral,
    duration: LendingDuration,
}

/*
    STATE
    This is the most important structure in the contract because represents
    all the lendings per user
*/

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct LendingState {
    lendings: Vec<(ActorId, Vec<LendingStateItem>)>,
}

/*
    Implements the 2 more important methods for this initial lending platform
*/

impl LendingState {
    pub fn deposit_to_pool(&mut self, actor_id: ActorId, new_lending: LendingStateItem) {
        // Search if the actor already exist
        if let Some((_actor_id, lendings)) = self
            .lendings
            .iter_mut()
            .find(|(id, _items)| *id == actor_id)
        {
            // If exists the actor we only add a new LendingStateItem
            lendings.push(new_lending);
        } else {
            // If not,we add the pair (ActorId, Vec<LendingStateItem>)
            self.lendings.push((actor_id, vec![new_lending]));
        }
    }
    fn redeem_deposit(&self, amount: i32) {
        debug!("Redeemed {:?} to pool", amount);
    }
}
