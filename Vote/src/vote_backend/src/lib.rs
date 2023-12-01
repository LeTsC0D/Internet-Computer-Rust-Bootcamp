use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(CandidType, Deserialize)]
struct Proposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voted: Vec<candid::Principal>,
    owner: candid::Principal,
}

impl Storable for Proposal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

#[derive(CandidType, Deserialize)]
struct CreateProposal {
    description: String,
    is_active: bool,
}

#[derive(CandidType, Deserialize)]
enum VoteTypes {
    Approve,
    Reject,
    Pass,
}

#[derive(CandidType, Deserialize)]
enum VoteError {
    AlreadyVoted,
    ProposalNotActive,
    Unauthorized,
    NoProposal,
    UpdateError,
    VoteFailed,
}

const MAX_VALUE_SIZE: u32 = 100;
impl BoundedStorable for Proposal {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static PROPOSAL_MAP: RefCell<StableBTreeMap<u64, Proposal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}


#[ic_cdk_macros::query]
fn get_proposal(key: u64) -> Option<Proposal> {
    PROPOSAL_MAP.with(|p| p.borrow().get(&key))
}
#[ic_cdk_macros::query]
fn get_proposal_count() -> u64 {
    PROPOSAL_MAP.with(|p| p.borrow().len())
}

#[ic_cdk_macros::query]
fn get_proposal_status(key: u64) -> Option<&'static str> {
    PROPOSAL_MAP.with(|p| {
        let proposal = match p.borrow().get(&key) {
            Some(value) => value,
            None => return None,
        };

        if proposal.voted.len() < 5 {
            // The proposal does not have enough votes for evaluation.
            return Some("Undecided");
        }

        let total_votes = proposal.approve + proposal.reject + proposal.pass;
        let approval_percentage = (proposal.approve as f64 / total_votes as f64) * 100.0;
        let rejection_percentage = (proposal.reject as f64 / total_votes as f64) * 100.0;
        let pass_percentage = (proposal.pass as f64 / total_votes as f64) * 100.0;

        if approval_percentage >= 50.0 {
            Some("Approved")
        } else if rejection_percentage >= 50.0 {
            Some("Rejected")
        } else if pass_percentage >= 50.0 {
            Some("Passed")
        } else {
            Some("Undecided")
        }
    })
}



#[ic_cdk_macros::update]
fn create_proposal(key: u64, proposal: CreateProposal) -> Option<Proposal> {    let value = Proposal {
        description: proposal.description,
        approve: 0u32,
        reject: 0u32,
        pass: 0u32,
        is_active: proposal.is_active,
        voted: vec![],
        owner: ic_cdk::caller(),
    };
    PROPOSAL_MAP.with(|p| p.borrow_mut().insert(key, value))
}


#[ic_cdk_macros::update]
fn edit_proposal(key: u64, proposal: CreateProposal) -> Result<(), VoteError> {
    PROPOSAL_MAP.with(|p| {
        let old_proposal = match p.borrow().get(&key) {
            Some(value) => value,
            None => return Err(VoteError::NoProposal),
        };
        if ic_cdk::caller() != old_proposal.owner {            return Err(VoteError::Unauthorized);
        }
        let value = Proposal {
            description: proposal.description,
            approve: old_proposal.approve,
            reject: old_proposal.reject,
            pass: old_proposal.pass,
            is_active: proposal.is_active,
            voted: old_proposal.voted,
            owner: ic_cdk::caller(),
        };
        let res = p.borrow_mut().insert(key, value);
        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::UpdateError),
        }
    })
}


#[ic_cdk_macros::update]
fn end_proposal(key: u64) -> Result<(), VoteError> {
    PROPOSAL_MAP.with(|p| {
        let mut proposal = p.borrow_mut().get(&key).unwrap();
        if ic_cdk::caller() != proposal.owner {
            return Err(VoteError::Unauthorized);
        }
        proposal.is_active = false;
        let res = p.borrow_mut().insert(key, proposal);
        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::UpdateError),
        }
    })
}


#[ic_cdk_macros::update]
fn vote(key: u64, choice: VoteTypes) -> Result<(), VoteError> {
    PROPOSAL_MAP.with(|p| {
        let mut proposal = p.borrow_mut().get(&key).unwrap();
        let caller = ic_cdk::caller();
        if proposal.voted.contains(&caller) {
            return Err(VoteError::AlreadyVoted);
        } else if !proposal.is_active {
            return Err(VoteError::ProposalNotActive);
        }
        match choice {
            VoteTypes::Approve => proposal.approve += 1,
            VoteTypes::Reject => proposal.reject += 1,
            VoteTypes::Pass => proposal.pass += 1,
        }
        proposal.voted.push(caller);
        let res = p.borrow_mut().insert(key, proposal);
        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::VoteFailed),
        }
    })
}

