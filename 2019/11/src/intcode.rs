pub struct Intcode {
    values: Vec<i128>,
    inputs: Vec<i128>,
    pc: usize,
    rel_base: usize,
    done: bool,
}

impl Intcode {
    pub fn new(line: &str, inputs: Option<Vec<i128>>) -> Self {
        Self {
            values: line
                .split(",")
                .map(|i| i.parse::<i128>().unwrap())
                .collect(),
            inputs: match inputs {
                Some(i) => i,
                None => Vec::new(),
            },
            pc: 0,
            rel_base: 0,
            done: false,
        }
    }

    pub fn add_inputs(&mut self, inputs: Vec<i128>) {
        self.inputs.append(&mut inputs.to_owned())
    }

    fn get_param(&self, offset: usize) -> i128 {
        let mut access_mode = self.values[self.pc] / 10;
        for _ in 0..offset {
            access_mode /= 10;
        }
        let pos = self.pc + offset;
        // println!("pos {}, rel_base: {}", pos, self.rel_base);
        let pos = match access_mode % 10 {
            0 => self.values[pos] as usize,
            1 => pos,
            2 => (i128::try_from(self.rel_base).unwrap() + self.values[pos]) as usize,
            _ => panic!("Unknown access_mode {}", access_mode),
        };
        match pos {
            p if p >= self.values.len() => 0,
            _ => self.values[pos],
        }
    }

    fn set_output(&mut self, offset: usize, value: i128) {
        let mut access_mode = self.values[self.pc] / 10;
        for _ in 0..offset {
            access_mode /= 10;
        }
        let pos = self.pc + offset;
        /*
        println!(
            "len {}, pos {}, rel_base: {}",
            self.values.len(),
            pos,
            self.rel_base
        );
        */
        let pos = match access_mode % 10 {
            0 => self.values[pos] as usize,
            2 => (i128::try_from(self.rel_base).unwrap() + self.values[pos]) as usize,
            _ => panic!("Unknown access_mode for set_value {}", access_mode),
        };
        if pos >= self.values.len() {
            self.values.resize(pos + 1, 0)
        }
        self.values[pos] = value;
    }

    pub fn run(&mut self) -> Option<Vec<i128>> {
        let mut outputs: Vec<i128> = Vec::new();

        if self.done {
            return None;
        }

        loop {
            let opcode = self.values[self.pc] % 100;
            match opcode {
                1 => {
                    self.set_output(3, self.get_param(1) + self.get_param(2));
                }
                2 => self.set_output(3, self.get_param(1) * self.get_param(2)),
                3 => {
                    if self.inputs.len() == 0 {
                        break;
                    }
                    let result = self.inputs.remove(0);
                    self.set_output(1, result);
                }
                4 => {
                    outputs.push(self.get_param(1));
                }
                5 => match self.get_param(1) {
                    0 => self.pc += 3,
                    _ => self.pc = self.get_param(2) as usize,
                },
                6 => match self.get_param(1) {
                    0 => self.pc = self.get_param(2) as usize,
                    _ => self.pc += 3,
                },
                7 => {
                    self.set_output(
                        3,
                        match self.get_param(1) < self.get_param(2) {
                            true => 1,
                            false => 0,
                        },
                    );
                }
                8 => self.set_output(
                    3,
                    match self.get_param(1) == self.get_param(2) {
                        true => 1,
                        false => 0,
                    },
                ),
                9 => {
                    self.rel_base =
                        (i128::try_from(self.rel_base).unwrap() + self.get_param(1)) as usize;
                }
                99 => {
                    self.done = true;
                    break;
                }
                _ => panic!("Unknown opcode {}", opcode),
            };

            self.pc += match opcode {
                1 | 2 | 7 | 8 => 4,
                3 | 4 | 9 => 2,
                _ => 0,
            }
        }
        Some(outputs)
    }
}
