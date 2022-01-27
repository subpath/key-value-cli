use std::collections::btree_map::Values;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() {
    // filename is hardcoded :)
    let db_name = "key_value_storage.db";
    // create new file if not exists
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(db_name);

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut db = Database::new(db_name).unwrap();

    // read cmd arguments
    // first argument should be key and second is value
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next();
    if value.is_none() {
        let value_by_key = db.read(&key);
        if value_by_key.is_none() {
            println!("Can't find key {} in db", key)
        } else {
            println!("{}", value_by_key.unwrap());
        }
    } else {
        let value = value.unwrap();
        db.insert(&key, &value);
        db.write(db_name, &key, &value);
    }
}

struct Database {
    field: HashMap<String, String>,
}

impl Database {
    fn new(db_name: &str) -> Result<Database, std::io::Error> {
        let contens: String = fs::read_to_string(db_name)?;
        // ? is the same thing as
        // let contens = match fs::read_to_string(db_name) {
        //     Ok(c) => c,
        //     Err(error) => return Err(error),
        // };
        let mut field = HashMap::new();
        for line in contens.lines() {
            let line_splitted: Vec<&str> = line.split('\t').collect();
            if line_splitted.len() != 2 {
                todo!("Handle this error")
            }
            let key = line_splitted[0];
            let value = line_splitted[1];
            field.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { field: field })
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.field.insert(key.to_owned(), value.to_owned());
    }

    fn read(&self, key: &str) -> Option<&String> {
        return self.field.get(key);
    }

    fn write(&self, db_name: &str, key: &str, value: &str) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(db_name)
            .unwrap();

        let line = format!("{}\t{}\n", key, value);
        file.write_all(line.as_bytes());
    }
}
