fn get_next_output(a_init: usize) -> (usize, usize) {
    let mut a = a_init;
    let mut b = (a % 8) ^ 5;
    let c = a / 2usize.pow(b as u32);
    b ^= c;
    b ^= 6;
    a /= 2usize.pow(3);
    return (b % 8, a);
}

fn get_required_output(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect()
}

fn find_working_a(required_a: usize, p: usize, output: &Vec<usize>) -> Option<usize> {
    if p == output.len() {
        return Some(required_a);
    }

    let required_output = output[output.len() - p - 1];
    let shift = required_a << 3;
    for i in 0..1 << 10 {
        let test = shift ^ i;
        let (out, a) = get_next_output(test);
        if a == required_a && out == required_output {
            let res = find_working_a(test, p + 1, output);
            if res.is_some() {
                return res;
            }
        }
    }

    None
}

pub fn part_2() -> usize {
    let output = get_required_output(include_str!("../input.txt"));
    find_working_a(0, 0, &output).unwrap_or_default()
}
