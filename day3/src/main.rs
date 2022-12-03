

fn split_compartments(input:String) -> (String, String) {
    let len = input.len();
    let right = len / 2;

    return ( String::from(&input[0..right]), String::from(&input[right..]))
}

fn make_compartment_mask(input:String) -> u64 {
    let mut mask: u64 = 0;
    let base = 2u64;

    for c in input.chars() {
        let n:u32 = map_char(c);
        mask = mask | base.pow(n);
    }

    mask
}

fn map_char(c: char) -> u32 {
    let x = c as u32;
    if x >=97  { x - 97 } else { x - 65 + 26 }
}


fn main() {
    let data: Vec<String> = commons::read_input("input.txt");


    let mut sum = 0;

    for d in data {
        let (left, right) = split_compartments(d);
        let left_mask = make_compartment_mask(left);
        let right_mask = make_compartment_mask(right);

        let sim = left_mask & right_mask;

        sum += sim.trailing_zeros() + 1;
    }

    println!("{sum}")
}
