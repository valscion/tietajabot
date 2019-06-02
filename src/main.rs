fn main() {
    let mut strings = vec![
        "first".to_string(),
        "apple".to_string(),
        "fooboobaa".to_string(),
        "orange".to_string(),
        // The next would panic as we're being lazy and using .unwrap(),
        // assuming every string always has a first character.
        // "".to_string(),
    ];

    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'z',
    ];
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for s in &mut strings {
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();
        if consonants.contains(&first_char) {
            let mut new_s = String::new();
            for c in chars {
                new_s.push(c);
            }
            *s = format!("{}-{}ay", new_s, first_char);
        } else if vowels.contains(&first_char) {
            s.push_str("-hay");
        }
    }

    println!("strings: {:#?}", strings);
}
