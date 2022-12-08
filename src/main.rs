use std::fs;


struct Brainfuck<'a> {
    input: &'a str, // ">+."
    memory: Vec<u8>, // [71, 78, 85]
    cti: usize, // Current Token Index for `input`
    cvi: usize // Current Value Index (not index) for `memory`
}

impl<'a> Brainfuck<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            memory: vec![0],
            cti: 0,
            cvi: 0
        }
    }
    fn proc_token(&mut self) {
        if let Some(current_token) = self.input.chars().nth(self.cti) {
            let current_value = self.memory[self.cvi];
            
            match current_token {
                '>' => { if self.memory.len() <= self.cvi + 1 { self.memory.push(0) } self.cvi += 1 }
                '<' => if self.cvi == 0 { panic!("Negative memory index.") } else { self.cvi -= 1 },
                '+' => self.memory[self.cvi] = current_value.checked_add(1).unwrap_or(0),
                '-' => self.memory[self.cvi] = current_value.checked_sub(1).unwrap_or(0),
                '.' => println!("{}", current_value as char),
                _ => {}
            }
        }
        self.cti += 1;
    }
    fn interpret(mut self) {
        while self.input.len() > self.cti {
            self.proc_token();
        }
    }
}

fn main() {
    let file_content = fs::read_to_string("src/test.bf").expect("Cannot find file.");
    let file_content = file_content.as_str();
    let bf = Brainfuck::new(file_content);
    bf.interpret();
}