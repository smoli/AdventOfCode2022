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

fn scenic_score(map: &Vec<Vec<usize>>, sx: usize, sy: usize) -> u32 {
    let h = map.len();
    let w = map.get(0).unwrap().len();
    let tree_value = map[sy][sx];
    let mut x: usize;
    let mut y: usize;

    let mut scenic_score = 1;
    let mut score = 0;

    // L
    if sx > 0 {
        x = sx - 1;
        y = sy;
        loop {
            score += 1;
            if map[y][x] >= tree_value {
                break;
            }
            if x == 0 {
                break;
            }
            x -= 1;
        }
    }

    scenic_score *= score;
    score = 0;

    // R
    if sx < w - 1 {
        x = sx + 1;
        y = sy;
        loop {
            score += 1;
            if map[y][x] >= tree_value {
                break;
            }
            if x >= w - 1 {
                break;
            }
            x += 1;
        }
    }

    scenic_score *= score;
    score = 0;

    // D
    if sy < h - 1 {
        x = sx;
        y = sy + 1;
        loop {
            score += 1;
            if map[y][x] >= tree_value {
                break;
            }
            if y >= h - 1 {
                break;
            }

            y += 1;
        }
    }

    scenic_score *= score;
    score = 0;

    // U
    if sy > 0 {
        x = sx;
        y = sy - 1;

        loop {
            score += 1;
            if map[y][x] >= tree_value {
                break;
            }
            if y == 0 {
                break;
            }

            y -= 1;
        }
    }

    scenic_score *= score;

    scenic_score
}

fn find_visible(map: &Vec<Vec<usize>>) -> HashMap<usize, bool> {
    let mut res: HashMap<usize, bool> = HashMap::new();
    let mut x: usize;
    let mut y: usize;
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

    let res = find_visible(&map);

    let w = map.get(0).unwrap().len();
    let h = map.len();

    println!("Part one: {}", 2 * w + 2 * h + res.values().len() - 4);

    let mut max = 0;

    for x in 0..w-1 {
        for y in 0..h - 1 {
            let score = scenic_score(&map, x, y);

            if score > max {
                max = score;
            }

        }
    }

    println!("Part two: {max}");
}
