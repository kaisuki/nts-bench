// #![feature(test)]
// extern crate test;
use crate::nts::nts::obtain_nts_time;

pub mod records;
pub mod nts_protocol;
pub mod byteorder;
pub mod nts;


fn main() {
    obtain_nts_time()
}

