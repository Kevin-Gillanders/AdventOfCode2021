use std::{env, fs};

use rand::prelude::*;

struct BingoBoard {
    board_data: Vec<Vec<i32>>,
    size_of_board: i32,
}

struct BingoGame {
    drawn_numbers: Vec<i32>,

    bingo_boards: Vec<BingoBoard>,
    amount_of_boards: usize,
}

impl BingoBoard {
    fn new(size: i32, board_range: i32) -> BingoBoard {
        // if(full_bingo_set.len() as i32) < (size * size){
        //     panic!("The provided set is not large enough to create a full bingo board")
        // }

        // println!("full bingo set : {:?}", full_bingo_set);
        // println!("full bingo set size : {:?}", full_bingo_set.len());
        // println!("===============================================");
        // let mut bingo_set = HashSet::new();
        let mut rng = rand::thread_rng();

        // while (bingo_set.len() as i32) < (size * size) {
        //     let idx: i32 = rng.gen();
        //     bingo_set.insert(full_bingo_set[(idx as usize)%full_bingo_set.len()]);
        // }

        // println!("sub bingo set : {:?}", bingo_set);
        // println!("sub bingo set size : {:?}", bingo_set.len());
        // println!("===============================================");
        let mut board: Vec<Vec<i32>> = vec![];

        let mut counter = 1;
        // println!("Going into board making loop");
        let mut row: Vec<i32> = vec![];

        for _item in 0..size * size {
            // println!("Loop {}\t Counter {}\tGoing into row push {} & {}", _item, counter, counter % (size), counter > 0);
            let val: i32 = rng.gen();
            row.push(val.abs() % board_range);
            if counter % (size) == 0 && counter > 0 {
                // println!("In loop with counter {}\t row is {} long\trow data {:?}", counter, row.len(), row);
                board.push(row);
                row = vec![];
            }
            counter = counter + 1;
        }

        BingoBoard {
            board_data: board,
            size_of_board: size,
        }
    }

    fn print(&self) {
        for row in &self.board_data {
            for cell in row {
                print!("{:?} ", cell);
            }
            println!();
        }
        println!("\n+++++++++++++");
    }

    fn new_from_vec(board: &Vec<Vec<i32>>) -> BingoBoard {
        BingoBoard {
            board_data: board.to_vec(),
            size_of_board: board[0].len() as i32,
        }
    }

    fn is_winning_board(&self, drawn_number: &[i32]) -> bool {
        if self.size_of_board > drawn_number.len() as i32 {
            return false;
        }
        return self.check_rows(drawn_number) || self.check_cols(drawn_number);
    }

    fn calc_winning_score(&self, drawn_number: &[i32]) -> i32 {
        let remaining: Vec<i32> = self.board_data.clone().into_iter().flatten().collect();

        let remaining_sum = remaining
            .into_iter()
            .filter(|num| !drawn_number.contains(num))
            .sum::<i32>();
        let last_num = drawn_number.last().expect("No elements in drawn numbers");
        let result = remaining_sum * last_num;

        println!("Result is {} * {} = {}", remaining_sum, last_num, result);

        return result;
    }

    fn check_line_numbers(&self, line_of_nums: &[i32], drawn_numbers: &[i32]) -> bool {
        let mut count_of_matches = 0;
        for number in drawn_numbers {
            if line_of_nums.contains(number) {
                count_of_matches = count_of_matches + 1;
            }
            if count_of_matches == line_of_nums.len() as i32 {
                return true;
            }
        }
        return false;
    }

    fn check_rows(&self, drawn_numbers: &[i32]) -> bool {
        for rows in &self.board_data {
            if self.check_line_numbers(&rows, drawn_numbers) {
                return true;
            }
        }
        return false;
    }

    fn check_cols(&self, drawn_numbers: &[i32]) -> bool {
        let transposed_data = transpose(self.board_data.clone());

        for rows in transposed_data {
            if self.check_line_numbers(&rows, drawn_numbers) {
                return true;
            }
        }

        return false;
    }
}

impl BingoGame {
    fn new_from_file(file_name: String) -> BingoGame {
        println!("Reading in {}", file_name);

        let file_content =
            fs::read_to_string(file_name).expect("There was an issue reading in the file");

        let mut numbers: Vec<i32> = vec![];
        let mut board: Vec<Vec<i32>> = vec![];
        let mut boards: Vec<BingoBoard> = vec![];
        let mut board_count = 0;
        let mut started_entering_data = false;

        for (idx, line) in file_content.split('\n').enumerate() {
            if idx == 0 {
                let number_line: Vec<&str> = line.split(',').map(|x| x.trim()).collect();
                numbers = number_line
                    .into_iter()
                    .map(|num| num.parse::<i32>().expect("Incorrect number"))
                    .collect();
                continue;
            }

            if line.trim().is_empty() {
                if !started_entering_data {
                    continue;
                }

                boards.push(BingoBoard::new_from_vec(&board));
                board_count = board_count + 1;
                board.clear();
                continue;
            }

            started_entering_data = true;

            let row: Vec<&str> = line
                .split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .collect();

            board.push(
                row.into_iter()
                    .map(|num| num.parse::<i32>().expect("Incorrect number"))
                    .collect(),
            );
        }

        BingoGame {
            drawn_numbers: numbers,
            bingo_boards: boards,
            amount_of_boards: board_count,
        }
    }

    fn print_game(&self) {
        println!("====================================");
        println!("{:?}", self.drawn_numbers);
        println!("====================================");

        for board in &self.bingo_boards {
            board.print();
        }
    }

    fn find_winning_board_part1(&self) -> i32 {
        for drawn_number in 0..self.drawn_numbers.len() {
            for board in &self.bingo_boards {
                if board.is_winning_board(&self.drawn_numbers[0..drawn_number]) {
                    // println!("found one");
                    // println!("{:?}", &self.drawn_numbers[0..drawn_number]);
                    // board.print();
                    return board.calc_winning_score(&self.drawn_numbers[0..drawn_number]);
                }
            }
        }
        return -10000;
    }

    fn find_winning_board_part2(&self) -> i32 {
        let mut win_count = 0;
        let mut has_won: Vec<bool> = Vec::with_capacity(self.amount_of_boards);
        for _i in 0..self.amount_of_boards {
            has_won.push(false);
        }

        for drawn_number in 0..self.drawn_numbers.len() {
            for (idx, board) in self.bingo_boards.iter().enumerate() {
                if board.is_winning_board(&self.drawn_numbers[0..drawn_number])
                    && has_won[idx] == false
                {
                    // println!("found {}", win_count);
                    // println!("{:?}", &self.drawn_numbers[0..drawn_number]);
                    // board.print();
                    // println!(
                    //     "its score {}",
                    //     board.calc_winning_score(&self.drawn_numbers[0..drawn_number])
                    // );
                    // println!("its position {}", idx);
                    // println!("========================");
                    has_won[idx] = true;
                    win_count = win_count + 1;
                    if win_count == self.amount_of_boards {
                        println!("found winner");
                        println!("{:?}", &self.drawn_numbers[0..drawn_number]);
                        board.print();
                        return board.calc_winning_score(&self.drawn_numbers[0..drawn_number]);
                    }
                }
            }
        }
        return -10000;
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("Advent of Code 2021 Day 4 - Rust!");

    // println!("{}", std::env::current_dir().unwrap().display());
    // let bingo_nums = [7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

    // let size = 5;

    // let bing = BingoBoard::new(size, 100);

    // bing.print();

    let game = BingoGame::new_from_file("./input.txt".to_string());

    println!("Part 1 answer : {}", game.find_winning_board_part1());
    println!("===============================");
    println!("Part 2 answer : {}", game.find_winning_board_part2());
    // game.print_game();
    // let mut test_set = HashSet::new();

    // test_set.insert(1);
    // test_set.insert(1);
    // test_set.insert(2);
    // test_set.insert(1);

    // for item in &test_set {
    //     println!("{item}");
    // }

    // let mut rng = rand::thread_rng();

    // let itest: u32 = rng.gen();
    // println!("{}",itest%3);
    // println!("{}", bingo_nums[26]);
}
