#[test]
fn test_check_compliance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HyperEnforcementContract);
    let client = HyperEnforcementContractClient::new(&env, &contract_id);
    
    client.init(&admin, &pi_coin_id);
    let compliant = client.check_compliance(&Symbol::new(&env, "merchant_pi"));
    assert!(compliant);  // Assuming Pi usage
}
