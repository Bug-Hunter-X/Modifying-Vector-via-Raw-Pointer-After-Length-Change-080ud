fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    v[index] = 4; //safe and correct way to modify the vector
    println!("v: {:?}", v);
}