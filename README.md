# Simple tail viewer for log files

## Purpose
I work on different systems where `tail` like command can be not available. Another reason 
is
my log file entries contain a timestamp in milliseconds since the epoch. The program converts
the information in a human readable format.

## Command line arguments
Surprisingly, but AI doesn't give any simple and powerful command line arguments processor.
The list of `clap`, `pico-args`, `lexopt`, `args`, and  `docopt` looks ridiculous.

Not a big deal, the simple tail uses an own arguments parsing module. 

Define the arguments first,
```Rust
let mut cli = CLI::new();
cli.opt("n", OptTyp::Num)?.description("Number lines")
    .opt("v", OptTyp::None)?.description("Version").opt("h", OptTyp::None)?;
```

You can process the arguments after,
```rust
let lns = match cli.get_opt("n") {
    Some(OptVal::Num(n)) => *n as usize,
    _ => 15usize
};
if cli.get_opt("v") == Some(&OptVal::Empty) {
    return Ok(println!("\nVersion {VERSION}"))
} else if cli.get_opt("h") == Some(&OptVal::Empty)  || cli.args().len()  != 1 {
    return Ok(println!("simtail [opts] <file path>\n{}", cli.get_description()?))
}
tail_of(&cli.args().first()?);
```

If you have arguments in a form like - *-Xname=value*, then you can define them 
using the code bellow
```rust
cli.opt("D", OptTyp::InStr)?.description("A definition as name=value");
// and then read their presences in the command line
let d_o = cli.get_opt("D");
if let Some(OptVal::Arr(d_o)) = d_o {
    for (i,d) in d_o.into_iter().enumerate() {
        println!("opt[{i}] {d:?}");
    }
}
```

## Like the arguments processor?
Until it isn't separated in a dedicated crate, you can simply include the processor in your project:
```Rust
mod cli;
use crate::cli::{CLI,OptTyp,OptVal};
```
and then use it for parsing command arguments as shown above.

## How to build

1. Obtain [RustBee](https://github.com/vernisaz/rust_bee) 
2. Check out [Simple Time](https://github.com/vernisaz/simtime) and build (unless  already did that)
3. Run *rb*