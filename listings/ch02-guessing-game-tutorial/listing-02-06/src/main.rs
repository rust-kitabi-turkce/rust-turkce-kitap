use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Sayıyı tahmin et!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Lütfen tahminini gir.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Satır okunamadı");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tahminin: {guess}");

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
