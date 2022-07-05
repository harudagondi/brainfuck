pub fn run(input: &str, program: &str) -> String {
    let mut memory = [0u8; u16::MAX as usize + 1];
    let mut cursor = 0u16;
    let mut loc = 0usize;
    let mut input: Vec<u8> = input.bytes().rev().collect();
    let mut output = String::new();
    let mut bracket_locs = Vec::new();

    while loc < program.len() {
        let c = cursor as usize;
        match program.chars().nth(loc).unwrap() {
            '+' => memory[c] = memory[c].wrapping_add(1),
            '-' => memory[c] = memory[c].wrapping_sub(1),
            '>' => cursor = cursor.wrapping_add(1),
            '<' => cursor = cursor.wrapping_sub(1),
            '.' => output.push(memory[c] as char),
            ',' => memory[c] = input.pop().unwrap_or(b'\0') as u8,
            '[' => bracket_locs.push(loc),
            ']' => {
                if memory[c] == 0 {
                    bracket_locs.pop();
                } else {
                    loc = *bracket_locs.iter().last().unwrap();
                }
            }
            _ => {}
        }
        loc += 1;
    }
    output
}

fn main() {
    use seahorse::{App, Flag, FlagType};
    use std::fs::File;
    use std::io::Read;

    let args: Vec<String> = std::env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("bf [args]")
        .flag(
            Flag::new("input", FlagType::String)
                .description("Input for the program")
                .alias("i"),
        )
        .action(|c| {
            let mut f = File::open(&c.args[0]).expect("File not found.");
            let mut program = String::new();
            Read::read_to_string(&mut f, &mut program).expect("Cannot read file.");
            let output = run(&c.string_flag("input").unwrap_or_default(), &program);
            println!("{output}");
        });

    app.run(args)
}
