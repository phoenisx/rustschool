struct Counter {
    count: u32,
    max_count: u32,
}

impl Counter {
    fn new(max_count: u32) -> Counter {
        Counter {
            count: 0,
            max_count,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < self.max_count {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(20);
    print!("[");
    for val in counter {
        print!("{}, ", val);
    }
    print!("]\n"); // Since this print contains \n explicitly, thus, everything will now get flushed to screen buffer...

    // Can I use `counter` again??
    // No, as Iterator has modified the `counter.count` to max, and thus, iterator will always return `None`
    // and anyways, counter becomes invalid here, compiler throws error, val is moved...

    let c2 = Counter::new(30);
    let evens: Vec<u32> = c2.filter(|x| x % 2 == 0).collect();
    println!("Evens -> {:?}", evens);
}
