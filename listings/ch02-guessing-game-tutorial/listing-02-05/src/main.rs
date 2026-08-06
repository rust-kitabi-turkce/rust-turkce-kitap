use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Sayıyı tahmin et!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("Gizli sayı: {secret_number}");

    loop {
        println!("Lütfen tahminini gir.");

        let mut guess = String::new();

        // ANCHOR: here
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Satır okunamadı");

        // ANCHOR: ch19
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // ANCHOR_END: ch19

        println!("Tahminin: {guess}");

        // --snip--
        // ANCHOR_END: here

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Çok küçük!"),
            Ordering::Greater => println!("Çok büyük!"),
            Ordering::Equal => {
                println!("Kazandın!");
                break;
            }
        }
    }
}
