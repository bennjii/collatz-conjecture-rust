use std::io::{self, Write};

use std::time::{Instant};
use indicatif::{ProgressBar, ProgressStyle};
use std::convert::TryInto;

use console::style;

struct Solve {
    steps: u128,
    highest_number: u128,
    time: Instant
}

fn main() {
    print!("\n\tWhat Number do you wish to compute up to? ");
    let _ = io::stdout().flush();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u128>() {
        Ok(target_count) => {
            solve_collatz_set(target_count);
        },
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

fn solve_collatz_set(lim: u128) {
    let mut solves: Vec<Solve> = vec![];
    println!("\n\t{} {}", style("Attempting up to:").green().bold(), lim);

    let bar = ProgressBar::new(lim.try_into().unwrap());
    bar.set_style(ProgressStyle::default_bar()
        .template("\t[{elapsed_precise}] {bar:40.green/gray} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-"));

    for i in 1..lim {
        let solve: Solve = solve_collatz(i);
        solves.push(solve);
        bar.inc(1);
        // println!("\n\nSolved Sequence! \nSteps: \t\t{}\nHighest Number: {}\nSolve Time: \t{:?}", solve.steps, solve.highest_number, Instant::now().duration_since(solve.time))
    } 

    bar.finish();
    println!("\n\t{} Analysis:", style("Solved!").green().bold());

    analyse(solves);
}

fn analyse(solves: Vec<Solve>) {
    println!("");

    let mut total_steps = 0;
    let mut highest_number = 0;

    for (_i, x) in solves.iter().enumerate() {
        total_steps += x.steps;

        if(x.highest_number > highest_number) { highest_number = x.highest_number; }
        
    }

    println!("\tTotal Steps: \t\t{} ", total_steps);
    println!("\tHighest Number Reached: {} ", highest_number);
    println!("\n");
}

fn solve_collatz(n: u128) -> Solve {
    let now = Instant::now();

    let mut value = n;
    let mut steps = 0;
    let mut highest_number = 0;

    loop {
        steps += 1;

        let computed = collatz(value);
        if computed > highest_number {
            highest_number = computed;
        }

        value = computed;

        if computed == 1 {
            break;
        }
    }

    return Solve {
        steps: steps,
        highest_number: highest_number,
        time: now
    };
}

fn collatz(n: u128) -> u128 {
    if n % 2 == 0 {
        // Even
        return n / 2;
    }else {
        // Odd
        return (3 * n) + 1;
    }
}