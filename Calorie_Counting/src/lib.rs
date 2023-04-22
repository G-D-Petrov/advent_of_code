pub fn get_max_calories(input: &str, n: usize) -> usize {
    let mut max_calories = Vec::<usize>::new();
    for group in input.split("\n\n"){
        let mut calories = 0;
        println!("Group: {}", group);
        for line in group.lines() {
            println!("Line: {}", line);
            calories += line.trim().parse::<usize>().unwrap();
        }
        max_calories.push(calories);
    }   

    max_calories.sort();
    max_calories.reverse();
    max_calories.into_iter().take(n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part_1_1() {
        let input = include_str!("test_input.txt");

        let result = get_max_calories(input, 1);
        assert_eq!(result, 24000);
    }

    #[test]
    fn it_works_part_1_2() {
        let input = include_str!("eval_1.txt");
    
        let result = get_max_calories(input, 1);
        println!("{}", result);
        assert!(result == 68802);
    }

    #[test]
    fn test_part_2_1() {
        let input = include_str!("test_input.txt");
        let result = get_max_calories(input, 3);
        assert_eq!(result, 45000);
    }

    #[test]
    fn test_part_2_2() {
        let input = include_str!("eval_2.txt");
        let result = get_max_calories(input, 3);
        println!("Result: {}", result);

        assert!(result == 205370);
    }
}
