mod cli; // Declare the cli module (Rust looks for cli.rs)

use cli::Cli;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::io;

use std::string::String;

fn main() -> io::Result<()> {
    let args = Cli::parse();

    println!("Input file: {}", args.input);
    println!("Unrev? {}", args.unrev);

    let output_path = args.output.unwrap_or_else(|| {
        if args.unrev {
            "output_unrevved.bin".to_string()
        } else {
            "output.rev".to_string()
        }
    });

    // Run the transform (same logic both directions)
    let transformed = rev(args.input)?;

    // Write to file
    let mut output = File::create(&output_path)?;
    output.write_all(&transformed)?;

    println!("Written to: {}", output_path);
    Ok(())
}


fn rev(input: String) -> io::Result<Vec<u8>> {
   
    let mut input = File::open(input)?;

    let mut output_bin: Vec<u8> = Vec::new();
    let mut binary = Vec::new();

    input.read_to_end(&mut binary)?;



    for byte in &binary{
        
        let high = (byte >> 4) & 0x0F; // Upper nibble
        let low  = byte & 0x0F;        // Lower nibble

        let hi_rev = (high + 8) % 16;
        let low_rev = (low + 8) % 16;

        let transformed_byte = (hi_rev << 4) | low_rev;
        output_bin.push(transformed_byte);
    }

    

    Ok(output_bin) 
}



 