extern crate text;
use text::Sort;

#[test]
fn test_sort_by_occurrence() {
    let input: Vec<String> = (vec!["One", "Single", "One", "1"])
        .iter()
        .map(|w| w.to_string())
        .collect();
    let expected: Vec<String> = (vec!["One", "One", "Single", "1"])
        .iter()
        .map(|w| w.to_string())
        .collect();

    let got = input.sort_by_occurrence();

    assert_eq!(got, expected);
}

#[test]
fn test_repetitions() {
    let input: Vec<String> = (vec!["One", "Single", "One", "1"])
        .iter()
        .map(|w| w.to_string())
        .collect();
    let got = input.repetitions();

    assert_eq!(got[0], ("One".to_string(), 2));
    assert_eq!(got[1], ("Single".to_string(), 1));
    assert_eq!(got[2], ("1".to_string(), 1));
}
