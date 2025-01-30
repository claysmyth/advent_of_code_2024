mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        println!("Usage: {} <day> <part>", args[0]);
        return;
    }

    let day = &args[1];
    let part = &args[2];

    let result = match (day.as_str(), part.as_str()) {
        ("1", "1") | ("01", "1") => solutions::day01::part_1(),
        ("1", "2") | ("01", "2") => solutions::day01::part_2(),
        ("2", "1") | ("02", "1") => solutions::day02::part_1(),
        ("3", "1") | ("03", "1") => solutions::day03::part_1(),
        ("3", "2") | ("03", "2") => solutions::day03::part_2(),
        ("4", "1") | ("04", "1") => solutions::day04::part_1(),
        ("5", "1") | ("05", "1") => solutions::day05::part_1(),
        ("5", "2") | ("05", "2") => solutions::day05::part_2(),
        _ => {
            println!("Invalid day ({}) or part ({})", day, part);
            return;
        }
    };

    match result {
        Ok(answer) => println!("Day {} Part {} result: {}", day, part, answer),
        Err(e) => println!("Error running Day {} Part {}: {}", day, part, e),
    }
}
