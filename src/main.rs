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
}

// draw the current cells' status
fn draw(arr: &Vec<Vec<MyCell>>) -> () {
    for i in arr {
        for j in i {
            match j.is_alive {
                true => print!("+"),
                false => print!("-"),
            }
        }
        print!("\n");
    }
}

// set alive cells
fn set_alive(arr: &mut Vec<Vec<MyCell>>) -> () {
    // 2*2
    arr[10][10].is_alive = true;
    arr[11][10].is_alive = true;
    arr[10][11].is_alive = true;
    arr[11][11].is_alive = true;

    // 2*2
    arr[16][41].is_alive = true;
    arr[16][40].is_alive = true;
    arr[15][41].is_alive = true;
    arr[15][40].is_alive = true;
}

// count_cells_around
fn count_cells_around(arr: &Vec<Vec<MyCell>>, cols: usize, rows: usize) -> i32 {
    let max_cols = arr.len();
    let max_rows = arr[0].len();

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
    let max_cols: usize = arr.len();
    let max_rows: usize = arr[0].len();
    for i in (0..max_cols) {
        for j in (0..max_rows) {
            match count_cells_around(&arr, i, j) {
                -1 => continue,
                count => arr[i][j].alive_around_cnt = count as usize,
            }
        }
    }
}

fn update_mycells(arr: &mut Vec<Vec<MyCell>>) -> () {
    let max_cols: usize = arr.len();
    let max_rows: usize = arr[0].len();
    for i in 0..max_cols {
        for j in 0..max_rows {
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
    print!("\x1b[2J\x1b[3J\x1b[H");
    io::stdout().flush().unwrap();
}

fn main() {
    // init an array
    let cols: usize = 50;
    let rows: usize = 50;
    let mut arr: Vec<Vec<MyCell>> = (0..cols)
        .map(|p_cols| {
            (0..rows)
                .map(|p_rows| MyCell {
                    is_alive: false,
                    cols: p_cols,
                    rows: p_rows,
                    alive_around_cnt: 0,
                })
                .collect()
        })
        .collect();
    // functions to process
    set_alive(&mut arr);
    //    loop {
    thread::sleep(Duration::from_secs(3));
    //        update_alive_around_count(&mut arr);
    //        update_mycells(&mut arr);
    draw(&arr);
    flush_the_screen();
    //    }
}
