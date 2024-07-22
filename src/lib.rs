#![no_std]

use gstd::{msg, Vec};
use io::{ContractHandleAction, ContractHandleEvent, ContractInit, StateOutput, StatePayload};
use risc0_zkvm::{guest::sha::Impl, sha::Sha256, Receipt};

use rs_merkle::MerkleTree;

#[derive(Clone)]
pub struct DigestWrapper(Impl);

impl rs_merkle::Hasher for DigestWrapper{
    type Hash = [u8; 32];
    fn hash(data: &[u8]) -> Self::Hash {
        (*Impl::hash_bytes(data)).into()
    }
}

#[derive(Default)]
pub struct Mixer {
    pub fixed_deposit_amount: u32,
    pub guest_id: [u32; 8],
    pub merkle_tree: MerkleTree<DigestWrapper>,
    pub withdrawn: Vec<[u8; 32]>,
}

impl Mixer {
    fn deposit(&mut self, hash: [u8; 32]){
        self.merkle_tree.insert(hash).commit();

        msg::reply(ContractHandleEvent::Deposited, 0).expect("Error in reply in deposit");
    }   

    fn withdraw(&mut self, receipt: Vec<u8>){
        let (receipt, _): (Receipt, _) = serde_json_core::from_slice(&receipt).expect("msg");
        let output_root: [u8; 32] = receipt.journal.decode().expect("a");

        assert!(self.merkle_tree.history.contains(&output_root), "There has never been such a root");
        receipt.verify(self.guest_id).expect("error");

        msg::reply(ContractHandleEvent::Withdrawed, self.fixed_deposit_amount as u128).expect("Error in reply in withdraw");
    }
}

static mut MIXER: Option<Mixer> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let init_config: ContractInit = msg::load().expect("#Error in decoding ContractInit#");
    MIXER = Some(Mixer {
        fixed_deposit_amount: init_config.fixed_deposit_amount,
        guest_id: init_config.guest_id,
        ..Default::default()
    });
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: ContractHandleAction =
        msg::load().expect("Unable to decode ContractHandleAction");
    let mixer = MIXER.get_or_insert(Default::default());

    match action {
        ContractHandleAction::Deposit { hash } => mixer.deposit(hash),
        ContractHandleAction::Withdraw { receipt } => mixer.withdraw(receipt),
    };
}

#[no_mangle]
extern "C" fn state() {
    let payload: StatePayload = msg::load().expect("Error in decoding payload in state function");
    let mixer: &mut Mixer = unsafe { MIXER.get_or_insert(Default::default()) };
    match payload {
        StatePayload::Leaves => {
            let res = mixer.merkle_tree.leaves().unwrap();
            msg::reply(StateOutput::Leaves { res }, 0).expect("Failed to share state");
        },
    }
}