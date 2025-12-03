fn main() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();

    let answer: u64 = input
        .split(',')
        .map(|range| {
            range
                .trim()
                .split_once('-')
                .map(|(f, l)| (f.parse().unwrap(), l.parse().unwrap()))
                .unwrap()
        })
        .flat_map(|(f, l)| (f..=l).filter_map(|number| repeats_pattern(number).then_some(number)))
        .sum();

    println!("Answer: {answer}");
    assert_eq!(answer, 4174379265);
}

fn repeats_pattern(number: u64) -> bool {
    let text = format!("{number}");
    for pat_len in 1..=text.len() / 2 {
        if text.len() % pat_len != 0 {
            continue;
        }

        let pat = &text[..pat_len];

        let mut invalid = true;
        let mut i = 0;
        while i < text.len() {
            if !text[i..].starts_with(pat) {
                invalid = false;
                break;
            }
            i += pat_len;
        }

        if invalid {
            println!("{text}");
            return true;
        }
    }
    false
}
