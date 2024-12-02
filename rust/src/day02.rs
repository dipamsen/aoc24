fn check_grad_mono(nums: &[i32], ignore: i32) -> bool {
    if ignore == 0 {
        return check_grad_mono(&nums[1..], -1);
    }
    let is_inc = if ignore == 1 {
        nums[0] < nums[2]
    } else {
        nums[0] < nums[1]
    };

    for i in 1..nums.len() {
        if ignore == (i as i32) {
            continue;
        }

        let curr = nums[i];
        let prev = if ignore == (i as i32) - 1 {
            nums[i - 2]
        } else {
            nums[i - 1]
        };

        if (curr < prev) == is_inc {
            return false;
        }
        let diff = (curr - prev).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

pub fn run(input: &str) -> (i32, i32) {
    let reports = input
        .lines()
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut safe_count = 0;

    for report in reports.iter() {
        if check_grad_mono(report, -1) {
            safe_count += 1;
        }
    }

    let mut mod_safe_count = 0;

    for report in reports.iter() {
        for i in 0..report.len() {
            if check_grad_mono(report, i as i32) {
                mod_safe_count += 1;
                break;
            }
        }
    }

    (safe_count, mod_safe_count)
}
