use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Cell {
    Static(u8),
    Avail(HashSet<u8>)
}

impl std::default::Default for Cell {
    fn default() -> Self {
        Cell::Avail(HashSet::new())
    }
}

const sub_grid_size: usize = 3;
const grid_size: usize = sub_grid_size * sub_grid_size;

type Grid = [Cell; grid_size * grid_size];

#[inline]
fn pos(row: usize, col: usize) -> usize {
    row * grid_size + col
}

#[inline]
fn sub_pos(sub_row: usize, sub_col: usize, row: usize, col: usize) -> usize {
    (row + sub_row * sub_grid_size) * grid_size + col + (sub_col * sub_grid_size)
}

#[inline]
fn grid_cord(sub_row: usize, sub_col: usize, row: usize, col: usize) -> (usize, usize) {
    ((row + sub_row * sub_grid_size), (col + sub_col * sub_grid_size))
}

#[inline]
fn sub_grid_coord(row: usize, col: usize) -> (usize, usize) {
    ((row / sub_grid_size), (col / sub_grid_size))
}

fn print_grid_with_avail(grid: &Grid) {
    let mut avail_list = Vec::new();
    let mut longest_avail = 0;

    for row in 0..grid_size {
        for col in 0..grid_size {
            match &grid[pos(row, col)] {
                Cell::Static(_) => {}
                Cell::Avail(avail) => {
                    let mut list = String::new();
                    write!(&mut list, "{row}:{col} =");

                    for value in avail {
                        write!(&mut list, " {value}");
                    }

                    let count = list.chars().count();

                    if count > longest_avail {
                        longest_avail = count;
                    }

                    avail_list.push(list);
                }
            }
        }
    }

    let mut avail_iter = avail_list.into_iter();
    let avail_rows = 2;

    for row in 0..grid_size {
        if row == 0 {
            print!("  |");

            for index in 0..grid_size {
                print!(" ");

                if index != 0 && index % sub_grid_size == 0 {
                    print!("| ");
                }

                print!("{index}");
            }

            print!("  | ");

            for _ in 0..avail_rows {
                let Some(msg) = avail_iter.next() else {
                    break;
                };

                print!(" {msg:<longest_avail$} |");
            }

            println!("");
        }

        if row % sub_grid_size == 0 {
            print!("--+-");

            let v = "-".repeat(sub_grid_size * 2);

            for c in 0..3 {
                if c != 0 {
                    print!("+-");
                }

                print!("{v}");
            }

            print!("-| ");

            for _ in 0..avail_rows {
                let Some(msg) = avail_iter.next() else {
                    break;
                };

                print!(" {msg:<longest_avail$} |");
            }

            println!("");
        }

        print!("{row} |");

        for col in 0..grid_size {
            print!(" ");

            if col != 0 && col % sub_grid_size == 0 {
                print!("| ");
            }

            match &grid[pos(row, col)] {
                Cell::Static(value) => print!("{value}"),
                Cell::Avail(_) => print!(" ")
            }
        }

        print!("  | ");

        for _ in 0..avail_rows {
            let Some(msg) = avail_iter.next() else {
                break;
            };

            print!(" {msg:<longest_avail$} |");
        }

        println!("");
    }

    {
        print!("--+-");

        let v = "-".repeat(sub_grid_size * 2);

        for c in 0..3 {
            if c != 0 {
                print!("+-");
            }

            print!("{v}");
        }

        print!("-| ");

        for _ in 0..avail_rows {
            let Some(msg) = avail_iter.next() else {
                break;
            };

            print!(" {msg:<longest_avail$} |");
        }

        println!("");
    }

    let mut count = 0;

    while let Some(msg) = avail_iter.next() {
        if count == 0 {
            let spacer = " ".repeat(26);
            print!("{spacer} | ");
        }

        print!(" {msg:<longest_avail$} |");

        count = (count + 1) % avail_rows;

        if count == 0 {
            println!("");
        }
    }

    if count != 0 {
        println!("");
    }
}

fn main() {
    let input_file = PathBuf::from("./input.txt");
    let mut grid: [Cell; grid_size * grid_size] = std::array::from_fn(|_| Cell::default());
    let file = OpenOptions::new()
        .read(true)
        .open(&input_file)
        .expect("failed to open input file");
    let mut reader = std::io::BufReader::new(file);
    let mut line_num = 0;

    loop {
        line_num += 1;

        let mut line = String::new();

        let amount = match reader.read_line(&mut line) {
            Ok(read) => read,
            Err(err) => {
                panic!("failed to read data from file. {}:{line_num} {err}", input_file.display());
            }
        };

        if amount == 0 {
            break;
        }

        let Some((value, coord)) = line.trim().split_once(" ") else {
            panic!("invalid line input. no space between value and grid position. {}:{line_num} \"{line}\"", input_file.display());
        };

        let Some((row, col)) = coord.split_once(",") else {
            panic!("invalid line input. no comma for grid position. {}:{line_num} \"{line}\"", input_file.display());
        };

        let Ok(value): Result<u8, _> = value.parse() else {
            panic!("invalid cell value. cannot parse to u8. {}:{line_num} \"{line}\"", input_file.display());
        };
        let Ok(mut row): Result<usize, _> = row.parse() else {
            panic!("invalid row value. cannot parse to usize. {}:{line_num} \"{line}\"", input_file.display());
        };
        let Ok(mut col): Result<usize, _> = col.parse() else {
            panic!("invalid col value. cannot parse to usize. {}:{line_num} \"{line}\"", input_file.display());
        };

        if row == 0 {
            panic!("invalid row value. row value is 0. {}:{line_num} \"{line}\"", input_file.display());
        }

        if col == 0 {
            panic!("invalid col value. col value is 0. {}:{line_num} \"{line}\"", input_file.display());
        }

        row -= 1;
        col -= 1;

        let (sub_row, sub_col) = sub_grid_coord(row, col);
        let index = pos(row, col);

        println!("{row}:{col} = {value} sub grid: {sub_row}:{sub_col} index: {index}");

        grid[pos(row, col)] = Cell::Static(value);
    }

    let mut non_static = HashSet::new();

    // initialize

    for row in 0..grid_size {
        for col in 0..grid_size {
            match &mut grid[pos(row, col)] {
                Cell::Static(_) => {}
                Cell::Avail(avail) => {
                    non_static.insert((row, col));

                    for v in 1..=9 {
                        avail.insert(v);
                    }
                }
            }
        }
    }

    let mut to_change = Vec::new();
    // check state

    for row in 0..grid_size {
        for col in 0..grid_size {
            let (sub_row, sub_col) = sub_grid_coord(row, col);
            let index = pos(row,col);

            let checking = match grid[index] {
                Cell::Static(value) => value,
                Cell::Avail(_) => continue,
            };

            println!("checking {row}:{col} {checking} sub grid: {sub_row}:{sub_col} index: {index}");
            println!("checking row");

            // check entire row
            for check_row in 0..grid_size {
                let check_index = pos(check_row, col);

                if index == check_index {
                    continue;
                }

                print!("{check_row}:{col} index: {check_index}");

                match &mut grid[pos(check_row, col)] {
                    Cell::Static(value) => if *value == checking {
                        println!(" duplicate value found");
                        print_grid_with_avail(&grid);

                        panic!("halt");
                    } else {
                        println!("");
                    }
                    Cell::Avail(avail) => if avail.remove(&checking) {
                        let len = avail.len();

                        if len == 0 {
                            println!(" no more available options");
                            print_grid_with_avail(&grid);

                            panic!("halt");
                        } else if len == 1 {
                            println!(" only one option left");

                            to_change.push((check_row, col));
                        } else {
                            println!("");
                        }
                    } else {
                        println!("");
                    }
                }
            }

            println!("checking col");

            // check entire col
            for check_col in 0..grid_size {
                let check_index = pos(row, check_col);

                if check_index == index {
                    continue;
                }

                print!("{row}:{check_col} index: {check_index}");

                match &mut grid[check_index] {
                    Cell::Static(value) => if *value == checking {
                        println!(" duplicate value found");
                        print_grid_with_avail(&grid);

                        panic!("halt");
                    } else {
                        println!("");
                    }
                    Cell::Avail(avail) => if avail.remove(&checking) {
                        let len = avail.len();

                        if len == 0 {
                            println!(" no more available options");
                            print_grid_with_avail(&grid);

                            panic!("halt");
                        } else if len == 1 {
                            println!(" only one option left");

                            to_change.push((row, check_col));
                        } else {
                            println!("");
                        }
                    } else {
                        println!("");
                    }
                }
            }

            println!("checking sub grid");

            // check sub grid
            for check_sub_row in 0..sub_grid_size {
                for check_sub_col in 0..sub_grid_size {
                    let sub_index = sub_pos(sub_row, sub_col, check_sub_row, check_sub_col);

                    if sub_index == index {
                        continue;
                    }

                    print!("{check_sub_row}:{check_sub_col} {sub_index}");

                    match &mut grid[sub_index] {
                        Cell::Static(value) => if *value == checking {
                            println!(" duplicate value found in sub grid");
                            print_grid_with_avail(&grid);

                            panic!("halt");
                        } else {
                            println!("");
                        }
                        Cell::Avail(avail) => if avail.remove(&checking) {
                            let len = avail.len();

                            if len == 0 {
                                println!(" no more available options");
                                print_grid_with_avail(&grid);

                                panic!("halt");
                            } else if len == 1 {
                                println!(" only one option left");

                                to_change.push(grid_cord(sub_row, sub_col, check_sub_row, check_sub_col));
                            } else {
                                println!("");
                            }
                        } else {
                            println!("");
                        }
                    }
                }
            }
        }
    }

    let mut step = 0;

    loop {
        step += 1;

        {
            let msg = format!("step {step} ");

            println!("{msg:-<width$}", width = 80);
            print_grid_with_avail(&grid);
        }

        if to_change.is_empty() {
            break;
        }

        let mut next = Vec::new();

        for (row, col) in to_change {
            let index = pos(row, col);
            let (sub_row, sub_col) = sub_grid_coord(row, col);

            let value = match &grid[index] {
                Cell::Static(_) => {
                    println!("attempting to update static cell {row}:{col}");
                    print_grid_with_avail(&grid);

                    panic!("halt");
                }
                Cell::Avail(avail) => if avail.len() != 1 {
                    println!("mis-calculation for grid {row}:{col}");
                    print_grid_with_avail(&grid);

                    panic!("halt");
                } else {
                    *avail.iter()
                        .next()
                        .unwrap()
                }
            };

            println!("changing {row}:{col} -> {value}");

            grid[index] = Cell::Static(value);
            non_static.remove(&(row, col));

            // update row
            for update_row in 0..grid_size {
                let update_index = pos(update_row, col);

                if update_index == index {
                    continue;
                }

                match &mut grid[update_index] {
                    Cell::Static(_) => {}
                    Cell::Avail(avail) => if avail.remove(&value) {
                        let len = avail.len();

                        print!("updating {update_row}:{col}");

                        if len == 0 {
                            println!(" no more available options");
                            print_grid_with_avail(&grid);

                            panic!("halt");
                        } else if len == 1 {
                            println!(" only one option left");

                            next.push((update_row, col));
                        } else {
                            println!("");
                        }
                    }
                }
            }

            // update col
            for update_col in 0..grid_size {
                let update_index = pos(row, update_col);

                if update_index == index {
                    continue;
                }

                match &mut grid[update_index] {
                    Cell::Static(_) => {}
                    Cell::Avail(avail) => if avail.remove(&value) {
                        let len = avail.len();

                        print!("updating {row}:{update_col}");

                        if len == 0 {
                            println!(" no more available options");
                            print_grid_with_avail(&grid);

                            panic!("halt");
                        } else if len == 1 {
                            println!(" only one option left");

                            next.push((row, update_col));
                        } else {
                            println!("");
                        }
                    }
                }
            }

            // update sub grid
            for update_sub_row in 0..sub_grid_size {
                for update_sub_col in 0..sub_grid_size {
                    let update_index = sub_pos(sub_row, sub_col, update_sub_row, update_sub_col);

                    if update_index == index {
                        continue;
                    }

                    match &mut grid[update_index] {
                        Cell::Static(_) => {}
                        Cell::Avail(avail) => if avail.remove(&value) {
                            let len = avail.len();

                            print!("updating sub grid {sub_row}:{sub_col} {update_sub_row}:{update_sub_col}");

                            if len == 0 {
                                println!(" no more available options");
                                print_grid_with_avail(&grid);

                                panic!("halt");
                            } else if len == 1 {
                                println!(" only one option left");

                                next.push(grid_cord(sub_row, sub_col, update_sub_row, update_sub_col));
                            } else {
                                println!("");
                            }
                        }
                    }
                }
            }
        }

        if next.is_empty() {
            println!("checking subgrids");

            let mut grids_checked = [false; sub_grid_size * sub_grid_size];

            for (row, col) in &non_static {
                let (sub_row, sub_col) = sub_grid_coord(*row, *col);
                let grids_checked_index = sub_row * sub_grid_size + sub_col;

                if grids_checked[grids_checked_index] {
                    continue;
                } else {
                    grids_checked[grids_checked_index] = true;
                }

                let index = pos(*row, *col);

                let mut unique: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

                for check_sub_row in 0..sub_grid_size {
                    for check_sub_col in 0..sub_grid_size {
                        let check_index = sub_pos(sub_row, sub_col, check_sub_row, check_sub_col);
                        let check_coord = grid_cord(sub_row, sub_col, check_sub_row, check_sub_col);

                        match &grid[check_index] {
                            Cell::Static(_) => {}
                            Cell::Avail(avail) => {
                                for value in avail {
                                    unique.entry(*value)
                                        .or_default()
                                        .push(check_coord);
                                }
                            }
                        }
                    }
                }

                for (value, mut coords) in unique {
                    if coords.len() == 1 {
                        let (update_row, update_col) = coords.pop().unwrap();
                        let new_set = HashSet::from([value]);

                        grid[pos(update_row, update_col)] = Cell::Avail(new_set);

                        println!("adding {update_row}:{update_col} {value}");
                        next.push((update_row, update_col));
                    }
                }
            }
        }

        to_change = next;
    }
}
