use std::vec;

#[derive(Debug, Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    p: usize,
    prog: Vec<u64>,
    output: Vec<u64>,
}

impl Computer {
    fn instruction(&self) -> u64 {
        self.prog[self.p]
    }

    fn literal_operand(&self) -> u64 {
        self.prog[self.p + 1]
    }

    fn combo_operand(&self) -> u64 {
        match self.prog[self.p + 1] {
            0..4 => self.prog[self.p + 1],
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand"),
        }
    }

    fn adv(&mut self) {
        self.a /= 2u64.pow(self.combo_operand() as u32);
        self.p += 2;
    }

    fn bxl(&mut self) {
        self.b ^= self.literal_operand();
        self.p += 2;
    }

    fn bst(&mut self) {
        self.b = self.combo_operand() % 8;
        self.p += 2;
    }

    fn jnz(&mut self) {
        if self.a == 0 {
            self.p += 2;
            return;
        }

        self.p = self.literal_operand() as usize;
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
        self.p += 2;
    }

    fn out(&mut self) {
        self.output.push(self.combo_operand() % 8);
        self.p += 2;
    }

    fn bdv(&mut self) {
        self.b = self.a / 2u64.pow(self.combo_operand() as u32);
        self.p += 2;
    }

    fn cdv(&mut self) {
        self.c = self.a / 2u64.pow(self.combo_operand() as u32);
        self.p += 2;
    }

    fn run(&mut self) {
        let max_pointer: usize = self.prog.len();
        while self.p < max_pointer - 1 {
            match self.instruction() {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => panic!("Invalid instruction"),
            }
        }
    }

    fn output_string(&self) -> String {
        self.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn parse_input(input: &str) -> Computer {
    let mut parts = input.split("\n\n");
    let mut registers = parts
        .next()
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(": ").skip(1).next().unwrap())
        .map(|r| r.parse().unwrap());

    let a = registers.next().unwrap();
    let b = registers.next().unwrap();
    let c = registers.next().unwrap();

    let program = parts
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    Computer {
        a,
        b,
        c,
        p: 0,
        prog: program,
        output: vec![],
    }
}

pub fn part_1() -> String {
    let mut computer = parse_input(include_str!("../input.txt"));
    computer.run();
    computer.output_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut computer = parse_input(include_str!("../input-test.txt"));
        computer.run();
        assert!(computer.output_string() == "4,6,3,5,6,3,5,2,1,0")
    }
}
