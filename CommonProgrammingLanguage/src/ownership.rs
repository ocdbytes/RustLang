fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    let len_ref = calculate_length_ref(&s2);

    println!("The length of '{}' is {}.", s2, len);
    println!("[REF] The length of '{}' is {}.", s2, len_ref);

    // Mutable References

    let mut s = String::from("Hello");
    change(&mut s);

    // Traversing and Return Types

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let s = String::from("This is a car");
    let l_ind_word = first_word(&s);
    println!("The last index of first word : {l_ind_word}");

    // Slicing

    fn first_word_whole(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]; // Slice Operator
            }
        }

        &s[..]
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
