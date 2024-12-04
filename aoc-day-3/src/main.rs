use memory_diagnostics::MemoryParser;
use std::{
    env,
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut args = env::args_os();
    args.next(); // Skip program name
    let filename = match args.next() {
        Some(f) => f,
        None => {
            eprintln!("❄️ Error: No corrupted memory file provided!");
            eprintln!("Usage: program <memory-dump-path>");
            return Ok(());
        }
    };

    let mut file = File::open(filename)?;
    let mut parser = MemoryParser::default();
    let mut buffer = [0; 1024]; // Read corrupted memory in 1KB chunks

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for &byte in &buffer[..bytes_read] {
            let c = byte as char;
            parser.process_char(c);
        }
    }

    println!("\n🎅 North Pole Memory Analysis Complete! 🎄");
    println!("==========================================");
    println!("🔍 Corrupted Memory Scan Results:");
    println!("✨ Total Checksum: {}", parser.checksum());
    println!("==========================================\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shopkeeper_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let mut parser = MemoryParser::default();
        for c in input.chars() {
            parser.process_char(c);
        }
        assert_eq!(parser.checksum(), &161_u32);
    }

    #[test]
    fn test_conditional_memory() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        let mut parser = MemoryParser::default();
        for c in input.chars() {
            parser.process_char(c);
        }
        assert_eq!(parser.checksum(), &48_u32);
    }
}
