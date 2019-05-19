use std::env;

mod error;

pub use error::{Error, ErrorKind};

#[derive(Debug)]
pub struct Arguments {
  query: String,
}

impl Arguments {
  pub fn new(mut args: env::Args) -> error::Result<Self> {
    let arg = Arguments::parse(&mut args)?;
    Ok(arg)
  }

  fn parse(args: &mut env::Args) -> error::Result<Arguments> {
    match Arguments::validate_length(args) {
      Ok(()) => {
        let query = args.next().unwrap(); // Second Argument, i.e., querystring
        Ok(Arguments { query })
      }
      Err(e) => Err(e),
    }
  }

  fn validate_length(args: &mut env::Args) -> error::Result<()> {
    if args.len() == 1 {
      return Err(Error::new(ErrorKind::NoArgs));
    }

    if args.len() < 3 {
      args.next(); // Remove first option, as it's just the filename
      Ok(())
    } else {
      Err(Error::new(ErrorKind::TooMany(args.len())))
    }
  }
}
