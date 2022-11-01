const REGISTERS_COUNT: usize = 8;
const REGISTERS_OFFSET: usize = 32768;

pub struct Machine {
    memory: Vec<u8>,
    register: [u16; REGISTERS_COUNT],
    stack: Vec<u16>,

    cp: usize,      // code pointer

    is_running: bool,
}

impl Machine {
    pub fn new(program: Vec<u8>) -> Self {
        Machine { 
            memory: program, 
            register: [0; REGISTERS_COUNT], 
            stack: Vec::new(),
            cp: 0,

            is_running: false,
        }
    }

    // -- main loop
    pub fn run(&mut self) {
        self.is_running = true;
        while self.is_running {
            let instruction = self.read_next();
            match instruction {
                 0 => self.halt(),
                 1 => self.set(),
                 2 => self.push(),
                 3 => self.pop(),
                 4 => self.eq(),
                 5 => self.gt(),
                 6 => self.jmp(),
                 7 => self.jt(),
                 8 => self.jf(),
                 9 => self.add(),
                10 => self.mult(),
                12 => self.and(),
                13 => self.or(),
                14 => self.not(),
                17 => self.call(),
                19 => self.out(),
                21 => self.noop(),
                _ =>
                    panic!("Unhandled instruction {}", instruction),
            }
        }
    }

    fn read_next(&mut self) -> u16 {
        let value = self.read_memory();
        self.cp += 1;
        value
    }

    fn read_value(&mut self) -> u16 {
        let value = self.read_next();
        if value < REGISTERS_OFFSET as u16 {
            value
        } else {
            let register_idx = value as usize - REGISTERS_OFFSET;
            self.register[register_idx]
        }
    }

    fn read_register_idx(&mut self) -> usize {
        let value = self.read_next() as usize;
        value - REGISTERS_OFFSET
    }

    fn read_memory(&self) -> u16 {
        let pos = self.cp << 1;
        self.memory[pos] as u16 | (self.memory[pos + 1] as u16) << 8
    }

    // -- operations 
    // 0: stop execution and terminate the program
    fn halt(&mut self) {
        self.is_running = false;
    }

    // 1:  set register <a> to the value of <b>
    fn set(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        self.register[a] = b;
    }

    // 2: push <a> onto the stack
    fn push(&mut self) {
        let a = self.read_value();
        self.stack.push(a);
    }

    // 3: remove the top element from the stack and write it into <a>; empty stack = error
    fn pop(&mut self) {
        let a = self.read_register_idx();
        self.register[a] = self.stack.pop().unwrap();        
    }

    // 4: set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
    fn eq(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = if b == c { 1 } else { 0 }
    }

    // 5: set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
    fn gt(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = if b > c { 1 } else { 0 }
    }

    // 6: jump to <a>
    fn jmp(&mut self) {
        let jmp_addr = self.read_next();
        self.cp = jmp_addr as usize;
    }

    // 7: if <a> is nonzero, jump to <b>
    fn jt(&mut self) {
        let a = self.read_value();
        let b = self.read_next();
        if a != 0 {
            self.cp = b as usize;
        }
    }

    // 8: if <a> is zero, jump to <b>
    fn jf(&mut self) {
        let a = self.read_value();
        let b = self.read_next();
        if a == 0 {
            self.cp = b as usize;
        }
    }

    // 9: assign into <a> the sum of <b> and <c> (modulo 32768)
    fn add(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = (b + c) % REGISTERS_OFFSET as u16;
    }

    // 10: store into <a> the product of <b> and <c> (modulo 32768)
    fn mult(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value() as usize;
        let c = self.read_value() as usize;
        self.register[a] = (b * c % REGISTERS_OFFSET) as u16;
    }

    // 12: stores into <a> the bitwise and of <b> and <c>
    fn and(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = b & c;
    }

    // 13: stores into <a> the bitwise or of <b> and <c>
    fn or(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = b | c;
    }

    // 14: stores 15-bit bitwise inverse of <b> in <a>
    fn not(&mut self) {
        let a = self.read_register_idx();
        // TODO: if high bit is erased
        let b = !self.read_value() << 1 >> 1;
        self.register[a] = b
    }

    // 17: write the address of the next instruction to the stack and jump to <a>
    fn call(&mut self) {
        // TODO: check the order
        let jmp_addr = self.read_value();
        self.stack.push(self.cp as u16);
        self.cp = jmp_addr as usize;
    }

    // 19: write the character represented by ascii code <a> to the terminal
    fn out(&mut self) {
        let arg = self.read_value() as u8 as char;
        print!("{}", arg);
    }

    // 21: no operation
    fn noop(&self) {
        // no op
    }

    // -- utils
}