const MEMORY_MAP_SIZE: usize = 30000;

pub struct VM {
    pub memory: [u8; MEMORY_MAP_SIZE],
    pub pointer: usize,
}

impl VM {
    /// Creates a new virtual machine with the given program.
    pub fn new() -> Result<VM, Box<dyn std::error::Error + 'static>> {
        Ok(VM {
            memory: [0; MEMORY_MAP_SIZE],
            pointer: 0
        })
    }

    fn check_if_overflow(&self) -> bool {
        if self.pointer >= MEMORY_MAP_SIZE {
            return true;
        } else {
            return false;
        }
    }

    pub fn increment(&mut self) -> Result<(),Box<dyn std::error::Error + 'static>> {
        if self.check_if_overflow() {
            Err("SEGMENTATION_FAULT.\n".into())
        } else {
            self.memory[self.pointer]+=1;
            Ok(())
        }
    }

    pub fn decrement(&mut self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if self.check_if_overflow() {
            Err("SEGMENTATION_FAULT.\n".into())
        } else {
            self.memory[self.pointer]-=1;
            Ok(())
        }
    }

    pub fn left(&mut self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if self.pointer == 0 {
            Err("SEGMENTATION_FAULT.\n".into())
        } else {
            self.pointer-=1;
            Ok(())
        }
    }

    pub fn right(&mut self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if self.pointer >= MEMORY_MAP_SIZE {
            Err("SEGMENTATION_FAULT.\n".into())
        } else {
            self.pointer+=1;
            Ok(())
        }
    }

    /// Take input into cell
    pub fn input(&mut self) -> Result<(),Box<dyn std::error::Error + 'static>> {
        if self.check_if_overflow() {
            Err("SEGMENTATION_FAULT.\n".into())
        } else {
            let mut buf = String::new();
            match std::io::stdin().read_line(&mut buf) {
                Ok(_) => self.memory[self.pointer] = buf.as_bytes()[0],
                Err(_) => return Err("Failed to read input.\n".into()),
            }
            Ok(())
        }
    }

    /// Jump Forward
    pub fn jmp_forward(&self, code: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if self.check_if_overflow() {
            Err("SEGMENTATION_FAULT\n".into())
        } else {
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
    }

    //Jump Backward
    pub fn jmp_backward(&self, code: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if self.check_if_overflow() {
            Err("SEGMENTATION_FAULT\n".into())
        } else {
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
}