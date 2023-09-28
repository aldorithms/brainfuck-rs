pub mod vm;
use vm::*;

/// Run the Brainf*ck code.
pub fn run(code: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut vm = match VM::new(){
        Ok(vm) => vm,
        Err(_) => return Err("Failed to form Virtual Memory. Compilation stopped.\n".into()),
    };
    for c in code.chars() {
        match c {
            '+' => match vm.increment() {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.\n".into()),
            },

            '-' => match vm.decrement() {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.\n".into()),
            },

            '<' => match vm.left() {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.\n".into()),
            },

            '>' => match vm.right() {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.\n".into()),
            },

            '.' => print!("{}", vm.memory[vm.pointer]),

            ',' => match vm.input() {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.\n".into()),
            },

            '[' => match vm.jmp_forward(&code) {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.\n".into()),
            },

            ']' => match vm.jmp_backward(&code) {
                Ok(()) => {},
                Err(_) => return Err("Compilation stopped.\n".into()),
            },

            _ => { /* Do Nothing, illegal characters are just comments */ },
        }
    }
    Ok(())
}
