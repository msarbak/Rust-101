fn main() {
    let mut n = 0;

    loop {
        n += 1;

        if n == 7 {
            continue; // pas gec
        }

        if n > 10 {
            break; //donguyu durdur
        }

        println!("n is {}", n);
    }
}
