use rand::Rng;
use rand::rng;
use std::env;
use std::process;

fn parse_dice_arg(arg: &str) -> Option<(u32, u32)> {
    // Fix: avoid temporary value dropped while borrowed
    let arg_lc = arg.to_lowercase();
    let parts: Vec<&str> = arg_lc.split('d').collect();
    if parts.len() != 2 {
        return None;
    }
    let num = parts[0].parse::<u32>().ok()?;
    let sides = parts[1].parse::<u32>().ok()?;
    if num == 0 || sides == 0 {
        return None;
    }
    Some((num, sides))
}

fn roll_and_sum(num: u32, sides: u32, rng: &mut impl rand::Rng) -> (Vec<u32>, u32) {
    let rolls: Vec<u32> = (0..num).map(|_| rng.random_range(1..=sides)).collect();
    let total = rolls.iter().sum();
    (rolls, total)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut verbose = false;
    let mut dice_arg = None;
    let mut advantage = false;
    let mut disadvantage = false;

    for arg in &args[1..] {
        if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else if arg == "--adv" || arg == "-a" {
            advantage = true;
        } else if arg == "--dis" || arg == "-d" {
            disadvantage = true;
        } else if dice_arg.is_none() {
            dice_arg = Some(arg.clone());
        }
    }

    if advantage && disadvantage {
        eprintln!("Thats not a thing.");
        process::exit(1);
    }

    let dice_str = match dice_arg {
        Some(s) => s,
        None => {
            eprintln!("Usage: {} <NdM> [-v]", args[0]);
            process::exit(1);
        }
    };

    let (num, sides) = match parse_dice_arg(&dice_str) {
        Some(t) => t,
        None => {
            eprintln!("Invalid dice format. Use {{n}}d{{s}}, e.g., 2d6 or 1d20.");
            process::exit(1);
        }
    };

    let mut rng = rng();

    if advantage || disadvantage {
        if num != 2 {
            eprintln!("Incorrect syntax! Roll 2d20 for advantage/disadvantage.");
            process::exit(1);
        }
        let roll1 = rng.random_range(1..=sides);
        let roll2 = rng.random_range(1..=sides);
        if advantage {
            let result = roll1.max(roll2);
            // advantage result print as green
            println!("[{}, {}] = \x1b[32m{}\x1b[0m", roll1, roll2, result);
        } else {
            let result = roll1.min(roll2);
            // disadvantage result print as red
            println!("[{}, {}] = \x1b[31m{}\x1b[0m", roll1, roll2, result);
        }
    } else {
        let (rolls, total) = roll_and_sum(num, sides, &mut rng);
        if verbose {
            // sum result is printed blue
            println!("{:?} = \x1b[34m{}\x1b[0m", rolls, total);
        } else {
            println!("\x1b[34m{}\x1b[0m", total);
        }
    }
}


