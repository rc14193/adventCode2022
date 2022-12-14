use num;
use std::collections::VecDeque;

fn main() {
    // today was just easier to parse manually
    // didn't feel like writing a parser
    let mut inspection_counts = [0u64,0,0,0,0,0,0,0];
    let mut example_inspection_counts = [0u64,0,0,0];
    let mut start_items: Vec<VecDeque<u64>> = vec![
        VecDeque::from([97, 81, 57, 57, 91, 61]),
        VecDeque::from([88, 62, 68, 90]),
        VecDeque::from([74, 87]),
        VecDeque::from([53, 81, 60, 87, 90, 99, 75]),
        VecDeque::from([57]),
        VecDeque::from([54, 84, 91, 55, 59, 72, 75, 70]),
        VecDeque::from([95, 79, 79, 68, 78]),
        VecDeque::from([61, 97, 67]),
    ];
    let mut example_start_items: Vec<VecDeque<u64>> = vec![
        VecDeque::from([79,98]),
        VecDeque::from([54,65,75,74]),
        VecDeque::from([79,60,97]),
        VecDeque::from([74])
    ];
    let operations: Vec<fn(u64) -> u64> = vec![
        |val: u64| -> u64 {val*7},
        |val: u64| -> u64 {val*17},
        |val: u64| -> u64 {val+2},
        |val: u64| -> u64 {val+1},
        |val: u64| -> u64 {val+6},
        |val: u64| -> u64 {val*val},
        |val: u64| -> u64 {val+3},
        |val: u64| -> u64 {val+4},
        ];
    let example_operations: Vec<fn(u64) -> u64> = vec![
        |val: u64| -> u64 {val*19},
        |val: u64| -> u64 {val+6},
        |val: u64| -> u64 {val*val},
        |val: u64| -> u64 {val+3},
    ];

    let tests = [11u64,19,5,2,13,7,3,17];
    let example_tests = [23,19,13,17];
    let destinations = [(5usize,6usize),(4,2),(7,4),(2,1),(7,0),(6,3),(1,3),(0,5)];
    let example_destinations = [(2,3),(2,0),(1,3),(0,1)];
    let remainder: u64 = tests.clone().into_iter().reduce(num::integer::lcm).unwrap();
    let example_big_remainder: u64 = example_tests.clone().into_iter().reduce(num::integer::lcm).unwrap();
    let example_divisor = 3u64;

    // actual opeartions
    for _rounds in 0u64..10_000 { // 20 rounds p1
        println!("On round {}", _rounds+1);
        for monkey in 0..start_items.len() { // iterate through each monkey
            for _i in 0..start_items[monkey].len() { // iterate through each item that each monkey has
                inspection_counts[monkey] += 1;
                let mut item = start_items[monkey].pop_front().unwrap(); 
                item = operations[monkey](item) % remainder;
                //item = item / example_divisor; // part 1 operation
                if item % tests[monkey] == 0 {
                    start_items[destinations[monkey].0].push_back(item);
                } else {
                    start_items[destinations[monkey].1].push_back(item);
                }

            }
        }
    }
    inspection_counts.sort();
    println!("Final counts are {:?}", inspection_counts);
    let total = inspection_counts[inspection_counts.len()-1]*inspection_counts[inspection_counts.len()-2];
    println!("Total value is {}", total);
}
