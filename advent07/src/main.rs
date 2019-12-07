use permutohedron::LexicalPermutation;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn generate_permutations(data: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut permutations = Vec::new();
    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }
    permutations
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // part 1
    // let permutations = generate_permutations(&mut (0..5).collect());
    // part 2
    let permutations = generate_permutations(&mut (5..10).collect());

    let mut outputs = Vec::new();
    for initial_inputs in permutations {
        // setup amplifiers
        let mut amplifiers: Vec<Amplifier> = vec![];
        for _ in 0..5 {
            amplifiers.push(Amplifier::from(&contents))
        }

        // initialize amplifiers with phases
        for (i, amplifier) in amplifiers.iter_mut().enumerate() {
            amplifier.write_input(initial_inputs[i]);
        }

        // previous output is the initial signal for this whole thing
        let mut previous_output: isize = 0;
        while !amplifiers[4].has_halted() {
            // orderly do one loop for each amplifier
            for amplifier in amplifiers.iter_mut() {
                // write the output of the previous amplifier as input for the current
                amplifier.write_input(previous_output);
                // whatever the previous state of the amplifier, restart
                if amplifier.is_waiting() {
                    amplifier.resume();
                } else {
                    amplifier.run();
                }
                // save output of the current one for the next loop
                previous_output = amplifier.pop_output();
            }
        }
        outputs.push((previous_output, initial_inputs));
    }
    let max = outputs.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();
    println!("max output: {:?} (initial inputs: {:?})", max.0, max.1);
    Ok(())
}

#[derive(Eq, PartialEq)]
enum State {
    Halt,
    Error,
    Wait,
    Run,
}

struct Amplifier {
    pos: usize,
    program: Vec<isize>,

    state: State,
    inputs: VecDeque<isize>,
    outputs: VecDeque<isize>,
}

impl Amplifier {
    fn new(program: Vec<isize>) -> Amplifier {
        Amplifier {
            pos: 0,
            program: program,
            state: State::Run,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }

    fn from(program: &str) -> Amplifier {
        Amplifier::new(
            program
                .trim()
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect(),
        )
    }

    fn write_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }

    fn read_input(&mut self) -> Option<isize> {
        self.inputs.pop_front()
    }

    fn pop_output(&mut self) -> isize {
        self.outputs.pop_front().unwrap()
    }

    fn has_halted(&self) -> bool {
        self.state == State::Halt
    }

    fn is_waiting(&self) -> bool {
        self.state == State::Wait
    }

    fn load(&self, mode: isize, param: isize) -> isize {
        match mode {
            0 => self.program[param as usize],
            1 => param,
            _ => {
                println!("Error, unexpected mode: {:?}", mode);
                0
            }
        }
    }

    fn add(&mut self, params_mode: Vec<isize>) {
        let a = self.program[self.pos + 1];
        let b = self.program[self.pos + 2];
        let store_pos = self.program[self.pos + 3] as usize;
        let first_param = self.load(params_mode[0], a);
        let second_param = self.load(params_mode[1], b);
        self.program[store_pos] = first_param + second_param;
    }

    fn mul(&mut self, params_mode: Vec<isize>) {
        let a = self.program[self.pos + 1];
        let b = self.program[self.pos + 2];
        let store_pos = self.program[self.pos + 3] as usize;
        let first_param = self.load(params_mode[0], a);
        let second_param = self.load(params_mode[1], b);
        self.program[store_pos] = first_param * second_param;
    }

    fn input(&mut self) -> State {
        let store_pos = self.program[self.pos + 1] as usize;
        match self.read_input() {
            None => State::Wait,
            Some(value) => {
                self.program[store_pos] = value;
                State::Run
            }
        }
    }

    fn output(&mut self, params_mode: Vec<isize>) {
        let a = self.program[self.pos + 1];
        let out = self.load(params_mode[0], a);
        self.outputs.push_back(out)
    }

    fn jump_if_true(&mut self, params_mode: Vec<isize>) -> bool {
        let a = self.program[self.pos + 1];
        let first_param = self.load(params_mode[0], a);
        if first_param != 0 {
            let b = self.program[self.pos + 2];
            let second_param = self.load(params_mode[1], b);
            self.pos = second_param as usize;
            true
        } else {
            false
        }
    }

    fn jump_if_false(&mut self, params_mode: Vec<isize>) -> bool {
        let a = self.program[self.pos + 1];
        let first_param = self.load(params_mode[0], a);
        if first_param == 0 {
            let b = self.program[self.pos + 2];
            let second_param = self.load(params_mode[1], b);
            self.pos = second_param as usize;
            true
        } else {
            false
        }
    }

    fn lt(&mut self, params_mode: Vec<isize>) {
        let a = self.program[self.pos + 1];
        let b = self.program[self.pos + 2];
        let store_pos = self.program[self.pos + 3] as usize;
        let first_param = self.load(params_mode[0], a);
        let second_param = self.load(params_mode[1], b);
        if first_param < second_param {
            self.program[store_pos] = 1
        } else {
            self.program[store_pos] = 0
        }
    }

    fn eq(&mut self, params_mode: Vec<isize>) {
        let a = self.program[self.pos + 1];
        let b = self.program[self.pos + 2];
        let store_pos = self.program[self.pos + 3] as usize;
        let first_param = self.load(params_mode[0], a);
        let second_param = self.load(params_mode[1], b);
        if first_param == second_param {
            self.program[store_pos] = 1
        } else {
            self.program[store_pos] = 0
        }
    }

    fn run(&mut self) {
        loop {
            match self.state {
                State::Run => self.one_op(),
                State::Halt => break,
                State::Wait => break,
                State::Error => panic!("Error"),
            }
        }
    }

    fn resume(&mut self) {
        self.state = State::Run;
        self.run();
    }

    fn one_op(&mut self) {
        let opcode = self.program[self.pos] % 100;
        let params_mode = parse_mode(self.program[self.pos] / 100);
        match opcode {
            1 => {
                self.add(params_mode);
                self.pos += 4
            }
            2 => {
                self.mul(params_mode);
                self.pos += 4
            }
            3 => {
                self.state = self.input();
                if self.state == State::Run {
                    self.pos += 2
                }
            }
            4 => {
                self.output(params_mode);
                self.pos += 2
            }
            5 => {
                if !self.jump_if_true(params_mode) {
                    self.pos += 3
                }
            }
            6 => {
                if !self.jump_if_false(params_mode) {
                    self.pos += 3
                }
            }
            7 => {
                self.lt(params_mode);
                self.pos += 4
            }
            8 => {
                self.eq(params_mode);
                self.pos += 4
            }

            99 => {
                self.state = State::Halt;
            }
            v => {
                println!("Error, unexpected opcode: {:?}", v);
                self.state = State::Error;
            }
        }
    }
}

fn parse_mode(mode: isize) -> Vec<isize> {
    let mut v = Vec::new();
    v.push(mode % 10);
    v.push(mode / 10 % 10);
    v.push(mode / 100 % 10);
    v
}
