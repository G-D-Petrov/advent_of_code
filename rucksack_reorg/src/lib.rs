fn split_line(line: &str) -> Vec<&str> {
    let (first_half, second_half) = line.split_at(line.len() / 2);

    vec![first_half, second_half]
}

fn find_unique_item(input: Vec<&str>) -> Option<char> {
    for c in input[0].chars() {
        let mut contained = true;
        for i in 1..input.len() {
            contained &= input[i].contains(c) 
        }

        if contained {
            return Some(c);
        }
    }

    None
}

pub enum SplitStrat {
    SplitLine,
    SplitOn3Lines,
}

pub fn calculate_priorities(input: &str, split_stat: SplitStrat) -> usize {
    let mut result = 0;
    
    let item_input = match split_stat {
        SplitStrat::SplitLine => {
            let mut result = Vec::<Vec<&str>>::new();
            for line in input.lines() {
                let inner = split_line(line);
                result.push(inner) 
            }
            result
        },
        SplitStrat::SplitOn3Lines => {
            let mut result = Vec::<Vec<&str>>::new();
            let mut it = input.lines();
            while let Some(line) = it.next() {
                let inner = vec![line, it.next().unwrap(), it.next().unwrap()];
                result.push(inner);
            }
            result
        },
    };

        

    for input in item_input {
        let unique_item = find_unique_item(input).unwrap();
        let priority = if unique_item >= 'a' && unique_item <= 'z' {
            (unique_item as usize - 'a' as usize) + 1 
        } else {
            (unique_item as usize - 'A' as usize) + 27
        };
        result += priority;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let input = include_str!("test_input.txt");
        let result = calculate_priorities(input, SplitStrat::SplitLine);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part_1_2() {
        let input = include_str!("test_1_2.txt");
        let result = calculate_priorities(input, SplitStrat::SplitLine);
        assert_eq!(result, 7737);
    }

    #[test]
    fn test_part_2_1() {
        let input = include_str!("test_input.txt");
        let result = calculate_priorities(input, SplitStrat::SplitOn3Lines);
        assert_eq!(result, 70);
    }

    #[test]
    fn test_part_2_2() {
        let input = include_str!("test_2_2.txt");
        let result = calculate_priorities(input, SplitStrat::SplitOn3Lines);
        assert_eq!(result, 2697);
    }
}
