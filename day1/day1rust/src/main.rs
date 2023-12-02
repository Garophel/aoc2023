use std::convert::identity;
use std::fs::File;
use std::io::{ BufRead, BufReader };

#[derive(PartialEq, Eq, Clone, Copy)]
enum Part {
    Part1,
    Part2,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Forward,
    Reverse,
}

static WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five",
    "six", "seven", "eight", "nine"
];

fn find_number_words<F>(line: &str, find_func: F) -> Vec<(u32, usize)>
where F: Fn(&str, &str) -> Option<usize>
{
    let mut word_start_indices = (1..).zip(WORDS)
        .map(|(num, word)| (num, find_func(line, word)))
        .filter_map(|(num, opt_i)| opt_i.map(|i| (num, i)))
        .collect::<Vec<_>>();

    word_start_indices.sort_by_key(|(_, i)| *i);
    word_start_indices
}

fn find_number(line: &str, direction: Direction, part: Part) -> Option<u32> {
    use Direction::*;

    let mut chars = line.chars().enumerate().collect::<Vec<_>>();

    if direction == Reverse {
        chars.reverse();
    }

    let digit = chars.iter()
        .find(|(_, c)| c.is_ascii_digit())
        .map(|(i, c)| (i, c.to_digit(10)))
        .and_then(|(i, num_opt)| num_opt.map(|num| (num, i)));

    if part == Part::Part1 {
        return digit.map(|(num, _)| num);
    }

    let word = if direction == Forward {
        find_number_words(line, |line, word| line.find(word)).first().copied()
    } else {
        find_number_words(line, |line, word| line.rfind(word)).last().copied()
    };

    let cmp_func = if direction == Forward {
        usize::lt
    } else {
        usize::gt
    };

    match (digit, word) {
        (Some((digit_num, digit_index)), Some((word_num, word_index))) => {
            if cmp_func(&digit_index, &word_index) {
                Some(digit_num)
            } else {
                Some(word_num)
            }
        },
        (Some((digit_num, _)), None) => Some(digit_num),
        (None, Some((word_num, _))) => Some(word_num),
        (None, None) => None,
    }
}

fn extract_line_code(line: &str, part: Part) -> Option<i32> {
    use Direction::*;

    let first = find_number(line, Forward, part)? as i32;
    let last = find_number(line, Reverse, part)? as i32;
    Some(first * 10 + last)
}

fn find_answer(path: &str, part: Part) -> i32 {
    let reader = BufReader::new(File::open(path).unwrap());

    reader.lines()
        .flat_map(identity)
        .flat_map(|line| extract_line_code(&line, part))
        .sum()
}

fn main() {
    use Part::*;
    let path = std::env::args().skip(1).next();

    if let Some(path) = path {
        let part1sum = find_answer(&path, Part1);
        println!("Part 1 result is: {}", part1sum);

        let part2sum = find_answer(&path, Part2);
        println!("Part 2 result is: {}", part2sum);
    } else {
        eprintln!("Missing argument: path-to-input-file");
        std::process::exit(1);
    }
}
