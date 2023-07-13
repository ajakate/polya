use std::env;

fn get_prime_factors_count(n: u64) -> u32 {
    let mut count = 0;
    let mut num = n;
    let mut i = 2;

    while i <= num {
        if num % i == 0 {
            count += 1;
            num /= i;
        } else {
            i += 1;
        }
    }

    count
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let number: u64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number provided.");
            return;
        }
    };

    let prime_factors_count = get_prime_factors_count(number);
    println!("{}", prime_factors_count);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factors_count() {
        assert_eq!(get_prime_factors_count(84), 4);
        assert_eq!(get_prime_factors_count(12), 3);
        assert_eq!(get_prime_factors_count(37), 1);
    }
}
