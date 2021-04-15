#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]
mod bindingsMbuf;

pub mod client;

use std::result;

#[derive(Debug)]
pub enum Error {
    /// Failed to do rte_eal_init()
    EalInitFailed(i32),
    NotSecondaryDpdk,
    RingLookupFailed,
    MempoolLookupFailed,
    MempoolGetFailed
}

pub type Result<T> = result::Result<T, Error>;


pub fn play_winning_sound(name: String) {
    println!("Playing winning sound {}", name);
    client::test_func();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
