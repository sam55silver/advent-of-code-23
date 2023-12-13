use utilities::get_input_lines;

fn main() {
    let (lines, part) = get_input_lines();
    lines.iter().for_each(|i| println!("{i}"));
    println!("Part: {part}")
}
