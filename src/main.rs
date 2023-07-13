use std::env;

fn generate_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;

    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 2..=sqrt_n {
        if primes[i] {
            let mut j = i * i;
            while j <= n {
                primes[j] = false;
                j += i;
            }
        }
    }

    let prime_numbers: Vec<usize> = primes
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(num, _)| num)
        .collect();

    prime_numbers
}

fn get_prime_factors_count(n: usize, primes: &[usize]) -> u32 {
    let mut count = 0;
    let mut num = n;

    for &prime in primes {
        if prime * prime > num {
            break;
        }
        while num % prime == 0 {
            count += 1;
            num /= prime;
        }
    }

    if num > 1 {
        count += 1;
    }

    count
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let number: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number provided.");
            return;
        }
    };

    let primes = generate_primes(number);
    println!("primes generation done");

    let mut i = 1;
    let mut num_even = 0;
    let mut num_odd = 0;
    let mut found = false;

    while (i < number) && (found == false) {
        let prime_count_total = get_prime_factors_count(i, &primes);
        
        if prime_count_total % 2 == 0 {
            num_even += 1;
        } else {
            num_odd += 1;
        }
        
        if (i > 4) && (num_even > num_odd) {
            println!("WOW THIS IS IT! {}", i);
            found = true;
        }

        if i % 100000 == 0 {
            println!("{}", i);
        }
        i = i + 1;
    }

    println!("DONE");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factors_count() {
        let primes: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
        assert_eq!(get_prime_factors_count(2, &primes), 1);
        assert_eq!(get_prime_factors_count(3, &primes), 1);
        assert_eq!(get_prime_factors_count(4, &primes), 2);
        assert_eq!(get_prime_factors_count(84, &primes), 4);
        assert_eq!(get_prime_factors_count(12, &primes), 3);
        assert_eq!(get_prime_factors_count(37, &primes), 1);
    }

    #[test]
    fn test_generate_primes() {
        // Test for prime numbers up to 20
        let expected_primes_20: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let generated_primes_20 = generate_primes(20);
        assert_eq!(expected_primes_20, generated_primes_20);

        // Test for prime numbers up to 100
        let expected_primes_100: Vec<usize> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        ];
        let generated_primes_100 = generate_primes(100);
        assert_eq!(expected_primes_100, generated_primes_100);
    }
}
