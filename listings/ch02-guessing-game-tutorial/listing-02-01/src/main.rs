// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Sayıyı tahmin et!");

    println!("Lütfen tahminini gir.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut guess = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut guess)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Satır okunamadı");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Tahminin: {guess}");
    // ANCHOR_END: print_guess
}
// ANCHOR: all
