use crate::{
    storage::{self, DataKey},
    types::*,
};
use soroban_sdk::{token::TokenClient, Address, Env, Vec};
pub const MAX_MEMBERS: u32 = 20;

pub fn create(
    env: &Env,
    admin: Address,
    token: Address,
    amount: i128,
    members: Vec<Address>,
    round_length: u64,
) -> Result<u64, Error> {
    admin.require_auth();
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    if members.len() < 2 || members.len() > MAX_MEMBERS {
        return Err(Error::InvalidMembers);
    }
    if round_length == 0 {
        return Err(Error::InvalidRoundLength);
    }
    if token == env.current_contract_address() {
        return Err(Error::InvalidAddress);
    }
    validate_members(env, &members)?;
    amount
        .checked_mul(i128::from(members.len()))
        .and_then(|pot| pot.checked_mul(i128::from(members.len())))
        .ok_or(Error::ArithmeticOverflow)?;
    let duration = round_length
        .checked_mul(u64::from(members.len()))
        .ok_or(Error::ArithmeticOverflow)?;
    let retention_until = env
        .ledger()
        .timestamp()
        .checked_add(duration)
        .ok_or(Error::ArithmeticOverflow)?;
    storage::instance_ttl(env);
    let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(1);
    let next = id.checked_add(1).ok_or(Error::ArithmeticOverflow)?;
    let circle = Circle {
        id,
        admin: admin.clone(),
        token,
        contribution_amount: amount,
        members,
        round_length_seconds: round_length,
        created_at: env.ledger().timestamp(),
        retention_until,
        round: 0,
        paid_members: Vec::new(env),
        received_total: 0,
        paid_total: 0,
        balance: 0,
        completed: false,
    };
    env.storage().instance().set(&DataKey::NextId, &next);
    storage::save(env, &circle);
    CircleCreated {
        circle_id: id,
        admin,
        member_count: circle.members.len(),
        contribution_amount: amount,
    }
    .publish(env);
    Ok(id)
}
fn validate_members(env: &Env, members: &Vec<Address>) -> Result<(), Error> {
    for (i, member) in members.iter().enumerate() {
        if member == env.current_contract_address() {
            return Err(Error::InvalidAddress);
        }
        for previous in members.iter().take(i) {
            if member == previous {
                return Err(Error::DuplicateMember);
            }
        }
    }
    Ok(())
}
pub fn contribute(env: &Env, id: u64, member: Address) -> Result<(), Error> {
    member.require_auth();
    let mut circle = storage::load(env, id)?;
    if circle.completed {
        return Err(Error::Completed);
    }
    if !circle.members.contains(&member) {
        return Err(Error::NotMember);
    }
    if circle.paid_members.contains(&member) {
        return Err(Error::AlreadyContributed);
    }
    circle.balance = circle
        .balance
        .checked_add(circle.contribution_amount)
        .ok_or(Error::ArithmeticOverflow)?;
    circle.received_total = circle
        .received_total
        .checked_add(circle.contribution_amount)
        .ok_or(Error::ArithmeticOverflow)?;
    TokenClient::new(env, &circle.token).transfer(
        &member,
        env.current_contract_address(),
        &circle.contribution_amount,
    );
    circle.paid_members.push_back(member.clone());
    let key = DataKey::Paid(id, circle.round, member.clone());
    env.storage().persistent().set(&key, &true);
    storage::extend(env, &key, circle.retention_until);
    storage::save(env, &circle);
    Contributed {
        circle_id: id,
        round: circle.round,
        member,
        amount: circle.contribution_amount,
    }
    .publish(env);
    Ok(())
}
pub fn settle(env: &Env, id: u64) -> Result<(), Error> {
    let mut circle = storage::load(env, id)?;
    if circle.completed {
        return Err(Error::Completed);
    }
    if circle.paid_members.len() != circle.members.len() {
        return Err(Error::MissingContributions);
    }
    let recipient = circle
        .members
        .get(circle.round)
        .ok_or(Error::ArithmeticOverflow)?;
    let amount = circle.balance;
    let paid_total = circle
        .paid_total
        .checked_add(amount)
        .ok_or(Error::ArithmeticOverflow)?;
    let next_round = circle
        .round
        .checked_add(1)
        .ok_or(Error::ArithmeticOverflow)?;
    TokenClient::new(env, &circle.token).transfer(
        &env.current_contract_address(),
        &recipient,
        &amount,
    );
    RoundSettled {
        circle_id: id,
        round: circle.round,
        recipient,
        amount,
    }
    .publish(env);
    circle.paid_total = paid_total;
    circle.balance = 0;
    circle.round = next_round;
    circle.paid_members = Vec::new(env);
    circle.completed = next_round == circle.members.len();
    storage::save(env, &circle);
    if circle.completed {
        CircleCompleted {
            circle_id: id,
            received_total: circle.received_total,
            paid_total,
        }
        .publish(env);
    }
    Ok(())
}
pub fn status(env: &Env, id: u64) -> Result<RoundStatus, Error> {
    let circle = storage::load(env, id)?;
    let mut outstanding = Vec::new(env);
    if !circle.completed {
        for member in circle.members.iter() {
            if !circle.paid_members.contains(&member) {
                outstanding.push_back(member);
            }
        }
    }
    Ok(RoundStatus {
        circle_id: id,
        round: circle.round,
        recipient: circle.members.get(circle.round),
        paid_members: circle.paid_members,
        can_settle: !circle.completed && outstanding.is_empty(),
        outstanding_members: outstanding,
        completed: circle.completed,
    })
}
