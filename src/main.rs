// mod trebuchet;
// mod cube_conundrum;
// mod gear_rotations;
// mod scratchcards;
// mod fertilizer;
// mod boats;
// mod camel_cards;
mod haunted_wastelands;

fn main() {
    /*
        Day 1
        ====================
    */

    /*
    Day 2
    ====================
    */

    /*
    Day 3
    ====================
        1. 520019
        2.
    */

    /*
    Day 4
    ====================
        1. 15268
        2. 6283755
    */

    /*
    Day 5
    ====================
        1. 199602917
        2. :(
    */

    /*
    Day 6
    ====================
        1. 440000
        2. 26187338
    */

    /*
    Day 6
    ====================
        1. 248179786
        2. 247885995
    */

    match haunted_wastelands::solve_2() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    };
}
