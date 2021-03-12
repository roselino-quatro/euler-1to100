use std::io;

fn main() {
    let mut limit = String::new();
    
    io::stdin()
        .read_line(&mut limit)
        .expect("Failed to read line");


    let limit: u32 = limit.trim().parse()
        .expect("Not a number");

    

    println!("Solution: {}", final_solution(limit-1));
}

// Initial solution iterating over every number between 1 to 999
fn first_solution(limit: u32) -> u32 {
    let mut result = 0;
    
    for n in 1..=limit {
        if n % 3 == 0 || n % 5 == 0 {
            result += n;
        }
    }

    result
}

// Two functions used on final solution, using a arithmetic series to find the result faster
fn divisile_sum(limit: u32, n: u32) -> u32 {
    let final_term = limit/n as u32;
    n*(final_term*(final_term+1))/2
}

fn final_solution(limit: u32) -> u32 {
    let result;
    result = divisile_sum(limit, 3)
           + divisile_sum(limit, 5)
           - divisile_sum(limit, 3*5);

    result
}
