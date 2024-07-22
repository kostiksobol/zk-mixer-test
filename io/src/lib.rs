#![no_std]

use gmeta::{InOut, Metadata};
use gstd::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<ContractInit, ()>;
    type Handle = InOut<ContractHandleAction, ContractHandleEvent>;
    type Reply = InOut<(), ()>;
    type State = InOut<StatePayload, StateOutput>;
    type Signal = ();
    type Others = ();
}

#[derive(Encode, Decode, TypeInfo)]
pub struct ContractInit {
    pub guest_id: [u32; 8],
    pub fixed_deposit_amount: u32,
}

#[derive(Clone, Encode, Decode, TypeInfo)]
pub enum ContractHandleAction {
    Deposit {
        hash: [u8; 32],
    },
    Withdraw {
        receipt: Vec<u8>,
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ContractHandleEvent {
    Deposited,
    Withdrawed,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StatePayload{
    Leaves,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateOutput{
    Leaves{res: Vec<[u8; 32]>},
}