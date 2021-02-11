// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let numbers: Vec<Option<u16>> = (0..5).into_iter().map(|x| {
        Some(((x * 1235) + 2) / (4 * 16))
    }).collect();
}
