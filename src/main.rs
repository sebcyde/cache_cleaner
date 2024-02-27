mod helper_functions;

use crate::helper_functions::helpers::{read_config, ConfigInterface};
use std::{path::Path, thread};

fn main() {
    println!("\nRunning Cache Cleaner!\n");

    let user_config: ConfigInterface = read_config();

    let handle_one = thread::spawn(move || {
        if user_config.flight_club.eq("") {
            println!("No Flight Club cache directory set. Skipping");
        } else {
            println!("Removing Flight Club Cache.");
            std::fs::remove_dir_all(Path::new(user_config.flight_club.as_str().unwrap()))
                .expect("Failed to delete FC cache");
        }
    });

    let handle_two = thread::spawn(move || {
        if user_config.electric_shuffle.eq("") {
            println!("No Electric Shuffle cache directory set. Skipping");
        } else {
            println!("Removing Electric Shuffle Cache.");
            std::fs::remove_dir_all(Path::new(user_config.electric_shuffle.as_str().unwrap()))
                .expect("Failed to delete ES cache");
        }
    });

    let handle_three = thread::spawn(move || {
        if user_config.red_engine.eq("") {
            println!("No Red Engine cache directory set. Skipping");
        } else {
            println!("Removing Red Engine Cache.");
            std::fs::remove_dir_all(Path::new(user_config.red_engine.as_str().unwrap()))
                .expect("Failed to delete RE cache");
        }
    });

    // Wait for all threads to finish
    handle_one.join().unwrap();
    handle_two.join().unwrap();
    handle_three.join().unwrap();

    println!("All functions completed!\n");
}
