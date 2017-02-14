extern crate text;
use text::Intersect;

#[test]
fn test_intersection() {
    let input: Vec<&str> = vec!["1", "2", "3"];
    let result: Vec<&str> = vec!["1"];

    // Vec<String>
    let got: Vec<String> = (input).iter().map(|w| w.to_string()).collect();
    assert_eq!(got.intersect(&result), result);

    // Vec<String>
    assert_eq!(input.intersect(&result), result);
}
