mod bf;
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || !args[1].ends_with(".bf") {
        return Err("Usage: brainfuck <filename.bf>".into());
    } else {
        // Check if file exists
        let pathbuf = std::path::PathBuf::from(&args[1]);
        if !pathbuf.exists() {
            return Err("File does not exist".to_string())?;
        } else {
            let code = std::fs::read_to_string(&args[1]).unwrap();
            match bf::run(&code) {
                Ok(()) => todo!("Nice."),
                Err(_) => return Err("Compilation failed. Get f*cked.".into()),
            };
        }
    }
}