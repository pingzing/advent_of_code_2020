use rustc_hash::FxHashMap;

const INPUT: [u32; 7] = [9, 19, 1, 6, 0, 5, 4];

fn main() {
    let num = get_nth_number(2020);
    println!("PART ONE: {}", num);

    let bignum = get_nth_number(30000000);
    println!("PART TWO: {}", bignum);
}

fn get_nth_number(n: u32) -> u32 {
    let mut last_spoken_tracker = FxHashMap::default();
    let mut prev_spoken = INPUT[0];
    for (i, value) in INPUT.iter().enumerate() {
        last_spoken_tracker.insert(*value, i);
        prev_spoken = *value;
    }       

    for turn in 7..n {     
        let cur;
        if let Some(last_seen_idx) = last_spoken_tracker.get(&prev_spoken) {                           
                cur = turn - 1 - *last_seen_idx as u32;
            } else {                           
                cur = 0;
            }        
        
        // Now that prev_spoken is about to be at i-2, it can safely live in the last_spoken_tracker
        last_spoken_tracker.insert(prev_spoken, turn as usize - 1);
        prev_spoken = cur;
    }
        
    prev_spoken
}