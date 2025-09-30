# Simple tail viewer for log files

## Purpose
I work on different systems where `tail` like command can be not availble. Another reason 
is
my log file entries contain a timestamp in milliseconds since the epoch. The program converts
the information in a human readable format.

## Command line arguments
Surprisingly, but AI doesn't give any simple and powerful cammand line arguments processor.
The list of `clap`, `pico-args`, `lexopt`, `args`, and  `docopt` looks ridiculous.

Not a big deal, the simple tail uses own arguments parsing module. 

Define the arguments first,
```Rust
let mut cli = CLI::new();
cli.opt("n", OptTyp::Num).description("Number lines")
    .opt("v", OptTyp::None).description("Version").opt("h", OptTyp::None);
```

You can process the arguments now,
```rust
let lns = match cli.get_opt("n") {
    Some(OptVal::Num(n)) => *n as usize,
    _ => 15usize
};
if cli.get_opt("v") == Some(&OptVal::Empty) {
    return Ok(println!("\nVersion {VERSION}"))
} else if cli.get_opt("h") == Some(&OptVal::Empty)  || cli.args().len()  != 1 {
    return Ok(println!("simtail [opts] <file path>\n{}", cli.get_description().unwrap()))
}
tail_of(&cli.args()[0]);
```

## How to build

1. Obtain [RustBee](https://github.com/vernisaz/rust_bee) 
2. Check out [Simple Time](https://github.com/vernisaz/simtime) and build (unless  already did that)
3. Run *rb*