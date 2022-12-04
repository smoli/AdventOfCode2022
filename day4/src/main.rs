
struct Range {
    a: u32,
    b: u32
}



fn get_ranges(inp: &String) -> Vec<Range> {
    let ranges = inp.split(",").collect::<Vec<&str>>();

    let mut result: Vec<Range> = vec![];

    if ranges.len() == 2 {
        let mut parts = ranges[0].split("-").collect::<Vec<&str>>();

        if parts.len() == 2 {
            result.push(Range { a: parts[0].parse().unwrap(), b: parts[1].parse().unwrap()})
        }

        parts = ranges[1].split("-").collect::<Vec<&str>>();

        if parts.len() == 2 {
            result.push(Range { a: parts[0].parse().unwrap(), b: parts[1].parse().unwrap()})
        }
    }

    result
}

fn inside(v: u32, range: &Range) -> bool {
    v >= range.a && v <= range.b
}

fn one_contains_the_other(range1: &Range, range2: &Range) -> bool {
    if inside(range1.a, range2) && inside(range1.b, range2) {
        return true;
    }

    if inside(range2.a, range1) && inside(range2.b, range1) {
        return true;
    }

    return false;
}

fn overlaps(range1: &Range, range2: &Range) -> bool {
    if inside(range1.a, range2) || inside(range1.b, range2) {
        return true;
    }

    if inside(range2.a, range1) || inside(range2.b, range1) {
        return true;
    }

    return false;
}


fn main() {
    let data = commons::read_input("input.txt");

    let mut sum = 0;
    for line in &data {
        let r = get_ranges(&String::from(line));

        if one_contains_the_other(&r[0], &r[1]) {
            sum += 1;
        }
    }

    println!("Containing ranges: {}", sum);


    sum = 0;
    for line in &data {
        let r = get_ranges(&String::from(line));

        if overlaps(&r[0], &r[1]) {
            sum += 1;
        }
    }

    println!("Overlapping ranges: {}", sum);
}
