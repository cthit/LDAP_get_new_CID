use dotenv::dotenv;
use std::io::BufRead;
use std::{env, error::Error};
use ureq::serde_json::json;

fn main() {
    dotenv().ok();

    let key_id = env::var("GAMMA_API_ID").expect("GAMMA_API_KEY_ID must be set");
    let key_secret = env::var("GAMMA_API_KEY").expect("GAMMA_API_KEY_SECRET must be set");

    let api_key = format!("pre-shared {}:{}", key_id, key_secret);

    println!("API Key: {}", api_key);
    try_sync(api_key.as_str())
}

fn try_sync(api_key: &str) {
    sync(api_key).unwrap_or_else(|e| {
        eprintln!("Failed sync: {}", e);
    });
}

fn sync(api_key: &str) -> Result<(), Box<dyn Error>> {
    let cids = read_cid_from_file().expect("Could not read cids from file");
    let res = ureq::post("https://auth.chalmers.it/api/allow-list/v1")
        .set("Authorization", api_key)
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .send_json(json!({
            "cids": cids,
        }))?;
    let failed_cids = match res.status() {
        200 => vec![],
        206 => res.into_json::<Vec<String>>().unwrap_or(cids.clone()),
        status => {
            println!("Server responded with unexpected status {}", status);
            cids.clone()
        }
    };
    let failed_count = cids.len() - failed_cids.len();
    println!("\nAdded {} out of {} CIDs", failed_count, cids.len());
    if !failed_cids.is_empty() {
        println!("The following CIDs failed to add:\n{:?}", failed_cids);
    }
    Ok(())
}

fn read_cid_from_file() -> Result<Vec<String>, Box<dyn Error>> {
    // Read user input and trim filename
    println!("Enter the file name containing the CIDs:");
    let mut file_name = String::new();
    std::io::stdin().read_line(&mut file_name).unwrap();
    let file_name = file_name.trim();

    // Open file
    let file_path = std::env::current_dir().unwrap().join(file_name);
    let file = std::fs::File::open(file_path).expect("Could not open file");

    // Read file line by line
    let reader = std::io::BufReader::new(file);
    let cids: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    Ok(cids)
}
