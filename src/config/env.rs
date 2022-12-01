pub enum Env {
    Development,
    Production,
    Test,
}

const TEST: &str = "test";
const DEVELOPMENT: &str = "dev";
const PRODUCTION: &str = "prod";

impl Env {
    pub fn as_str(&self) -> &str {
        match self {
            Env::Development => DEVELOPMENT,
            Env::Production => PRODUCTION,
            Env::Test => TEST,
        }
    }
}

pub fn get_env() -> &'static str {
    // let env = std::env::var("ENV")
    // .expect("`ENV` must be one of `dev`, `prod`, `test` but currently invalid value set");
    let env = std::env::var("ENV").unwrap_or(TEST.to_string());
    match env.as_str() {
        /*
            ==========================================
            DEFAULT("") | TEST                  "test"
            DEVELOPMENT                          "dev"
            PRODUCTION                          "prod"
            ==========================================
        */
        TEST => TEST,
        DEVELOPMENT => DEVELOPMENT,
        PRODUCTION => PRODUCTION,
        _ => panic!("invalid `ENV` set"),
    }
}

#[test]
fn test_get_env() {
    let env = get_env();
    assert_eq!(TEST, env);

    std::env::set_var("ENV", TEST);
    let env = get_env();
    assert_eq!(TEST, env);

    std::env::set_var("ENV", DEVELOPMENT);
    let env = get_env();
    assert_eq!(DEVELOPMENT, env);

    std::env::set_var("ENV", PRODUCTION);
    let env = get_env();
    assert_eq!(PRODUCTION, env);
}

#[test]
#[should_panic(expected = "invalid `ENV` set")]
// #[should_panic(expected = "")]
fn test_get_env_with_invalid_env() {
    std::env::set_var("ENV", "invalid");
    let env = get_env();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_env() {
//         let env = get_env();
//         assert_eq!(TEST, env);

//         std::env::set_var("ENV", TEST);
//         let env = get_env();
//         assert_eq!(TEST, env);

//         std::env::set_var("ENV", DEVELOPMENT);
//         let env = get_env();
//         assert_eq!(DEVELOPMENT, env);

//         std::env::set_var("ENV", PRODUCTION);
//         let env = get_env();
//         assert_eq!(PRODUCTION, env);
//     }

//     #[test]
//     #[should_panic(expected = "invalid `ENV` set")]
//     // #[should_panic(expected = "")]
//     fn test_get_env_with_invalid_env() {
//         std::env::set_var("ENV", "invalid");
//         let env = get_env();
//     }
// }
