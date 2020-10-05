/// Helper function to read user input.
pub fn read_str() -> String
{
    print!("> ");
    std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed!");

    // Get port as string
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Unable to read user input.");

    return  String::from(input.trim());
}

/// Helper function to have the user pick a character from a list of valid choices.
/// 
/// The argument is an array of valid uppercase characters and the function returns the chosen character.
pub fn pick_char(valid_chars : &Vec<char>) -> char
{
    // Loop to constantly as for input
    loop
    {
        // Read input
        let input = read_str();

        // Must be a single character
        if input.len() == 1
        {
            // Get the uppercase version of the letter
            // Guaranteed not to panic since we have at least 1 character
            let upper = input.chars().next().unwrap().to_uppercase().next().unwrap();

            // Loop over all valid chars
            for c in valid_chars
            {
                // All of this to 
                if upper == *c
                {
                    return *c;
                }
            }
        }

        // Unable to read input
        println!("Sorry, I wasn't able to understand you.");
    }   
}