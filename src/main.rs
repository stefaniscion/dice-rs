use clap::Parser;
use regex::Regex;
use rand::Rng;

/// Utility that rolls a dice
#[derive(Parser)]
#[command(author="Stefano Di Pasquale <stefaniscion@gmail.com>", version="1.0", about="Utility that rolls dices", long_about = None)]
struct Args {
    expression: String,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // find rolls in the expression
    let expression = &args.expression;
    let rolls = find_rolls(expression);
    let mut roll_totals: Vec<i32> = vec![];
    // roll parsed dices
    for roll in rolls {
        let results = roll_dice(roll);
        let dice_total: i32 = results.1.iter().sum();
        roll_totals.push(dice_total);
        if args.verbose {
            println!("{} -> {:?}, Total: {}", results.0, results.1, dice_total);
        }
    }
    // rewrite the expression with the results
    let replaced_expression = replace_rolls(expression, roll_totals);
    println!("{} = {}", expression, replaced_expression);
    // calculate the result
    // print the result
}

fn find_rolls<'a>(expression: &'a str) -> Vec<&'a str> {
    // find all the rolls in the expression and return them
    let re = Regex::new(r"\d*d\d+").unwrap();
    let mut rolls: Vec<&str> = Vec::new();
    for cap in re.captures_iter(expression) {
        rolls.push(cap.get(0).map_or("", |m| m.as_str()));
    } 
    rolls
}

fn roll_dice(expression: &str) -> (String, Vec<i32>) {
    // parse dice expression XdY
    let mut parts: Vec<&str> = expression.split("d").collect();    
    // if X is missing, set it to 1 (because 1dY is the same as dY)
    if parts[0] == "" {
        parts[0] = "1";
    }
    // convert X and Y to i32
    let num_rolls: i32 = parts[0].parse().expect("Invalid number of rolls");
    let die_type: i32 = parts[1].parse().expect("Invalid die type");
    // simulate the rolls
    let mut rng = rand::thread_rng();
    let mut results = Vec::new();
    // roll "num_rolls" times
    for _ in 0..num_rolls {
        let roll = rng.gen_range(1..=die_type);
        results.push(roll);
    }
    (expression.to_string(), results)
}

fn replace_rolls(expression: &str, roll_totals: Vec<i32>) -> String {
    // replace the rolls in the expression with the results
    let re = Regex::new(r"\d*d\d+").unwrap();
    let mut replaced_expression = expression.to_string();
    for roll_total in roll_totals {
        replaced_expression = re.replace(&replaced_expression, roll_total.to_string().as_str()).to_string();
    }
    replaced_expression
}