use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    // 1. Create a new blocking HTTP client
    let client = Client::new();

    // 2. Define a username and optional password for basic authentication
    let user = "testuser".to_string();
    let password: Option<String> = None; // No password provided (None)
    // let password: Option<String> = Some("password".to_string()); // If password is provided

    // 3. Make a GET request to http://httpbin.org/get
    //    with basic authentication using the username and optional password
    let response = client.get("http://httpbin.org/get")
        .basic_auth(user, password)
        .send(); // Perform the request

    // 4. Print the response (or error) to the console
    println!("The response: {:?}", response);

    // 5. Return Ok if everything went well
    Ok(())
}
