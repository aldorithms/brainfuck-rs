const MEMORY_MAP_SIZE: usize = 30000;

pub struct VM {
    pub memory: [u8; MEMORY_MAP_SIZE],
    pub pointer: usize,
}

impl VM {
    /// Creates a new virtual machine with the given program.
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_MAP_SIZE],
            pointer: 0
        }
    }

    /// Take input into cell
    pub fn input(&mut self) -> Result<(),Box<dyn std::error::Error + 'static>> {
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) =>         self.memory[self.pointer] = buf.as_bytes()[0],
            Err(_) => return Err("Failed to read input.".into()),
        }
        Ok(())
    }

    pub fn jmp_forward(&self, code: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if self.memory[self.pointer] == 0 {
            let mut depth = 1;
            let mut i = 0;
            for c in code.chars() {
                if c == '[' {
                    depth+=1;
                }
                else if c == ']' {
                    depth-=1;
                }
                if depth == 0 {
                    break;
                }
                i+=1;
            }
        }
        Ok(())
    }

    pub fn jmp_backward(&self, code: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if self.memory[self.pointer] != 0 {
            let mut depth = 1;
            let mut i = 0;
            for c in code.chars().rev() {
                if c == ']' {
                    depth+=1;
                }
                else if c == '[' {
                    depth-=1;
                }
                if depth == 0 {
                    break;
                }
                i+=1;
            }
        }
        Ok(())
    }
}