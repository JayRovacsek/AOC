use rand::random;

// Function to get user string input, as we're taking untrusted input
// we're expecting failure with a message.
fn get_string_input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin!");
    buffer
}

// Function to return a random u8 value, with a defined limit, also
// don't return 0
pub fn get_rand_u8(limit: u8) -> u8 {
    let r = random::<u8>() % (limit + 1);
    match r {
        0 => get_rand_u8(limit + 1) % limit,
        _ => r,
    }
}

// Function to get option value from the user, on failure return
// a suitable u8 in the range 1-25
pub fn get_option(prompt: &str) -> u8 {
    let input: String = get_string_input(prompt);

    match input.trim().parse::<u8>() {
        Ok(val) => val,
        _ => {
            let u: u8 = get_rand_u8(25);
            println!("Failed to parse a suitable number from input, let's enjoy some chaos and choose a random one...");
            println!("Looks like we chose {}!", u);
            u
        }
    }
}

#[test]
fn test_get_rand_u8() {
    let val = get_rand_u8(1 as u8);
    assert!(val > u8::min_value() || val <= u8::max_value())
}

#[test]
#[should_panic]
fn test_panic_get_rand_u8() {
    let val = get_rand_u8(1 as u8);
    assert!(val < u8::min_value() || val > u8::max_value())
}
