use day_01::part2;

fn main() {
    let input = include_str!("../../resources/input-01.txt");
    let result = part2::solve(input).unwrap();
    println!("{}", result)
}