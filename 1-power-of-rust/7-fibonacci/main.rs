use std::collections::HashMap;

const FIB_ZERO: u64 = 0;
const FIB_ONE: u64 = 1;

fn fib (index: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    match index {
        FIB_ZERO => FIB_ZERO,
        FIB_ONE => FIB_ONE,
        index => {
            // @todo Find why map key needs to be a reference.
            if cache.contains_key(&index) {
                // Return contents of mutable reference at key.
                // @todo Find why `unwrap()` is needed.
                return *cache.get(&index).unwrap();
            } else {
                let value = fib(index - 1, cache) + fib(index - 2, cache);
                cache.insert(index, value);
                value
            }
        }
    }
}

fn main () {
    // Create new mutable hash map.
    let mut cache = HashMap::new();
    for index in 0..21 {
        // Pass index and mutable reference to mutable hash map.
        let value = fib(index, &mut cache);
        println!("{}: {}", index, value);
    }
}
