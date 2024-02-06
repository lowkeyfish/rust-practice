use std::collections::HashMap;

mod test;


fn main() {
    let mut map = HashMap::new();
    let key = "a";
    let value = get_default(&mut map, key);
    println!("{key}:{value}");
}

fn get_default<'a>(map: &'a mut HashMap<String, i32>, key: &str) -> &'a mut i32 {
    match map.get_mut(key) {
        Some(value) => value,
        None => {
            map.insert(key.to_string().clone(), 0);
            map.get_mut(key).unwrap()
        }
    }
}



