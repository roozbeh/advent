use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Debug)]
enum State {
    Halt,
    Error,
    Wait,
    Run,
}

#[derive(Debug)]
pub struct Computer {
    pos: usize,
    rel_base: i128,
    program: Vec<i128>,

    state: State,
    inputs: VecDeque<i128>,
    outputs: VecDeque<i128>,
    extra_mem: HashMap<usize, i128>,
}

impl Computer {
    pub fn new(program: Vec<i128>) -> Computer {
        Computer {
            pos: 0,
            rel_base: 0,
            program: program,
            state: State::Run,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            extra_mem: HashMap::new(),
        }
    }

    pub fn from(program: &str) -> Computer {
        Computer::new(
            program
                .trim()
                .split(",")
                .map(|x| x.parse::<i128>().unwrap())
                .collect(),
        )
    }

    pub fn write_input(&mut self, input: i128) {
        self.inputs.push_back(input);
    }

    pub fn pop_output(&mut self) -> Option<i128> {
        self.outputs.pop_front()
    }

    pub fn has_halted(&self) -> bool {
        self.state == State::Halt
    }

    pub fn is_waiting(&self) -> bool {
        self.state == State::Wait
    }

    pub fn resume(&mut self) {
        self.state = State::Run;
        self.run();
    }

    pub fn run(&mut self) {
        loop {
            match self.state {
                State::Run => self.one_op(),
                State::Halt => break,
                State::Wait => break,
                State::Error => panic!("Error"),
            }
        }
    }

    fn one_op(&mut self) {
        println!("------");
        let op = self.read_mem(self.pos);
        println!("rel_base: {:?}, op: {}", self.rel_base, op);
        let opcode = op % 100;
        let params_mode = parse_mode(op / 100);
        match opcode {
            1 => self.add(params_mode),
            2 => self.mul(params_mode),
            3 => self.input(params_mode),
            4 => self.output(params_mode),
            5 => self.jump_if_true(params_mode),
            6 => self.jump_if_false(params_mode),
            7 => self.lt(params_mode),
            8 => self.eq(params_mode),
            9 => self.adjust_rel_base(params_mode),
            99 => self.state = State::Halt,
            v => {
                println!("Error, unexpected opcode: {:?}", v);
                self.state = State::Error;
            }
        }
    }

    fn read_mem(&mut self, offset: usize) -> i128 {
        if offset > self.program.len() {
            let entry = self.extra_mem.entry(offset).or_insert(0);
            return *entry;
        }
        self.program[offset]
    }

    fn store_mem(&mut self, offset: usize, value: i128) {
        if offset > self.program.len() {
            let entry = self.extra_mem.entry(offset).or_insert(0);
            *entry = value;
            return;
        }
        self.program[offset] = value;
    }

    fn load(&mut self, mode: isize, offset: usize) -> i128 {
        match mode {
            0 => {
                let rel_position = self.read_mem(self.pos + offset);
                self.read_mem(rel_position as usize)
            }
            1 => self.read_mem(self.pos + offset),
            2 => {
                let rel_offset = self.read_mem(self.pos + offset);
                let v = self.read_mem((self.rel_base + rel_offset) as usize);
                println!(
                    "pos {} offset {} rel_offset {} final_offset {} value {}",
                    self.pos,
                    offset,
                    rel_offset,
                    rel_offset + self.rel_base,
                    v
                );
                v
            }
            _ => {
                println!("Error, unexpected read mode: {:?}", mode);
                0
            }
        }
    }

    fn store(&mut self, mode: isize, offset: usize, value: i128) {
        match mode {
            0 => {
                let store_pos = self.read_mem(self.pos + offset) as usize;
                self.store_mem(store_pos, value);
                //let rel_position = self.read_mem(() as usize);
                //self.store_mem(self.pos as i128 + offset, value);
            }
            2 => {
                let relative_movement = self.read_mem(self.pos + offset);
                let rel_position = (self.rel_base + relative_movement) as usize;
                self.store_mem(rel_position, value);
            }
            _ => {
                println!("Error, unexpected store mode: {:?}", mode);
            }
        }
    }

    fn adjust_rel_base(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        self.rel_base += first_param;
        self.pos += 2;
    }

    fn add(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        let second_param = self.load(params_mode[1], 2);
        self.store(params_mode[2], 3, first_param + second_param);
        self.pos += 4;
    }

    fn mul(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        let second_param = self.load(params_mode[1], 2);
        self.store(params_mode[2], 3, first_param * second_param);
        self.pos += 4;
    }

    fn input(&mut self, params_mode: Vec<isize>) {
        println!("input mode {:?}", params_mode);
        match self.read_input() {
            None => self.state = State::Wait,
            Some(value) => {
                self.store(params_mode[0], 1, value);
                self.pos += 2;
            }
        }
    }

    fn read_input(&mut self) -> Option<i128> {
        self.inputs.pop_front()
    }

    fn output(&mut self, params_mode: Vec<isize>) {
        let out = self.load(params_mode[0], 1);
        self.outputs.push_back(out);
        println!("out {}", out);
        self.pos += 2;
    }

    fn jump_if_true(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        if first_param != 0 {
            let second_param = self.load(params_mode[1], 2);
            self.pos = second_param as usize;
        } else {
            self.pos += 3;
        }
    }

    fn jump_if_false(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        if first_param == 0 {
            let second_param = self.load(params_mode[1], 2);
            self.pos = second_param as usize;
        } else {
            self.pos += 3;
        }
    }

    fn lt(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        let second_param = self.load(params_mode[1], 2);
        if first_param < second_param {
            self.store(params_mode[2], 3, 1);
        } else {
            self.store(params_mode[2], 3, 0);
        }
        self.pos += 4
    }

    fn eq(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        let second_param = self.load(params_mode[1], 2);
        if first_param == second_param {
            self.store(params_mode[2], 3, 1);
        } else {
            self.store(params_mode[2], 3, 0);
        }
        self.pos += 4
    }
}

fn parse_mode(mode: i128) -> Vec<isize> {
    let mut v = Vec::new();
    v.push((mode % 10) as isize);
    v.push((mode / 10 % 10) as isize);
    v.push((mode / 100 % 10) as isize);
    v
}

// let first_param = self.load(params_mode[0], 1);
// let second_param = self.load(params_mode[1], 2);
// let store_pos = self.read_mem(self.pos + 3) as i128;
// if first_param == second_param {
//     self.store(params_mode[2], store_pos, 1);
// } else {
//     self.store(params_mode[2], store_pos, 0);
// }
// self.pos += 4
