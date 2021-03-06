#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate env_logger;

pub mod api;
mod commands;
mod errors;
mod services;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        assert! (true, "Dummy check!");
    }
}