#![no_std]
use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum ActionLending {
    Lend,
    Borrow,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum EventLending {
    LendDone,
    BorrowDone,
}

pub struct LendingMetadata;

impl Metadata for LendingMetadata {
    type Init = ();
    type Handle = InOut<ActionLending, EventLending>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Vec<(ActorId, String)>>;
}
