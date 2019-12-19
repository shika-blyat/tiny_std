use tiny_std::vec::Vector;
fn main() {
    let mut vec = Vector::new();
    vec.push(15);
    vec.push(17);
    vec.push(19);
    println!("{}", vec);
    vec.remove(1);
    println!("{}", vec);
}
