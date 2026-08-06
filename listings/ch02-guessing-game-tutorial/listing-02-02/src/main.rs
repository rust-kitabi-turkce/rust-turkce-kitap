use std::io;

fn main() {
    println!("Sayıyı tahmin et!");

    println!("Lütfen tahminini gir.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Satır okunamadı");

    println!("Tahminin: {guess}");
}
