use crate::types::{Circle, Error};
use soroban_sdk::{contracttype, Address, Env};

pub const DAY_LEDGERS: u32 = 17_280;
pub const RETENTION_MARGIN: u64 = 30 * 86_400;
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    NextId,
    Circle(u64),
    Paid(u64, u32, Address),
}

pub fn instance_ttl(env: &Env) {
    let target = (30 * DAY_LEDGERS).min(env.storage().max_ttl());
    env.storage().instance().extend_ttl(target / 2, target);
}
pub fn extend(env: &Env, key: &DataKey, until: u64) {
    let seconds = until
        .saturating_add(RETENTION_MARGIN)
        .saturating_sub(env.ledger().timestamp());
    let ledgers = u32::try_from(seconds.div_ceil(5)).unwrap_or(u32::MAX);
    let target = ledgers.max(7 * DAY_LEDGERS).min(env.storage().max_ttl());
    env.storage().persistent().extend_ttl(key, target, target);
}
pub fn load(env: &Env, id: u64) -> Result<Circle, Error> {
    instance_ttl(env);
    let key = DataKey::Circle(id);
    let circle: Circle = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(Error::CircleNotFound)?;
    extend(env, &key, circle.retention_until);
    for member in circle.paid_members.iter() {
        let paid_key = DataKey::Paid(id, circle.round, member);
        if env.storage().persistent().has(&paid_key) {
            extend(env, &paid_key, circle.retention_until);
        }
    }
    Ok(circle)
}
pub fn save(env: &Env, circle: &Circle) {
    let key = DataKey::Circle(circle.id);
    env.storage().persistent().set(&key, circle);
    extend(env, &key, circle.retention_until);
}
