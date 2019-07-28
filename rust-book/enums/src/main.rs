// Helps to hide dead code warnings for the whole file...
#![allow(dead_code)]

// IMPORTANT NOTE: Not showing any examples for https://doc.rust-lang.org/book/ch06-03-if-let.html, as it's all
// Syntactical Sugar... Depends on what user prefers to use. Match Expressions are more ellaborate compared to `if let` statements...

use std::io;

#[derive(Debug)]
struct MessageStruct {
  msg: String,
}

/**
 * Breakdown #1:
 *  - I was thinking Enum as C++ Enums, but it's not. It's more than a store of named integer values here,
 *      more like JAVA's Enums, much better than even that. Consider it as a Conditional Map Data Type,
 *      which changes it's behaviour respective to the type of instance created from Enum.
 *  - Just my observation till now, might be I am wrong here, but I think Enums are miniature CLASS.
 *  - Following is the way to create an Enum.
 *  - Figured out `'static` is some kind of Lifetime Speciifier, which we will learn later.
 */
#[derive(Debug)]
enum Message {
  Quit, // Enums can be Empty, Which represents null/Null/Nil in Rust, as Rust does not support `null` data type
  Move { x: i32, y: i32 }, // Anonymous Struct
  Write(String), // Accepts String Wrapper type
  Read(MessageStruct), // Accepts `MessageStruct` Custom Data Type
  Color(u8, u8, u8), // Accepts three arguments, could be anything, but here it's three primary data types.
}

/**
 * Breakdown #2:
 * This Code shows two most important details of Enum.
 * Also, usage of special Enum called Option (which is a part of Rust default prelude (already imported in every code, by default))
 *
 *  - Implement Methods in Rust...
 *  - Use Match Expresssions to figure out which type of enum to work with.
 *    -- Won't explain the details for match statement here.
 *       Check out - https://doc.rust-lang.org/book/ch06-02-match.html for details.
 *  - Return `Option<Message>` enum, which states that the method `lightenBy` can be a `Message` or `None` (nothing/null).
 */
impl Message {
  fn lighten_by(&self, amount: u8) -> Option<Message> {
    match self {
      Message::Color(red, green, blue) => {
        Some(Message::Color(red + amount, green + amount, blue + amount))
      }
      _ => None,
    }
  }
}

fn main() {
  loop {
    // Varible should be created in each loop, as if not (i.e., created before loop runs),
    // string gets appended in `read_line` io method...
    let mut choice = String::new();
    println!("Make a choice:");
    println!("\t1. Enum Simple Example");
    println!("\t2. Impl Methods to Enum and Use them...");
    println!("\t3. Access Enum using match expressions");
    io::stdin()
      .read_line(&mut choice)
      .expect("Failed in storing user input");

    match choice.trim() {
      "1" => {
        let write = Message::Write(String::from("WOWWW!!! It works...")); // Create an Enum Instance.
        let read = Message::Read(MessageStruct {
          msg: String::from("Can create struct instances directly"),
        });
        println!("Write: {:#?}, Read: {:#?}", write, read);
      }
      "2" => {
        let color = Message::Color(120, 200, 100);
        println!("Created Color: {:?}", color);
        println!("Lighten Color by 20: {:?}", color.lighten_by(20));
      }
      "3" => match Message::Color(120, 200, 100).lighten_by(30) {
        Some(message) => match message {
          // If params are unused, name them _{variable_name}, so that rust doesn't throw warnings
          Message::Color(red, _green, _blue) => println!("Lighten Color by 30 -> (RED): {}", red),
          _ => break,
        },
        None => println!("No Data Recieved Man!!!"),
      },
      _ => break,
    }
  }
}
