// ANCHOR: all
use std::io;

// ANCHOR: ch07-04
use rand::prelude::*;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Sayıyı tahmin et!");

    // ANCHOR: ch07-04
    let secret_number = rand::rng().random_range(1..=100);
    // ANCHOR_END: ch07-04

    println!("Gizli sayı: {secret_number}");

    println!("Lütfen tahminini gir.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Satır okunamadı");

    println!("Tahminin: {guess}");
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
