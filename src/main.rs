mod server;

use server::server::connect_and_clean;

use std::thread::{self};

pub enum COMPANY {
    ELECTRICSHUFFLE,
    FLIGHTCLUB,
    REDENGINE,
}

fn main() {
    println!("\nRunning Cache Cleaner!\n");

    let es_handle = thread::spawn(move || connect_and_clean(COMPANY::ELECTRICSHUFFLE));
    let fc_handle = thread::spawn(move || connect_and_clean(COMPANY::FLIGHTCLUB));
    let re_handle = thread::spawn(move || connect_and_clean(COMPANY::REDENGINE));

    es_handle.join().unwrap();
    fc_handle.join().unwrap();
    re_handle.join().unwrap();

    println!("\nFinished clearing caches.\n");
}
