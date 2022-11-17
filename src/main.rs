use reuler;
use std::env;
use std::process;
use std::time;

fn main() {
    // Inputs
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments. Usage : `reuler <problem_id>`");
        process::exit(1);
    }

    let problem_id: isize = args[1].trim().parse().unwrap_or_else(|_err| {
        println!(
            "The given argument is not an ID (`{}`). Please provide a number.",
            args[1]
        );
        process::exit(1);
    });

    // Solve the problem, timing the runtime
    let now = time::Instant::now();
    let res = reuler::solve(problem_id).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    let elapsed = now.elapsed();

    // Print the result
    println!("Solution for problem #{} : {}", problem_id, res);
    println!("Time taken : {elapsed:?}");
}
