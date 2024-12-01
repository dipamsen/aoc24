pub fn run(input: &str) -> (i32, i32) {
    // let it = input
    //     .lines()
    //     .map(|line| {
    //         line.trim()
    //             .split_ascii_whitespace()
    //             .map(|num| num.parse::<i32>().unwrap())
    //             .collect::<Vec<i32>>()
    //     })
    //     .collect::<Vec<Vec<i32>>>();

    // let mut list1 = it.iter().map(|x| x[0]).collect::<Vec<i32>>();
    // let mut list2 = it.iter().map(|x| x[1]).collect::<Vec<i32>>();

    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    list1.sort();
    list2.sort();

    let mut sum = 0;

    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }

    let mut sim = 0;

    let mut j = 0;

    for num in list1.iter() {
        while j < list2.len() && list2[j] < *num {
            j += 1;
        }

        while j < list2.len() && list2[j] == *num {
            sim += num;
            j += 1;
        }
    }

    // for num in list1.iter() {
    //     for num2 in list2.iter() {
    //         if num == num2 {
    //             sim += num;
    //         }
    //     }
    // }

    (sum, sim)
}
