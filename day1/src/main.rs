use std::fs;

fn main() {
    let file_path = "sampleInput.txt";
    let contents = fs::read_to_string(file_path)
        .expect("File not found");

    let str_values = contents.split("\r\n").collect::<Vec<&str>>();
    let mut values: Vec<i32> = Vec::new();


    let mut sum: i32 = 0;
    for v in str_values {

        if v != "" {
            let num: i32 = v.parse().unwrap();
            sum += num;
        } else {
            values.push(sum);
            sum = 0;
        }
    }

    values.sort();
    values.reverse();

    println!("Max: {}", values[0]);
    println!("Top 3: {}", values[0] + values[1] + values[2]);
}
