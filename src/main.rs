// Main module will be used for running the latest entry.
mod aoc1_calories;
mod aoc2_rps;
fn main() {
    aoc2_rps::rps(false);
    aoc2_rps::rps(true);
}
fn all() {
    // the christmas chaos
    aoc1_calories::calories();
    aoc2_rps::rps(false);
    aoc2_rps::rps(true);
}
