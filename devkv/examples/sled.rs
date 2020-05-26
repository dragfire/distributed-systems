use std::fs;
use std::path::PathBuf;
use yakv::{YakvEngine, YakvSledEngine};

fn main() {
    fs::remove_dir_all("sled_data").unwrap();
    {
        let mut store = YakvSledEngine::open(PathBuf::from("sled_data")).unwrap();
        store.set("key1".to_string(), "value1".to_string()).unwrap();
        println!("{:?}", store.get("key1".to_string()).unwrap());
        store.set("key1".to_string(), "value2".to_string()).unwrap();
        println!("{:?}", store.get("key1".to_string()).unwrap());
        println!("{:?}", store.get("key2".to_string()).unwrap());
        match store.remove("key2".to_string()) {
            Ok(_) => {}
            Err(_) => {
                println!("Error remove");
            }
        }
        store.set("key2".to_string(), "value3".to_string()).unwrap();
        match store.remove("key1".to_string()) {
            Ok(_) => {}
            Err(_) => {
                println!("Error remove");
            }
        }

        drop(store);
    }

    {
        let mut store = YakvSledEngine::open(PathBuf::from("sled_daa")).unwrap();
        println!("{:?}", store.get("key2".to_string()).unwrap());
        println!("{:?}", store.get("key1".to_string()).unwrap());
    }
}
