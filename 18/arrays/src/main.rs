fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    //her 2 si'de aynidir, sart degil ama eklenedebilir

    let numbers2 = [2; 400]; // 400 tane 2 den olusan dizi olusturur

    // numbers[0] // 1
    // numbers[3] // 4

    

    for n in numbers.iter() {
        println!("{}", n);
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
}
