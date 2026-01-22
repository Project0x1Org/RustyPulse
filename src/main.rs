use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // 1. Get User Input
    let url = get_user_input("Enter The URL to ping: ");

    // 2. Process The Request
    println!("Checking {}, .....", url);
    let result = check_status(&url).await;

    // 3. Output Results
    match result {
        Ok(msg) => println!("Success: {}", msg),
        Err(err) => println!("Error: {}", err),
    }
}

/// Pings a Provided URL and Returns a formatted status message.
/// # Arguments:
/// * `url` - A string slice containing the target URL (e.g: https://google.com)
/// # Returns:
/// * `Ok(String)` - if the network request completed, (even if server responded with 403,404,500,...etc)
/// * `Err(String)` - if the URL is malformed or the server is Unreachable
async fn check_status(url: &str) -> Result<String, String> {
    let ping_result = reqwest::get(url).await;

    match ping_result {
        Ok(response) => {
            let status = response.status();

            if status.is_success() {
                Ok(format!("Success!, Code: {}", status.as_u16()))
            } else {
                Ok(format!("Server Responded With Error: {}", status.as_u16()))
            }
        }
        Err(err) => {
            if err.is_builder() {
                Err(String::from("Invalid URL format"))
            } else {
                Err(String::from("Connection failed / Server Unreachable"))
            }
        }
    }
}

/// Helper to capture user input from terminal, structured to be integrated later in GUI.
/// # Arguments:
/// * `prompt` - string slice containing description to tell user what to input.
/// # Returns:
/// * `input(String)` - the input from the user
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // print prompt to stdout immediately

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read Line");

    input.trim().to_string()
}
