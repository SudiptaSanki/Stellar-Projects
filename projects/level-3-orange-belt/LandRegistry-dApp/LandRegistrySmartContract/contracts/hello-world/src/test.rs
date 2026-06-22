#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Address};

#[test]
fn test_register_and_fetch_property() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LandRegistry);
    let client = LandRegistryClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let location = String::from_str(&env, "Paris, France");
    let area = 150;

    let prop_id = client.register_property(&owner, &location, &area);
    assert_eq!(prop_id, 1);

    let property = client.fetch_property(&prop_id);
    assert_eq!(property.prop_id, 1);
    assert_eq!(property.owner, owner);
    assert_eq!(property.location, location);
    assert_eq!(property.area, area);
}