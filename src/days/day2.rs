use std::{cell::RefCell};
mod util;

struct Game {
    id: u32,
    red: RefCell<Vec<i32>>,
    green: RefCell<Vec<i32>>,
    blue: RefCell<Vec<i32>>,
}

impl Game {
    pub fn create(string: &str) -> Game {
        let game_split = string.split(':');

        let game_id = game_split.clone().nth(0).expect("Failed nth");
        let game_id = game_id.replace("Game ", "").parse().expect("parse failed");

        let game = Game {
            id: game_id,
            red: RefCell::new(Vec::new()),
            blue: RefCell::new(Vec::new()),
            green: RefCell::new(Vec::new()),
        };

        let game_rounds = game_split.clone().nth(1).expect("Failed nth");
        let game_rounds = game_rounds.split(';');

        for round in game_rounds {
            let split_round = round.split(','); // the colours for the round
            for colour in split_round {
                let split_colour = colour.trim().split(' ');
                let colour_qty = split_colour.clone().nth(0).expect("no qty");

                let colour_qty: i32 = colour_qty.parse().expect("colour_qty parse error");

                let colour_name = split_colour.clone().nth(1).expect("no name");

                match colour_name {
                    "red" => game.red.borrow_mut().push(colour_qty),
                    "blue" => game.blue.borrow_mut().push(colour_qty),
                    "green" => game.green.borrow_mut().push(colour_qty),
                    _ => panic!("No colour :'("),
                }
            }
        }

        // sort the colours, descending
        game.red.borrow_mut().sort_by(|a, b| b.cmp(a));
        game.blue.borrow_mut().sort_by(|a, b| b.cmp(a));
        game.green.borrow_mut().sort_by(|a, b| b.cmp(a));
        game
    }
}

pub fn main() {
    const DAY: u8 = 2;
    const RED: i32 = 12;
    const BLUE: i32 = 14;
    const GREEN: i32 = 13;

    println!("\n\n\nAdvent of Code day {DAY}");

    let contents = util::get_file_contents("data/day2/data.txt");

    let lines = contents.lines();

    let games = lines.map(|x| {
        let game = Game::create(x);
        game
    });

    let valid_games = games.clone().filter(|game| {
        let valid_red: bool = game.red.borrow().clone().into_iter().all(|qty| qty <= RED);
        let valid_blue: bool = game
            .blue
            .borrow()
            .clone()
            .into_iter()
            .all(|qty| qty <= BLUE);
        let valid_green: bool = game
            .green
            .borrow()
            .clone()
            .into_iter()
            .all(|qty| qty <= GREEN);
        valid_blue && valid_red && valid_green
    });

    let sum_part1: u32 = valid_games.map(|game| game.id).sum();
    println!("Part 1: {sum_part1}");

    let products = games.clone().map(|game| {
        let &max_red = game.red.borrow().get(0).unwrap();
        let &max_green = game.green.borrow().get(0).unwrap();
        let &max_blue = game.blue.borrow().get(0).unwrap();

        let product = max_red * max_green * max_blue;

        product
    });

    let sum_part2: i32 = products.clone().sum();
    println!("Part 2: {sum_part2}");
}
