extern crate num;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod bench;
pub mod statistics;

pub use bench::*;