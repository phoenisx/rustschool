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

  pub fn is_prime(&self, num: u32) -> Result<bool, &str> {
    let prime_iter = self.primes.iter();
    let remaining: Vec<u32> = prime_iter
      .filter(|prime| num % **prime == 0)
      .cloned() // cloned is required to get cloned data from immutable iter
      .collect();
    if remaining.is_empty() {
      return Ok(true);
    }
    Err("Not Found")
  }

  pub fn find_prime_at(&mut self, position: u32) -> &u32 {
    // This method will become more efficient after each call.
    // as the primes get memoized inside this.primes
    let mut from_value = self.primes.get(self.primes.len() - 1).unwrap_or_else(|| &2u32) + 1;
    while self.primes.len() != (position + 1) as usize {
      println!("Prime: {}, Vec: {:?}", from_value, self.primes);
      if self.is_prime(from_value).is_ok() {
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
