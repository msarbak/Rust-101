fn main() {
    let x = 10;

    {
        let y = 5; // buradan erisemiyor
    }
    
    let y = 5;
    println!("x: {} y: {}", x, y);
}
