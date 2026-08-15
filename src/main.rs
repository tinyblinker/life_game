use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

// alive: true->alive, false->dead
struct MyCell {
    is_alive: bool,
    cols: usize,
    rows: usize,
    alive_around_cnt: usize,
    is_father: bool,
}

// draw the current cells' status
fn draw(arr: &Vec<Vec<MyCell>>) -> () {
    for i in arr {
        for j in i {
            match j.is_alive {
                true => print!("O"),
                false => print!("-"),
            }
        }
        print!("\n");
    }
}

// set alive cells (fathers)
// fn set_fathers_alive(arr: &mut Vec<Vec<MyCell>>, cols: usize, rows: usize) -> () {
//     let direction: [(i32, i32); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];
//     for (dc, dr) in direction {
//         let p_cols: usize = (cols as i32 + dc) as usize;
//         let p_rows: usize = (rows as i32 + dr) as usize;
//         arr[p_cols][p_rows].is_alive = true;
//         arr[p_cols][p_rows].is_father = true;
//     }
// }

// set alive cells (seeds)
fn set_seeds_alive(arr: &mut Vec<Vec<MyCell>>, cols: usize, rows: usize) -> () {
    // other cells to be alive
    arr[cols][rows].is_alive = true;
}

// count_cells_around
fn count_cells_around(arr: &Vec<Vec<MyCell>>, cols: usize, rows: usize) -> i32 {
    let max_cols = arr.len() - 1;
    let max_rows = arr[0].len() - 1;

    // exceptions (do not include borders)
    if max_cols == cols || 0 == cols {
        return -1;
    }
    if max_rows == rows || 0 == rows {
        return -1;
    }

    // mainly dealt with
    let mut count: i32 = 0;
    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
        (1, 0),
        (0, 1),
        (1, 1),
    ];
    for (dr, dc) in directions {
        let p_rows = (rows as i32) + dr;
        let p_cols = (cols as i32) + dc;
        if arr[p_cols as usize][p_rows as usize].is_alive == true {
            count += 1;
        }
    }
    count
}

fn update_alive_around_count(arr: &mut Vec<Vec<MyCell>>) -> () {
    let max_cols: usize = arr.len() - 1;
    let max_rows: usize = arr[0].len() - 1;
    for i in 0..max_cols {
        for j in 0..max_rows {
            match count_cells_around(&arr, i, j) {
                -1 => continue,
                count => arr[i][j].alive_around_cnt = count as usize,
            }
        }
    }
}

fn update_mycells(arr: &mut Vec<Vec<MyCell>>) -> () {
    let max_cols: usize = arr.len() - 1;
    let max_rows: usize = arr[0].len() - 1;
    for i in 0..max_cols {
        for j in 0..max_rows {
            if arr[i][j].is_father {
                continue;
            }
            match arr[i][j].is_alive {
                true => {
                    if arr[i][j].alive_around_cnt < 2 {
                        arr[i][j].is_alive = false;
                    } else if arr[i][j].alive_around_cnt > 3 {
                        arr[i][j].is_alive = false;
                    }
                }
                false => {
                    if arr[i][j].alive_around_cnt == 3 {
                        arr[i][j].is_alive = true;
                    }
                }
            }
        }
    }
}

fn flush_the_screen() -> () {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
    // thread::sleep(Duration::from_secs_f32(0.1)); // test ok!
}

fn draw_the_gun(arr: &mut Vec<Vec<MyCell>>) -> () {
    let directions: [(usize, usize); 36] = [
        (24, 0),
        (22, 1),
        (24, 1),
        (12, 2),
        (13, 2),
        (20, 2),
        (21, 2),
        (34, 2),
        (35, 2),
        (11, 3),
        (15, 3),
        (20, 3),
        (21, 3),
        (34, 3),
        (35, 3),
        (0, 4),
        (1, 4),
        (10, 4),
        (16, 4),
        (20, 4),
        (21, 4),
        (0, 5),
        (1, 5),
        (10, 5),
        (14, 5),
        (16, 5),
        (17, 5),
        (22, 5),
        (24, 5),
        (10, 6),
        (16, 6),
        (24, 6),
        (11, 7),
        (15, 7),
        (12, 8),
        (13, 8),
    ];
    for (dr, dc) in directions {
        set_seeds_alive(arr, dr + 2, dc + 25);
    }
}

fn main() {
    // init an array
    let cols: usize = 40;
    let rows: usize = 100;
    let mut arr: Vec<Vec<MyCell>> = (0..cols)
        .map(|p_cols| {
            (0..rows)
                .map(|p_rows| MyCell {
                    is_alive: false,
                    cols: p_cols,
                    rows: p_rows,
                    alive_around_cnt: 0,
                    is_father: false,
                })
                .collect()
        })
        .collect();
    // functions to process
    // set_fathers_alive(&mut arr, 10, 10); // test ok!
    // set_fathers_alive(&mut arr, 25, 40); // test ok!
    // set_seeds_alive(&mut arr, 12, 12);
    // set_seeds_alive(&mut arr, 12, 14);
    // set_seeds_alive(&mut arr, 13, 15);
    // set_seeds_alive(&mut arr, 14, 15);
    // set_seeds_alive(&mut arr, 15, 12);
    // set_seeds_alive(&mut arr, 15, 15);
    // set_seeds_alive(&mut arr, 16, 13);
    // set_seeds_alive(&mut arr, 16, 14);
    // set_seeds_alive(&mut arr, 16, 15);
    draw_the_gun(&mut arr); // draw the gun
    loop {
        draw(&arr); // test ok!
        thread::sleep(Duration::from_secs_f32(0.15)); // test ok!
        flush_the_screen(); // test ok!
        update_alive_around_count(&mut arr);
        update_mycells(&mut arr);
    }
}
