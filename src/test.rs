use super::*;
use crate::storage::DataKey;
use soroban_sdk::{
    testutils::{storage::Persistent as _, Address as _, Events as _, Ledger as _},
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env,
};

pub(crate) struct Fixture {
    pub env: Env,
    pub contract: Address,
    pub admin: Address,
    pub token: Address,
    pub members: soroban_sdk::Vec<Address>,
}
impl Fixture {
    pub fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let contract = env.register(Contract, ());
        let admin = Address::generate(&env);
        let token = env
            .register_stellar_asset_contract_v2(admin.clone())
            .address();
        let members = vec![
            &env,
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ];
        let asset = StellarAssetClient::new(&env, &token);
        for member in members.iter() {
            asset.mint(&member, &1000);
        }
        Self {
            env,
            contract,
            admin,
            token,
            members,
        }
    }
    pub fn client(&self) -> ContractClient<'_> {
        ContractClient::new(&self.env, &self.contract)
    }
    pub fn create(&self) -> u64 {
        self.client()
            .create_circle(&self.admin, &self.token, &10, &self.members, &86_400)
    }
    pub fn pay_all(&self, id: u64) {
        for member in self.members.iter() {
            self.client().contribute(&id, &member);
        }
    }
    pub fn assert_balance(&self, id: u64) {
        let circle = self.client().get_circle(&id);
        assert_eq!(circle.balance, circle.received_total - circle.paid_total);
        assert_eq!(
            TokenClient::new(&self.env, &self.token).balance(&self.contract),
            circle.balance
        );
    }
}

#[test]
fn three_members_three_rounds_full_accounting() {
    let f = Fixture::new();
    let id = f.create();
    f.assert_balance(id);
    for round in 0..3 {
        let status = f.client().get_round_status(&id);
        assert_eq!(status.round, round);
        assert_eq!(status.recipient, f.members.get(round));
        assert_eq!(status.outstanding_members, f.members);
        assert!(!status.can_settle);
        for member in f.members.iter() {
            f.client().contribute(&id, &member);
            f.assert_balance(id);
        }
        assert!(f.client().get_round_status(&id).can_settle);
        f.client().settle_round(&id);
        f.assert_balance(id);
    }
    let circle = f.client().get_circle(&id);
    assert!(circle.completed);
    assert_eq!(circle.round, 3);
    assert_eq!(circle.received_total, 90);
    assert_eq!(circle.paid_total, 90);
    let status = f.client().get_round_status(&id);
    assert!(status.completed);
    assert!(!status.can_settle);
    assert!(status.recipient.is_none());
    assert!(status.outstanding_members.is_empty());
    for member in f.members.iter() {
        assert_eq!(TokenClient::new(&f.env, &f.token).balance(&member), 1000);
    }
}
#[test]
fn fixed_order_pays_the_exact_pot() {
    let f = Fixture::new();
    let id = f.create();
    f.pay_all(id);
    f.client().settle_round(&id);
    let token = TokenClient::new(&f.env, &f.token);
    assert_eq!(token.balance(&f.members.get(0).unwrap()), 1020);
    assert_eq!(token.balance(&f.members.get(1).unwrap()), 990);
    assert_eq!(token.balance(&f.members.get(2).unwrap()), 990);
    f.assert_balance(id);
}
#[test]
fn sequential_ids_do_not_mix_circle_balances() {
    let f = Fixture::new();
    let first = f.create();
    let second = f.create();
    assert_eq!((first, second), (1, 2));
    let member = f.members.get(0).unwrap();
    f.client().contribute(&first, &member);
    f.client().contribute(&second, &member);
    assert_eq!(f.client().get_circle(&first).balance, 10);
    assert_eq!(f.client().get_circle(&second).balance, 10);
    assert_eq!(TokenClient::new(&f.env, &f.token).balance(&f.contract), 20);
    f.pay_remaining(first);
    f.client().settle_round(&first);
    assert_eq!(f.client().get_circle(&second).balance, 10);
    assert_eq!(TokenClient::new(&f.env, &f.token).balance(&f.contract), 10);
}
impl Fixture {
    fn pay_remaining(&self, id: u64) {
        for member in self.members.iter().skip(1) {
            self.client().contribute(&id, &member);
        }
    }
}
#[test]
fn flag_key_is_scoped_to_round_and_member() {
    let f = Fixture::new();
    let id = f.create();
    let member = f.members.get(0).unwrap();
    f.client().contribute(&id, &member);
    f.env.as_contract(&f.contract, || {
        assert_eq!(
            f.env
                .storage()
                .persistent()
                .get::<_, bool>(&DataKey::Paid(id, 0, member.clone())),
            Some(true)
        );
        assert!(!f
            .env
            .storage()
            .persistent()
            .has(&DataKey::Paid(id, 1, member.clone())));
    });
    f.pay_remaining(id);
    f.client().settle_round(&id);
    f.client().contribute(&id, &member);
    assert_eq!(f.client().get_circle(&id).balance, 10);
}
#[test]
fn circle_paid_list_prevents_duplicate_if_flag_is_missing() {
    let f = Fixture::new();
    let id = f.create();
    let member = f.members.get(0).unwrap();
    f.client().contribute(&id, &member);
    f.env.as_contract(&f.contract, || {
        f.env
            .storage()
            .persistent()
            .remove(&DataKey::Paid(id, 0, member.clone()))
    });
    assert_eq!(
        f.client().try_contribute(&id, &member),
        Err(Ok(types::Error::AlreadyContributed))
    );
    f.assert_balance(id);
}
#[test]
fn access_renews_circle_and_current_paid_flags() {
    let f = Fixture::new();
    let id = f.create();
    let member = f.members.get(0).unwrap();
    f.client().contribute(&id, &member);
    f.env
        .ledger()
        .with_mut(|ledger| ledger.sequence_number += 100);
    let before = f.env.as_contract(&f.contract, || {
        f.env
            .storage()
            .persistent()
            .get_ttl(&DataKey::Paid(id, 0, member.clone()))
    });
    f.client().get_round_status(&id);
    let after = f.env.as_contract(&f.contract, || {
        f.env
            .storage()
            .persistent()
            .get_ttl(&DataKey::Paid(id, 0, member.clone()))
    });
    assert!(after > before);
}
#[test]
fn round_length_is_retention_hint_not_an_enforced_deadline() {
    let f = Fixture::new();
    let id = f.create();
    f.env
        .ledger()
        .with_mut(|ledger| ledger.timestamp += 400 * 86_400);
    f.pay_all(id);
    f.client().settle_round(&id);
    f.assert_balance(id);
}
#[test]
fn member_cap_is_accepted() {
    let f = Fixture::new();
    let mut members = soroban_sdk::Vec::new(&f.env);
    for _ in 0..20 {
        members.push_back(Address::generate(&f.env));
    }
    let id = f
        .client()
        .create_circle(&f.admin, &f.token, &1, &members, &1);
    assert_eq!(f.client().get_circle(&id).members.len(), 20);
}
#[test]
fn create_and_contribute_require_authentication() {
    let f = Fixture::new();
    f.env.mock_auths(&[]);
    assert!(f
        .client()
        .try_create_circle(&f.admin, &f.token, &10, &f.members, &100)
        .is_err());
    f.env.mock_all_auths();
    let id = f.create();
    f.env.mock_auths(&[]);
    assert!(f
        .client()
        .try_contribute(&id, &f.members.get(0).unwrap())
        .is_err());
    f.env.mock_all_auths();
    f.assert_balance(id);
}
#[test]
fn settlement_requires_no_signer() {
    let f = Fixture::new();
    let id = f.create();
    f.pay_all(id);
    f.env.mock_auths(&[]);
    f.client().settle_round(&id);
    f.assert_balance(id);
}
#[test]
fn insufficient_token_balance_rolls_back_contribution() {
    let f = Fixture::new();
    let id = f
        .client()
        .create_circle(&f.admin, &f.token, &2000, &f.members, &100);
    assert!(f
        .client()
        .try_contribute(&id, &f.members.get(0).unwrap())
        .is_err());
    f.assert_balance(id);
    assert!(f.client().get_round_status(&id).paid_members.is_empty());
}
#[test]
fn failing_token_rolls_back_settlement() {
    let f = Fixture::new();
    let failing_token = f.env.register(FailingToken, ());
    let token_client = FailingTokenClient::new(&f.env, &failing_token);
    for member in f.members.iter() {
        token_client.mint(&member, &1000);
    }
    let id = f
        .client()
        .create_circle(&f.admin, &failing_token, &10, &f.members, &100);
    f.pay_all(id);
    FailingTokenClient::new(&f.env, &failing_token).set_fail_from(&f.contract);
    let before = f.client().get_circle(&id);
    assert!(f.client().try_settle_round(&id).is_err());
    assert_eq!(f.client().get_circle(&id), before);
    assert_eq!(before.balance, before.received_total - before.paid_total);
    assert_eq!(
        TokenClient::new(&f.env, &failing_token).balance(&f.contract),
        30
    );
    for member in f.members.iter() {
        assert_eq!(token_client.balance(&member), 990);
    }
}

#[soroban_sdk::contract]
struct FailingToken;
#[soroban_sdk::contractimpl]
impl FailingToken {
    pub fn mint(env: Env, address: Address, amount: i128) {
        env.storage().persistent().set(&address, &amount);
    }
    pub fn set_fail_from(env: Env, from: Address) {
        env.storage()
            .instance()
            .set(&soroban_sdk::symbol_short!("fail"), &from);
    }
    pub fn balance(env: Env, address: Address) -> i128 {
        env.storage().persistent().get(&address).unwrap_or(0)
    }
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let fail: Option<Address> = env
            .storage()
            .instance()
            .get(&soroban_sdk::symbol_short!("fail"));
        if fail == Some(from.clone()) {
            soroban_sdk::panic_with_error!(&env, types::Error::InvalidAmount);
        }
        let from_balance = Self::balance(env.clone(), from.clone());
        let to_balance = Self::balance(env.clone(), to.clone());
        assert!(amount > 0 && from_balance >= amount);
        env.storage()
            .persistent()
            .set(&from, &from_balance.checked_sub(amount).unwrap());
        env.storage()
            .persistent()
            .set(&to, &to_balance.checked_add(amount).unwrap());
    }
}
#[test]
fn creates_contributes_settles_and_completion_emit_events() {
    let f = Fixture::new();
    let id = f.create();
    assert_eq!(
        f.env
            .events()
            .all()
            .filter_by_contract(&f.contract)
            .events()
            .len(),
        1
    );
    for round in 0..3 {
        for member in f.members.iter() {
            f.client().contribute(&id, &member);
            assert_eq!(
                f.env
                    .events()
                    .all()
                    .filter_by_contract(&f.contract)
                    .events()
                    .len(),
                1
            );
        }
        f.client().settle_round(&id);
        assert_eq!(
            f.env
                .events()
                .all()
                .filter_by_contract(&f.contract)
                .events()
                .len(),
            if round == 2 { 2 } else { 1 }
        );
    }
}
