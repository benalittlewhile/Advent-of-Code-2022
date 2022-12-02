fn main() {
    let contents = include_str!("input.txt");
    let contents = contents.split("\n");

    let mut sums: Vec<i32> = Vec::new();

    let mut current_sum = 0;

    for item in contents {
        if item.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += item.parse::<i32>().expect("should be a number lol");
        }
    }

    let mut c_max = 0;
    for item in &sums {
        if item > &c_max {
            c_max = *item
        };
        println!("{}", item);
    }
    let index = sums.iter().position(|&r| r == c_max).unwrap();
    println!("{}", index);
    println!("{}", c_max);

    sums.sort();
    let result_val = sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3];
    println!("{}", result_val);
}
