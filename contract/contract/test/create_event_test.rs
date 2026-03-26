#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

use crate::{
    base::errors::SecondCrowdfundingError, crowdfunding::CrowdfundingContract,
    interfaces::second_crowdfunding::SecondCrowdfundingTrait,
};

fn string_of_len(env: &Env, len: usize) -> String {
    String::from_str(env, &"a".repeat(len))
}

#[test]
fn test_create_event_success_path_returns_ok_for_valid_titles() {
    let env = Env::default();

    let creator = Address::generate(&env);
    let token = Address::generate(&env);
    let base_deadline = env.ledger().timestamp() + 86_400;

    let valid_title_lengths = [1usize, 50usize, 200usize];

    for (index, title_len) in valid_title_lengths.into_iter().enumerate() {
        let id = BytesN::from_array(&env, &[(index + 1) as u8; 32]);
        let title = string_of_len(&env, title_len);

        let result = <CrowdfundingContract as SecondCrowdfundingTrait>::create_event(
            env.clone(),
            id,
            title,
            creator.clone(),
            100,
            500,
            base_deadline + index as u64,
            token.clone(),
        );

        assert_eq!(
            result,
            Ok::<(), SecondCrowdfundingError>(()),
            "create_event should succeed for title length {title_len}"
        );
    }
}
