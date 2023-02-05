struct Color {
    red: u8, // u8: 0-255 arasını destekler bu da rgb icin yeterli
    green: u8,
    blue: u8
}

fn main() {
    // color: red, green, blue
    let mut bg = Color {red: 255, green: 70, blue: 15};

    bg.blue = 45; 

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
}
