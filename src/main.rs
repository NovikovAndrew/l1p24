use std::collections::HashSet;


fn main() {
    println!("{}", is_uniq("abcd"));  // true
    println!("{}", is_uniq("abCdefAaf")); // false
    println!("{}", is_uniq("aabcd")); // false
    println!("{}", is_uniq("a1B2c3D4")); // true
    println!("{}", is_uniq("A1B2c3D4a")); // false
    println!("{}", is_uniq("abcdefghijklmnopqrstuvwxyz")); // true
}

fn is_uniq(s: &str) -> bool {
    // для проверки уникальных символов
    let mut set = HashSet::new();

    for c in s.to_lowercase().chars() {
         // если у нас есть символ в сете, возвращаем false
         if !set.insert(c) {
             return false
         }
    }

    true
}
