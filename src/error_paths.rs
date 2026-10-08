use crate::{test::Fixture, types::Error};
use soroban_sdk::{testutils::Address as _, vec, Address};

#[test]
fn circle_not_found() {
    let f = Fixture::new();
    assert_eq!(
        f.client().try_get_circle(&999),
        Err(Ok(Error::CircleNotFound))
    );
    assert_eq!(
        f.client().try_get_round_status(&999),
        Err(Ok(Error::CircleNotFound))
    );
    assert_eq!(
        f.client().try_settle_round(&999),
        Err(Ok(Error::CircleNotFound))
    );
}
#[test]
fn completed() {
    let f = Fixture::new();
    let id = f.create();
    for _ in 0..3 {
        f.pay_all(id);
        f.client().settle_round(&id);
    }
    assert_eq!(
        f.client().try_contribute(&id, &f.members.get(0).unwrap()),
        Err(Ok(Error::Completed))
    );
    assert_eq!(f.client().try_settle_round(&id), Err(Ok(Error::Completed)));
}
#[test]
fn not_member() {
    let f = Fixture::new();
    let id = f.create();
    assert_eq!(
        f.client().try_contribute(&id, &Address::generate(&f.env)),
        Err(Ok(Error::NotMember))
    );
    f.assert_balance(id);
}
#[test]
fn already_contributed() {
    let f = Fixture::new();
    let id = f.create();
    let member = f.members.get(0).unwrap();
    f.client().contribute(&id, &member);
    assert_eq!(
        f.client().try_contribute(&id, &member),
        Err(Ok(Error::AlreadyContributed))
    );
    f.assert_balance(id);
}
#[test]
fn missing_contributions() {
    let f = Fixture::new();
    let id = f.create();
    assert_eq!(
        f.client().try_settle_round(&id),
        Err(Ok(Error::MissingContributions))
    );
    for member in f.members.iter().take(2) {
        f.client().contribute(&id, &member);
    }
    assert_eq!(
        f.client().try_settle_round(&id),
        Err(Ok(Error::MissingContributions))
    );
    f.assert_balance(id);
}
#[test]
fn invalid_amount() {
    let f = Fixture::new();
    for amount in [0, -1] {
        assert_eq!(
            f.client()
                .try_create_circle(&f.admin, &f.token, &amount, &f.members, &100),
            Err(Ok(Error::InvalidAmount))
        );
    }
}
#[test]
fn invalid_members() {
    let f = Fixture::new();
    for members in [vec![&f.env], vec![&f.env, f.admin.clone()]] {
        assert_eq!(
            f.client()
                .try_create_circle(&f.admin, &f.token, &10, &members, &100),
            Err(Ok(Error::InvalidMembers))
        );
    }
    let mut members = soroban_sdk::Vec::new(&f.env);
    for _ in 0..21 {
        members.push_back(Address::generate(&f.env));
    }
    assert_eq!(
        f.client()
            .try_create_circle(&f.admin, &f.token, &10, &members, &100),
        Err(Ok(Error::InvalidMembers))
    );
}
#[test]
fn duplicate_member() {
    let f = Fixture::new();
    let members = vec![&f.env, f.admin.clone(), f.admin.clone()];
    assert_eq!(
        f.client()
            .try_create_circle(&f.admin, &f.token, &10, &members, &100),
        Err(Ok(Error::DuplicateMember))
    );
}
#[test]
fn invalid_round_length() {
    let f = Fixture::new();
    assert_eq!(
        f.client()
            .try_create_circle(&f.admin, &f.token, &10, &f.members, &0),
        Err(Ok(Error::InvalidRoundLength))
    );
}
#[test]
fn arithmetic_overflow() {
    let f = Fixture::new();
    assert_eq!(
        f.client()
            .try_create_circle(&f.admin, &f.token, &i128::MAX, &f.members, &100),
        Err(Ok(Error::ArithmeticOverflow))
    );
    assert_eq!(
        f.client()
            .try_create_circle(&f.admin, &f.token, &10, &f.members, &u64::MAX),
        Err(Ok(Error::ArithmeticOverflow))
    );
}
#[test]
fn invalid_address() {
    let f = Fixture::new();
    assert_eq!(
        f.client()
            .try_create_circle(&f.admin, &f.contract, &10, &f.members, &100),
        Err(Ok(Error::InvalidAddress))
    );
    let members = vec![&f.env, f.admin.clone(), f.contract.clone()];
    assert_eq!(
        f.client()
            .try_create_circle(&f.admin, &f.token, &10, &members, &100),
        Err(Ok(Error::InvalidAddress))
    );
}
