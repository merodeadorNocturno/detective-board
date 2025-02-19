use dotenv::dotenv;
use std::env;

pub fn set_environment_variable(env_var: &str, defaul_value: &str) -> String {
    dotenv().ok();
    match env::var(env_var) {
        Ok(var_in_env_file) => var_in_env_file,
        Err(_) => defaul_value.to_string(),
    }
}
