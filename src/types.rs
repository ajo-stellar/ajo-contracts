use soroban_sdk::{contracterror, contractevent, contracttype, Address, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    CircleNotFound = 1,
    Completed = 10,
    NotMember = 11,
    AlreadyContributed = 12,
    MissingContributions = 13,
    InvalidAmount = 30,
    InvalidMembers = 31,
    DuplicateMember = 32,
    InvalidRoundLength = 33,
    ArithmeticOverflow = 34,
    InvalidAddress = 35,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: u64,
    pub admin: Address,
    pub token: Address,
    pub contribution_amount: i128,
    pub members: Vec<Address>,
    pub round_length_seconds: u64,
    pub created_at: u64,
    pub retention_until: u64,
    pub round: u32,
    pub paid_members: Vec<Address>,
    pub received_total: i128,
    pub paid_total: i128,
    pub balance: i128,
    pub completed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundStatus {
    pub circle_id: u64,
    pub round: u32,
    pub recipient: Option<Address>,
    pub paid_members: Vec<Address>,
    pub outstanding_members: Vec<Address>,
    pub can_settle: bool,
    pub completed: bool,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CircleCreated {
    #[topic]
    pub circle_id: u64,
    pub admin: Address,
    pub member_count: u32,
    pub contribution_amount: i128,
}
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contributed {
    #[topic]
    pub circle_id: u64,
    #[topic]
    pub round: u32,
    pub member: Address,
    pub amount: i128,
}
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundSettled {
    #[topic]
    pub circle_id: u64,
    #[topic]
    pub round: u32,
    pub recipient: Address,
    pub amount: i128,
}
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CircleCompleted {
    #[topic]
    pub circle_id: u64,
    pub received_total: i128,
    pub paid_total: i128,
}
