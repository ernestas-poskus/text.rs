extern crate text;
use text::Repetitions;
use text::Occurrence;

#[test]
fn test_repetitions() {
    let input = vec!["One", "One", "Single", "One", "1", "1"];
    let occurrence = input.occurrence();

    let repetitions = occurrence.repetitions();
    assert_eq!(repetitions[0].0, "One");
    assert_eq!(repetitions[0].1, 3);

    assert_eq!(repetitions[1].0, "1");
    assert_eq!(repetitions[1].1, 2);

    assert_eq!(repetitions[2].0, "Single");
    assert_eq!(repetitions[2].1, 1);
}
