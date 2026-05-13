use std::env;
use std::io::{self, Write};


fn main() -> io:: Result<()> { 

      let args = env::args_os().collect::<Vec<_>>();

      for i in 1..args.len() {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle.write(args[i].as_encoded_bytes())?;
            handle.write_all(b" ")?;
      }

      // to clear the terminal
      println!("");
      Ok(())
}
