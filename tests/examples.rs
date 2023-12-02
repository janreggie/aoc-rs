use aoc_rs::util;

mod y17;
mod y21;
mod y23;

#[test]
fn test_all_examples() {
    // let mut examples = y17::examples();
    // examples.extend(y21::examples());
    // examples.extend(y23::examples());
    let examples = y23::examples();

    for example in &examples {
        util::test_puzzle(example);
    }
}
