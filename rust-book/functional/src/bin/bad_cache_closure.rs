#![allow(dead_code)]

use rand;
use std::io;
use std::io::Write;
use std::{thread, time};

// fn some_expensive_func(intensity: u8, random: u8) -> (u8, u8) {
//     println!("Intensity: {}, Random: {}", intensity, random);
//     thread::sleep(time::Duration::from_millis(1000));
//     (intensity, random)
// }

struct CachedFunc<T>
where
    T: Fn(u8, u8) -> (u8, u8),
{
    call: T,
    value: Option<(u8, u8)>,
}

impl<T> CachedFunc<T>
where
    T: Fn(u8, u8) -> (u8, u8),
{
    pub fn new(call: T) -> CachedFunc<T> {
        CachedFunc { call, value: None }
    }

    pub fn value(&mut self, intensity: u8, random: u8) -> (u8, u8) {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.call)(intensity, random);
                self.value = Some(v);
                println!("After -> Intensity: {}, Random: {}", intensity, random);
                v
            }
        }
    }
}

fn generate_workout(intensity: u8, random: u8) {
    // Should be mutable instance, as value needs to be using `&mut self` for updating `self.value`
    let mut expensive = CachedFunc::new(|intensity, random| -> (u8, u8) {
        println!("Before -> Intensity: {}, Random: {}", intensity, random);
        thread::sleep(time::Duration::from_millis(1000));
        (intensity, random)
    });

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

        let random = rand::random::<u8>();

        generate_workout(intensity, random);

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
