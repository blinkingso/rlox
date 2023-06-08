fn main() {
    let mut vec = vec![1, 2, 3];
    let num = &vec[2];
    println!("e is : {}", *num);
    vec.push(4);
}
