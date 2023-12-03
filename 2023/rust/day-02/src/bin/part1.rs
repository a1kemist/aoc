use day_02::part1;

fn main() {
    let input = include_str!("../../resources/input-01.txt");
    let result = part1::solve(input).unwrap();
    println!("{}", result)
}
