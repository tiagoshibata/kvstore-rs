fn main() -> std::io::Result<()> {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let mut db = Database::from_disk()?;
    db.insert(key, value);
    db.flush()?;
    Ok(())
}

struct Database {
    hashmap: std::collections::HashMap<String, String>,
}

impl Database {
    fn from_disk() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kv.db")?;
        let mut hashmap = std::collections::HashMap::new();
        for line in contents.lines() {
            let mut parts = line.split('\t');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            hashmap.insert(key.to_string(), value.to_string());
        }
        Ok(Database {
            hashmap: hashmap,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.hashmap.insert(key, value);
    }

    fn serialize(self) -> String {
        let mut string = String::new();
        for (key, value) in self.hashmap.iter() {
            string.push_str(&key);
            string.push('\t');
            string.push_str(&value);
            string.push('\n');
        }
        string
    }

    fn flush(self) -> std::io::Result<()> {
        std::fs::write("kv.db", self.serialize())
    }
}
