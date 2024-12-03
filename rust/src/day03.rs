pub fn run(input: &str) -> (i32, i32) {
    let mut sum = 0;
    let mut sum2 = 0;
    let mut enabled = true;
    for line in input.lines() {
        for i in 0..line.len() {
            if i + 4 < line.len() && &line[i..i + 4] == "mul(" {
                let mut j = i + 4;
                let mut a = 0;
                let mut b = 0;

                while let Some(c) = line.chars().nth(j) {
                    if !c.is_digit(10) {
                        break;
                    }
                    a = a * 10 + c.to_digit(10).unwrap();
                    j += 1;
                }

                if line.chars().nth(j) != Some(',') {
                    continue;
                }

                j += 1;

                while let Some(c) = line.chars().nth(j) {
                    if !c.is_digit(10) {
                        break;
                    }
                    b = b * 10 + c.to_digit(10).unwrap();
                    j += 1;
                }

                if line.chars().nth(j) != Some(')') {
                    continue;
                }

                sum += a * b;
                if enabled {
                    sum2 += a * b;
                }
            } else if i + 4 < line.len() && &line[i..i + 4] == "do()" {
                enabled = true;
            } else if i + 7 < line.len() && &line[i..i + 7] == "don't()" {
                enabled = false;
            }
        }
    }

    (sum.try_into().unwrap(), sum2.try_into().unwrap())
}
