fn main() {
    let arguments = std::env::args();
    for arg in arguments {
        println!("Got arg: {}", arg);
    }
}
