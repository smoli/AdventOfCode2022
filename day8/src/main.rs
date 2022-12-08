use std::collections::HashMap;
use commons::{make_grid, read_input};

/*fn print_map(map: &Vec<Vec<usize>>) {
    for l in map {
        for v in l {
            print!("{v}")
        }
        println!();
    }
}*/

fn find_visibles(map:&Vec<Vec<usize>>) -> HashMap<usize, bool> {
    let mut res: HashMap<usize, bool> = HashMap::new();
    let mut x:usize;
    let mut y:usize;
    let h = map.len();
    let w = map.get(0).unwrap().len();


    // LR
    y = 1;

    loop {
        let first = map[y][0];
        let mut max = first;
        x = 1;
        loop {

            if map[y][x] > max {
                res.insert(y * w + x, true);
                max = map[y][x];
            }

            x += 1;
            if x >= w - 1 {
                break;
            }
        }

        y += 1;
        if y >= h - 1 {
            break;
        }
    }

    // RL
    y = 1;
    loop {
        let first = map[y][w - 1];
        let mut max = first;
        x = w - 2;
        loop {
            if map[y][x] > max {
                res.insert(y * w + x, true);
                max = map[y][x];
            }

            x -= 1;
            if x < 1 {
                break;
            }
        }

        y += 1;
        if y >= h - 1 {
            break;
        }
    }

    // TD
    x = 1;
    loop {
        let first = map[0][x];
        let mut max = first;
        y = 1;
        loop {

            if map[y][x] > max {
                res.insert(y * w + x, true);
                max = map[y][x];
            }

            y += 1;
            if y >= h - 1 {
                break;
            }
        }

        x += 1;
        if x >= w - 1 {
            break;
        }
    }

    // BU
    x = 1;
    loop {
        let first = map[h - 1][x];
        let mut max = first;
        y = h - 2;
        loop {
            if map[y][x] > max {
                res.insert(y * w + x, true);
                max = map[y][x];
            }

            y -= 1;
            if y < 1 {
                break;
            }
        }

        x += 1;
        if x >= w - 1 {
            break;
        }
    }


    res
}



fn main() {

    let data = read_input("input.txt");
    let map = make_grid(data, 0usize);

    let res = find_visibles(&map);

    let w = map.get(0).unwrap().len();
    let h = map.len();

    println!("Part one: {}", 2 * w + 2 * h + res.values().len() - 4);

}
