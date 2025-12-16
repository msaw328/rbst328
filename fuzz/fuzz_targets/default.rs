#![no_main]

use libfuzzer_sys::fuzz_target;

use rbst328::*;

fuzz_target!(|data: Vec<(u32, String)>| {
    let mut bst = BSTMap::<u32, String>::new();

    // Insert some data
    for (k, v) in data.iter() {
        bst.insert(*k, v.clone());
    }

    // Mutate the bst
    for (k, v) in bst.iter_mut() {
        v.insert_str(0, &k.to_string());
    }

    // Duplicate some data
    for (k, v) in data.iter() {
        bst.insert(*k, v.clone());
    }

    // Mutate the data again
    for (k, v) in bst.iter_mut() {
        v.insert_str(0, &k.to_string());
    }

    // Remove some data
    for (k, _) in data.iter() {
        bst.remove(*k);
    }

    drop(bst);
});
