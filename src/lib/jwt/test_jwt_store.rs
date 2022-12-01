#[cfg(test)]
mod jwt_tests {
    use crate::lib::jwt::{
        ts::{exp_ts, now_ts},
        JwtStore,
    };

    #[test]
    fn test_verify_auth_token() {
        let jwt_store = JwtStore::get();
        let token = jwt_store.create_auth_token(1);
        let auth_claims = jwt_store.verify_auth_token(&token).unwrap();
        println!("{auth_claims:?}");
    }

    #[test]
    #[should_panic]
    fn test_verify_expired_auth_token() {
        let jwt_store = JwtStore::get();

        let exp = now_ts() - 1;
        let token = jwt_store.create_auth_token_with_exp(1, exp);
        let auth_claims = jwt_store.verify_auth_token(&token).unwrap();

        println!("{auth_claims:?}");
    }

    #[test]
    fn test_test() {
        println!("{}", now_ts());
        println!("{}", exp_ts());
    }
}
