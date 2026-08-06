// ANCHOR: here
use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Sayıyı tahmin et!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("Gizli sayı: {secret_number}");

    println!("Lütfen tahminini gir.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Satır okunamadı");
    // ANCHOR: here

    println!("Tahminin: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Çok küçük!"),
        Ordering::Greater => println!("Çok büyük!"),
        Ordering::Equal => println!("Kazandın!"),
    }
}
// ANCHOR_END: here
