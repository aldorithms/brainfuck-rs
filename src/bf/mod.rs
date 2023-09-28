pub mod vm;
use vm::*;

/// Run the Brainf*ck code.
pub fn run(code: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut vm = VM::new();
    for c in code.chars() {
        match c {
            '+' => vm.memory[vm.pointer]+=1,
            '-' => vm.memory[vm.pointer]-=1,
            '<' => vm.pointer-=1,
            '>' => vm.pointer+=1,
            '.' => print!("{}",vm.memory[vm.pointer]),
            ',' => match vm.input() {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.".into()),
            },
            '[' => match vm.jmp_forward(&code) {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.".into()),
            },
            ']' => match vm.jmp_backward(&code) {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.".into()),
            },
            _ => {/* Do Nothing, illegal characters are just comments */},
        }
    }
    Ok(())
}
