#![allow(dead_code)]
#![allow(unused_variables)]

struct PrimeTime {
    primes: Vec<u32>,
}

impl PrimeTime {
    pub fn new(size: usize) -> Self {
        let mut primes = Vec::with_capacity(size);
        primes.push(2);
        PrimeTime { primes }
    }

    /**
     * take_wile and filter together reduces the amount of loops run
     *
     * The logic behind `p * p <= n` is that, a number is already either factored out by some smaller prime
     * Or if it's a multiple of `p * non-prime` it's a not a prime anyways...
     *
     * For eg. 13 is a prime, and filtering it with 10 % 5 is usesless, as 10 % 2 === 0,
     * which rejects the number even before coming to check with prime number 5.
     *
     */
    pub fn is_prime(&self, num: u32) -> bool {
        self.primes
            .iter()
            .take_while(|&prime| prime * prime <= num)
            .filter(|&prime| {
              num % prime == 0
            })
            .all(|&prime| false) // Empty iterator Returns true.
    }

    // Having prints in large loops, slows down the code...
    pub fn find_prime_at(&mut self, position: u32) -> &u32 {
        let mut from_value = self.primes.last().unwrap_or_else(|| &2u32) + 1;
        while self.primes.len() != (position + 1) as usize {
            if self.is_prime(from_value) {
                self.primes.push(from_value);
            }
            from_value += 1;
        }
        self.primes.get(position as usize).unwrap()
    }
}

pub fn nth(n: u32) -> u32 {
    *(PrimeTime::new(n as usize).find_prime_at(n))
}
