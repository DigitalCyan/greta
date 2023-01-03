use crate::greta::{fuel::FuelType, ship::Ship};
use read_stdin::read_stdin_until_ok;
use std::{fmt::Display, io::{stdout, Write}};

pub fn run() {
    let mut stdout = stdout().lock();

    let ships = vec![Ship::Sevastopol, Ship::Novorossiysk];

    print!("Distance [km]: ");
    stdout.flush().expect("Failed to flush stdout");

    let kilometers: f32 = read_stdin_until_ok();

    println!("Select a ship:");
    let ship = selection(&ships);

    let price = ship.get_price_for_distance(kilometers, FuelType::Methane.get_price());

    println!(
        "To travel {} with {} you will need to cash out {} Eur",
        kilometers, ship, price
    );
}

pub fn selection<T>(entries: &Vec<T>) -> T
where
    T: Display + Clone,
{
    loop {
        for (i, entry) in entries.iter().enumerate() {
            println!("[{}] {}", i + 1, entry);
        }

        let selection: usize = read_stdin_until_ok();

        if selection < 1 || selection > entries.len() {
            println!("Please select a valid entry.");
            continue;
        }

        return entries[selection - 1].clone();
    }
}
