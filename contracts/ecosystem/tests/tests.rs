#[test]
fn test_register_merchant() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EcosystemContract);
    let client = EcosystemContractClient::new(&env, &contract_id);
    
    client.init(&admin, &pi_coin_id);
    let products = Map::new(&env);
    products.set(Symbol::new(&env, "laptop"), 1000);
    let merchant = client.register_merchant(&Symbol::new(&env, "shop"), &products);
    assert_eq!(merchant.name, Symbol::new(&env, "shop"));
}
