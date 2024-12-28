fn main() {
    let mut v = vec![1, 2, 3];
    //Safe way to modify vector contents
    v[0] = 10;
    println!("v: {:?}", v);
}