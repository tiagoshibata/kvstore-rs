fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("Key: {}, value: {}", key, value);
    match write_database(key, value) {
        Ok(()) => {}
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    std::fs::write("kv.db", format!("{} {}\n", key, value))
}
