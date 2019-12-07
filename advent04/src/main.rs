#![feature(is_sorted)]
fn main() {
    let mut found_part1 = 0;
    let mut found_part2 = 0;
    for candidate in 165432..707912 {
        if checks_criteria(candidate, &|c| c >= 2) {
            found_part1 += 1;
        }
        if checks_criteria(candidate, &|c| c == 2) {
            found_part2 += 1;
        }
    }
    println!("found 1 {:?}", found_part1);
    println!("found 2 {:?}", found_part2);
}

fn checks_criteria(candidate: i32, filter: &dyn Fn(usize) -> bool) -> bool {
    let digits = Vec::from(candidate.to_string());
    if !digits.is_sorted() {
        return false;
    }
    let mut last_change = 0;
    for (i, digit) in digits.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if digits[i - 1] != *digit {
            if filter(i - last_change) {
                return true;
            }
            last_change = i;
        }
    }
    if filter(digits.len() - last_change) {
        return true;
    }
    false
}

#[test]
fn testcase1() {
    assert!(checks_criteria(234499, &|c| c == 2));
    assert!(checks_criteria(666799, &|c| c == 2));
}
