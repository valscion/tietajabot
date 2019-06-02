use std::io;
fn main() {
    let mut raw_list = String::new();
    io::stdin()
        .read_line(&mut raw_list)
        .expect("Failed to read line!");

    let mut ints: Vec<i32> = Vec::new();

    for part in raw_list.split(",") {
        let parsed: i32 = match part.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only integers, please!");
                break;
            }
        };
        ints.push(parsed);
    }

    println!("{:#?}", ints);

    let mean = calc_average(&ints);
    let median = calc_median(&ints);
    let mode = calc_mode(&ints);
    println!("mean: {:.2}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode);
}

fn calc_average(ints: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for i in ints {
        sum += i;
    }
    let len = ints.len() as f32;
    (sum as f32) / len
}

fn calc_median(ints: &Vec<i32>) -> i32 {
    let mut sorted = ints.clone();
    sorted.sort();
    let middle_pos = sorted.len() / 2;
    sorted[middle_pos]
}

fn calc_mode(ints: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut seen_counts: HashMap<i32, u32> = HashMap::new();

    for &i in ints {
        let val = seen_counts.entry(i).or_insert(0);
        *val += 1;
    }

    let mut largest_count = 0;
    let mut value_seen = 0;

    for (key, value) in seen_counts {
        if value > largest_count {
            value_seen = key;
            largest_count = value;
        }
    }

    value_seen
}
