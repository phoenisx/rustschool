#![allow(dead_code)]

use rand;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::{thread, time};

// fn some_expensive_func(intensity: u8, random: u8) -> (u8, u8) {
//     println!("Intensity: {}, Random: {}", intensity, random);
//     thread::sleep(time::Duration::from_millis(1000));
//     (intensity, random)
// }

struct CachedFunc<'a, T>
where
    T: Fn(u8, u8) -> (u8, u8),
{
    call: T,
    args: HashMap<&'a str, u8>, // Values passed to Hashmap, becomes owned, if `Copy` trait is not present...
    value: Option<(u8, u8)>,
}

impl<'a, T> CachedFunc<'a, T>
where
    T: Fn(u8, u8) -> (u8, u8),
{
    pub fn new(call: T) -> CachedFunc<'a, T> {
        CachedFunc {
            call,
            args: HashMap::new(),
            value: None,
        }
    }

    pub fn value(&mut self, intensity: u8, random: u8) -> (u8, u8) {
        let prev_intensity = match self.args.get("intensity") {
            Some(val) => *val, // This will create a new copy of inner value, instead of passing an immutable reference
            None => intensity,
        };
        let prev_rand = match self.args.get("random") {
            Some(val) => *val,
            None => random,
        };
        self.args.insert("intensity", intensity);
        self.args.insert("random", random);

        match self.value {
            Some(v) => {
                let mut val = v;
                if prev_intensity != intensity || prev_rand != random {
                    val = (self.call)(intensity, random);
                    self.value = Some(v);
                    println!("After -> Intensity: {}, Random: {}", intensity, random);
                }
                val
            }
            None => {
                let v = (self.call)(intensity, random);
                self.value = Some(v);
                println!("After -> Intensity: {}, Random: {}", intensity, random);
                v
            }
        }
    }
}

fn generate_workout<T>(expensive: &mut CachedFunc<T>, intensity: u8, random: u8)
where
    T: Fn(u8, u8) -> (u8, u8),
{
    if intensity < 25 {
        println!(
            "Today, do {:?} pushups!",
            expensive.value(intensity, random)
        );
        println!("Next, do {:?} situps!", expensive.value(intensity, random));
    } else {
        if random == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {:?} minutes!",
                expensive.value(intensity, random)
            );
        }
    }
}

fn main() {
    let mut choice = String::new();
    let mut expensive = CachedFunc::new(|intensity, random| -> (u8, u8) {
        println!(
            "Non Cached Before -> Intensity: {}, Random: {}",
            intensity, random
        );
        thread::sleep(time::Duration::from_millis(1000));
        (intensity, random)
    });
    loop {
        choice.clear();
        let mut intensity = String::new();
        print!("\nIntensity: ");
        io::stdout()
            .flush()
            .expect("I Don't for this small example");

        let intensity = match io::stdin().read_line(&mut intensity) {
            Ok(_) => intensity.trim().parse().unwrap(), // By the time we come to Ok in `match`, intesity would already be set.
            Err(_) => 0,
        };

        // let random = rand::random::<u8>();
        let random = 123u8; // For testing Caching, keeping random as constant...

        generate_workout(&mut expensive, intensity, random);

        print!("Continue (y/n): ");
        io::stdout()
            .flush()
            .expect("I Don't for this small example");
        io::stdin()
            .read_line(&mut choice)
            .expect("Fuck it failed man!!");
        if let "n" = choice.trim() {
            break;
        }
    }
}
