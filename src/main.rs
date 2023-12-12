// mod trebuchet;
// mod cube_conundrum;
// mod gear_rotations;
// mod scratchcards;
mod fertilizer;
// mod boats;
// mod camel_cards;
// mod haunted_wastelands;
// mod pipes;
// mod cosmic_expansion;

fn main() {
    match fertilizer::solve_2() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    };
}
