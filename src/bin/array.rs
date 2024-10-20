fn main() {
    let mut array: [i32; 20] = [0; 20];
    for i in 0..20 {
        array[i] = i as i32 + 1;
    }

    println!("{:?}", array);
}
