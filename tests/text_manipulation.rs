extern crate text;
use text::Manipulation;

#[test]
fn test_average_length() {
    let got: Vec<String> = (vec!["one", "two", "four"]).iter().map(|w| w.to_string()).collect();
    assert_eq!(got.average_length(), 3.3333333333333335);
    let got2: Vec<String> = Vec::new();
    assert_eq!(got2.average_length(), 0.0);
}

#[test]
fn test_char_count() {
    let got: Vec<String> =
        (vec!["one", "ударения", "four"]).iter().map(|w| w.to_string()).collect();
    assert_eq!(got.char_count(), 15);
}


#[test]
fn test_remove_punct() {
    let input: Vec<String> = (vec!["removed,;:...!'\""]).iter().map(|w| w.to_string()).collect();
    assert_eq!(input.remove_punct(), vec!["removed"])
}

#[test]
fn test_punctuaction_count() {
    let got: Vec<String> = (vec![",,", "-Ųę22aa191 ударения", "r!!"])
        .iter()
        .map(|w| w.to_string())
        .collect();
    assert_eq!(got.punct_count(), 5);
}
