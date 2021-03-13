fn main() {
    let mut current_number = 2;
    let mut previous_number = 0;
    let mut buffer;
    let mut total_sum = 0;

    while current_number < 4000000 {
        total_sum += current_number;
        buffer = previous_number;
        previous_number = current_number;
        current_number = even_fibonacci(current_number, buffer);
    }
    
    println!("{}", total_sum);
}

/* First Solution */

fn fibonacci(n: i32) -> i32 {
    if n <= 1  { 
        return 1; 
    }

    fibonacci(n-2) + fibonacci(n-1)
}

/* Final Solution */

fn even_fibonacci(even_current: i32, even_previous: i32) -> i32 {
    4*even_current + even_previous
}
