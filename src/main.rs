use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::path::PathBuf;

const SUB_GRID_SIZE: usize = 3;
const GRID_SIZE: usize = SUB_GRID_SIZE * SUB_GRID_SIZE;

type Grid = [Cell; GRID_SIZE * GRID_SIZE];

#[derive(Debug, Clone)]
struct StackState {
    grid: Grid,
    undecided: HashSet<(usize, usize)>,
    to_change: Vec<(usize, usize)>,
    step: usize,
}

#[derive(Debug, Clone)]
enum Cell {
    Static(u8),
    Avail(HashSet<u8>)
}

#[inline]
fn pos(row: usize, col: usize) -> usize {
    row * GRID_SIZE + col
}

#[inline]
fn sub_pos(sub_row: usize, sub_col: usize, row: usize, col: usize) -> usize {
    (row + sub_row * SUB_GRID_SIZE) * GRID_SIZE + col + (sub_col * SUB_GRID_SIZE)
}

#[inline]
fn grid_cord(sub_row: usize, sub_col: usize, row: usize, col: usize) -> (usize, usize) {
    ((row + sub_row * SUB_GRID_SIZE), (col + sub_col * SUB_GRID_SIZE))
}

#[inline]
fn sub_grid_coord(row: usize, col: usize) -> (usize, usize) {
    ((row / SUB_GRID_SIZE), (col / SUB_GRID_SIZE))
}

fn print_grid_with_avail(grid: &Grid) {
    let mut avail_list = Vec::new();
    let mut longest_avail = 0;

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            match &grid[pos(row, col)] {
                Cell::Static(_) => {}
                Cell::Avail(avail) => {
                    let mut list = String::new();
                    write!(&mut list, "{row}:{col} =").unwrap();

                    for value in avail {
                        write!(&mut list, " {value}").unwrap();
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

    for row in 0..GRID_SIZE {
        if row == 0 {
            print!("  |");

            for index in 0..GRID_SIZE {
                print!(" ");

                if index != 0 && index % SUB_GRID_SIZE == 0 {
                    print!("| ");
                }

                print!("{index}");
            }

            print!(" |");

            for _ in 0..avail_rows {
                let Some(msg) = avail_iter.next() else {
                    break;
                };

                print!(" {msg:<longest_avail$} |");
            }

            println!();
        }

        if row % SUB_GRID_SIZE == 0 {
            print!("--+-");

            let v = "-".repeat(SUB_GRID_SIZE * 2);

            for c in 0..3 {
                if c != 0 {
                    print!("+-");
                }

                print!("{v}");
            }

            print!("|");

            for _ in 0..avail_rows {
                let Some(msg) = avail_iter.next() else {
                    break;
                };

                print!(" {msg:<longest_avail$} |");
            }

            println!();
        }

        print!("{row} |");

        for col in 0..GRID_SIZE {
            print!(" ");

            if col != 0 && col % SUB_GRID_SIZE == 0 {
                print!("| ");
            }

            match &grid[pos(row, col)] {
                Cell::Static(value) => print!("{value}"),
                Cell::Avail(_) => print!(" ")
            }
        }

        print!(" |");

        for _ in 0..avail_rows {
            let Some(msg) = avail_iter.next() else {
                break;
            };

            print!(" {msg:<longest_avail$} |");
        }

        println!();
    }

    {
        print!("--+-");

        let v = "-".repeat(SUB_GRID_SIZE * 2);

        for c in 0..3 {
            if c != 0 {
                print!("+-");
            }

            print!("{v}");
        }

        print!("|");

        for _ in 0..avail_rows {
            let Some(msg) = avail_iter.next() else {
                break;
            };

            print!(" {msg:<longest_avail$} |");
        }

        println!();
    }

    let mut count = 0;

    for msg in avail_iter {
        if count == 0 {
            let spacer = " ".repeat(26);
            print!("{spacer}|");
        }

        print!(" {msg:<longest_avail$} |");

        count = (count + 1) % avail_rows;

        if count == 0 {
            println!();
        }
    }

    if count != 0 {
        println!();
    }
}

struct Options {
    verbose: bool,
    steps: bool,
}

fn main() {
    let mut undecided = HashSet::new();
    let mut to_change = Vec::new();
    let mut grid: [Cell; GRID_SIZE * GRID_SIZE] = std::array::from_fn(|index| {
        let row = index / GRID_SIZE;
        let col = index % GRID_SIZE;

        undecided.insert((row, col));

        Cell::Avail(HashSet::from([1,2,3,4,5,6,7,8,9]))
    });

    let mut args = std::env::args();
    let mut maybe_input_file = None;
    let mut options = Options {
        verbose: false,
        steps: false,
    };

    loop {
        let Some(arg) = args.next() else {
            break;
        };

        if arg.eq("-v") || arg.eq("--verbose") {
            options.verbose = true;
        } else if arg.eq("--steps") {
            options.steps = true;
        } else {
            maybe_input_file = Some(PathBuf::from(arg));
        }
    }

    let Some(input_file) = maybe_input_file else {
        println!("no input file provided");
        return;
    };

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

        let Some((value, coord)) = line.trim().split_once(' ') else {
            panic!("invalid line input. no space between value and grid position. {}:{line_num} \"{line}\"", input_file.display());
        };

        let Some((row, col)) = coord.split_once(',') else {
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

        if value == 0 || value > 9 {
            panic!("invalid cell value. value is 0 or greater than 9. {}:{line_num} \"{line}\"", input_file.display());
        }

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

        if options.verbose {
            println!("{row}:{col} = {value} sub grid: {sub_row}:{sub_col} index: {index}");
        }

        grid[pos(row, col)] = Cell::Avail(HashSet::from([value]));
        to_change.push((row, col));
    }

    let mut stack = Vec::new();
    stack.push(StackState {
        grid,
        undecided,
        to_change,
        step: 0
    });

    let mut sudokus = Vec::new();

    let start = std::time::Instant::now();

    while let Some(state) = stack.pop() {
        let Some(state) = process_state(state, &options) else {
            continue;
        };

        if state.undecided.is_empty() {
            sudokus.push(state.grid);

            continue;
        }

        let mut smallest = usize::MAX;
        let mut smallest_coord = (0, 0);

        for (row, col) in &state.undecided {
            match &state.grid[pos(*row, *col)] {
                Cell::Static(_) => {}
                Cell::Avail(avail) => if avail.len() < smallest {
                    smallest = avail.len();
                    smallest_coord = (*row, *col);
                }
            }
        }

        if options.verbose {
            let msg = "choosing cells ".to_owned();
            println!("{msg:%<width$}", width = 80);
        }

        let (row, col) = smallest_coord;
        let index = pos(row, col);

        match &state.grid[index] {
            Cell::Static(_) => {
                if options.verbose {
                    println!("static cell?");
                }
            }
            Cell::Avail(avail) => {
                for value in avail {
                    if options.verbose {
                        println!("choosing {row}:{col} -> {value}");
                    }

                    let mut cloned = state.clone();
                    cloned.grid[index] = Cell::Avail(HashSet::from([*value]));
                    cloned.to_change.push((row, col));

                    stack.push(cloned);
                }
            }
        }
    }

    let duration = start.elapsed();

    if !sudokus.is_empty() {
        let mut first = true;
        let full = "#".repeat(80);
        let msg = " !!SUDOKU!! ".to_owned();
        println!("{full}\n{msg:#^width$}\n{full}", width = 80);

        for grid in sudokus {
            if first {
                first = false;
            } else {
                println!("{full}");
            }

            print_grid_with_avail(&grid);
        }
    } else {
        println!("no solutions found");
    }

    println!("time: {duration:#?}");
}

fn process_state(state: StackState, options: &Options) -> Option<StackState> {
    let StackState {mut grid, mut undecided, mut to_change, mut step} = state;

    loop {
        step += 1;

        if options.verbose || options.steps {
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

            let checking = match &grid[index] {
                Cell::Static(_) => {
                    if options.verbose {
                        println!("attempting to update static cell {row}:{col}");
                        print_grid_with_avail(&grid);
                    }

                    return None;
                }
                Cell::Avail(avail) => if avail.len() != 1 {
                    if options.verbose {
                        println!("mis-calculation for grid {row}:{col}");
                        print_grid_with_avail(&grid);
                    }

                    return None;
                } else {
                    *avail.iter()
                        .next()
                        .unwrap()
                }
            };

            if options.verbose {
                println!("changing {row}:{col} -> {checking}");
            }

            grid[index] = Cell::Static(checking);
            undecided.remove(&(row, col));

            // update row
            for update_row in 0..GRID_SIZE {
                let update_index = pos(update_row, col);

                if update_index == index {
                    continue;
                }

                match &mut grid[update_index] {
                    Cell::Static(value) => if *value == checking {
                        if options.verbose {
                            println!(" duplicate value found in row");
                            print_grid_with_avail(&grid);
                        }

                        return None;
                    }
                    Cell::Avail(avail) => if avail.remove(&checking) {
                        let len = avail.len();

                        if options.verbose {
                            print!("updating {update_row}:{col}");
                        }

                        if len == 0 {
                            if options.verbose {
                                println!(" no more available options");
                                print_grid_with_avail(&grid);
                            }

                            return None;
                        } else if len == 1 {
                            if options.verbose {
                                println!(" only one option left");
                            }

                            next.push((update_row, col));
                        } else {
                            if options.verbose {
                                println!();
                            }
                        }
                    }
                }
            }

            // update col
            for update_col in 0..GRID_SIZE {
                let update_index = pos(row, update_col);

                if update_index == index {
                    continue;
                }

                match &mut grid[update_index] {
                    Cell::Static(value) => if *value == checking {
                        if options.verbose {
                            println!(" duplicate value found in col");
                            print_grid_with_avail(&grid);
                        }

                        return None;
                    }
                    Cell::Avail(avail) => if avail.remove(&checking) {
                        let len = avail.len();

                        if options.verbose {
                            print!("updating {row}:{update_col}");
                        }

                        if len == 0 {
                            if options.verbose {
                                println!(" no more available options");
                                print_grid_with_avail(&grid);
                            }

                            return None;
                        } else if len == 1 {
                            if options.verbose {
                                println!(" only one option left");
                            }

                            next.push((row, update_col));
                        } else {
                            if options.verbose {
                                println!();
                            }
                        }
                    }
                }
            }

            // update sub grid
            for update_sub_row in 0..SUB_GRID_SIZE {
                for update_sub_col in 0..SUB_GRID_SIZE {
                    let update_index = sub_pos(sub_row, sub_col, update_sub_row, update_sub_col);

                    if update_index == index {
                        continue;
                    }

                    match &mut grid[update_index] {
                        Cell::Static(value) => if *value == checking {
                            if options.verbose {
                                println!(" duplicate value found in sub grid");
                                print_grid_with_avail(&grid);
                            }

                            return None;
                        }
                        Cell::Avail(avail) => if avail.remove(&checking) {
                            let len = avail.len();

                            if options.verbose {
                                print!("updating sub grid {sub_row}:{sub_col} {update_sub_row}:{update_sub_col}");
                            }

                            if len == 0 {
                                if options.verbose {
                                    println!(" no more available options");
                                    print_grid_with_avail(&grid);
                                }

                                return None;
                            } else if len == 1 {
                                if options.verbose {
                                    println!(" only one option left");
                                }

                                next.push(grid_cord(sub_row, sub_col, update_sub_row, update_sub_col));
                            } else {
                                if options.verbose {
                                    println!();
                                }
                            }
                        }
                    }
                }
            }
        }

        if next.is_empty() {
            if options.verbose {
                println!("checking subgrids");
            }

            let mut grids_checked = [false; SUB_GRID_SIZE * SUB_GRID_SIZE];

            for (row, col) in &undecided {
                let (sub_row, sub_col) = sub_grid_coord(*row, *col);
                let grids_checked_index = sub_row * SUB_GRID_SIZE + sub_col;

                if grids_checked[grids_checked_index] {
                    continue;
                } else {
                    grids_checked[grids_checked_index] = true;
                }

                let mut unique: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

                for check_sub_row in 0..SUB_GRID_SIZE {
                    for check_sub_col in 0..SUB_GRID_SIZE {
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

                        if options.verbose {
                            println!("adding {update_row}:{update_col} {value}");
                        }

                        next.push((update_row, update_col));
                    }
                }
            }
        }

        to_change = next;
    }

    Some(StackState {
        grid, undecided, to_change, step
    })
}
