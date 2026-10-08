#![no_std]
mod circles;
#[cfg(test)]
mod error_paths;
mod storage;
#[cfg(test)]
mod test;
mod types;

use soroban_sdk::{contract, contractimpl, Address, Env, Vec};
use types::{Circle, Error, RoundStatus};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn create_circle(
        env: Env,
        admin: Address,
        token: Address,
        contribution_amount: i128,
        members: Vec<Address>,
        round_length_seconds: u64,
    ) -> Result<u64, Error> {
        circles::create(
            &env,
            admin,
            token,
            contribution_amount,
            members,
            round_length_seconds,
        )
    }
    pub fn contribute(env: Env, circle_id: u64, member: Address) -> Result<(), Error> {
        circles::contribute(&env, circle_id, member)
    }
    pub fn settle_round(env: Env, circle_id: u64) -> Result<(), Error> {
        circles::settle(&env, circle_id)
    }
    pub fn get_circle(env: Env, circle_id: u64) -> Result<Circle, Error> {
        storage::load(&env, circle_id)
    }
    pub fn get_round_status(env: Env, circle_id: u64) -> Result<RoundStatus, Error> {
        circles::status(&env, circle_id)
    }
}
