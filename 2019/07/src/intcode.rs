pub struct Intcode {
    values: Vec<isize>,
    inputs: Vec<isize>,
    pc: usize,
    done: bool,
}

impl Intcode {
    pub fn new(line: &str, inputs: Option<Vec<isize>>) -> Self {
        Self {
            values: line
                .split(",")
                .map(|i| i.parse::<isize>().unwrap())
                .collect(),
            inputs: match inputs {
                Some(i) => i,
                None => Vec::new(),
            },
            pc: 0,
            done: false,
        }
    }

    pub fn add_inputs(&mut self, inputs: Vec<isize>) {
        self.inputs.append(&mut inputs.to_owned())
    }

    fn get_param(&self, access_mode: isize, offset: usize) -> isize {
        let pos = self.pc + offset;
        match access_mode {
            1 => self.values[pos],
            0 => self.values[self.values[pos] as usize],
            _ => panic!("Unknown access_mode {}", access_mode),
        }
    }

    pub fn run(&mut self) -> Option<Vec<isize>> {
        let mut outputs: Vec<isize> = Vec::new();

        if self.done {
            return None;
        }

        loop {
            let opcode = self.values[self.pc] % 100;
            let access_mode = self.values[self.pc] / 100;
            match opcode {
                1 => {
                    let result_index = self.values[self.pc + 3] as usize;
                    self.values[result_index] = self.get_param(access_mode % 10, 1)
                        + self.get_param(access_mode / 10 % 10, 2);
                }
                2 => {
                    let result_index = self.values[self.pc + 3] as usize;
                    self.values[result_index] = self.get_param(access_mode % 10, 1)
                        * self.get_param(access_mode / 10 % 10, 2);
                }
                3 => {
                    let result_index = self.values[self.pc + 1] as usize;
                    if self.inputs.len() == 0 {
                        break;
                    }
                    self.values[result_index] = self.inputs.remove(0);
                }
                4 => {
                    outputs.push(self.get_param(access_mode % 10, 1));
                }
                5 => match self.get_param(access_mode % 10, 1) {
                    0 => self.pc += 3,
                    _ => self.pc = self.get_param(access_mode / 10 % 10, 2) as usize,
                },
                6 => match self.get_param(access_mode % 10, 1) {
                    0 => self.pc = self.get_param(access_mode / 10 % 10, 2) as usize,
                    _ => self.pc += 3,
                },
                7 => {
                    let result_index = self.values[self.pc + 3] as usize;
                    self.values[result_index] = match self.get_param(access_mode % 10, 1)
                        < self.get_param(access_mode / 10 % 10, 2)
                    {
                        true => 1,
                        false => 0,
                    };
                }
                8 => {
                    let result_index = self.values[self.pc + 3] as usize;
                    self.values[result_index] = match self.get_param(access_mode % 10, 1)
                        == self.get_param(access_mode / 10 % 10, 2)
                    {
                        true => 1,
                        false => 0,
                    };
                }
                99 => {
                    self.done = true;
                    break;
                }
                _ => panic!("Unknown opcode {}", opcode),
            };

            self.pc += match opcode {
                1 | 2 | 7 | 8 => 4,
                3 | 4 => 2,
                _ => 0,
            }
        }
        Some(outputs)
    }
}
