// BlockStripe unit tests.
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, test_utils, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        test_utils::VMContextBuilder::new()
            .signer_account_id("block-stripe.testnet".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn add_tenant_flow() {
        let context = get_context(false);
        testing_env!(context);

        let contract = BlockStripe::default();
        let tenant_id = BlockStripe::add_tenant(contract,  "some@gmail.com".to_string());
        
        assert_eq!(tenant_id, "block-stripe.testnet_0".to_string());
    }
}
