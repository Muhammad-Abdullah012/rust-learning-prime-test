fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    // we've only "as" keyword for typecasting, and there is no implicit typecasting
    for i in 2..f64::sqrt(n as f64) as i64 {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut number = String::new();
    let number = loop {
        println!("Please enter a number: ");
        number.clear();
        match std::io::stdin().read_line(&mut number) {
            Ok(_) => match number.trim().parse::<i64>() {
                Ok(n) => {
                    break n;
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            },
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    };
    if is_prime(number) {
        println!("{} is prime.", number);
    } else {
        println!("{} is not prime.", number);
    }
    println!("------------------------------------------------------------");
    println!("Table of {} is :", number);
    for i in 1..11 {
        match number.checked_mul(i) {
            Some(n) => {
                println!("{} x {} = {}", number, i, n);
            }
            None => {
                println!("{} x {} = overflow", number, i);
            }
        }
    }
}
