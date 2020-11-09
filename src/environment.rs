///Checking Env
use super::conf::Config;

pub fn check_env() -> Option<Config> {
    use std::env;
    let con = super::conf::Config::default();
// Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
    }
    Some(con)
}
