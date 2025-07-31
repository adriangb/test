use reqwest;
use std::error::Error;

async fn are_headers_equal(file_url: &str, file_urls: &[&str]) -> Result<bool, Box<dyn Error>> {
    todo!();
}

async fn are_files_equal(file_url: &str, file_urls: &[&str]) -> Result<bool, Box<dyn Error>> {
    todo!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Test are_headers_equal
    assert_eq!(
        are_headers_equal("https://www.adriangb.com/test/1.csv", &["https://www.adriangb.com/test/2.csv"]).await?,
        true
    );
    assert_eq!(
        are_headers_equal("https://www.adriangb.com/test/1.csv", &["https://www.adriangb.com/test/3.csv"]).await?,
        false
    );
    assert_eq!(
        are_headers_equal("https://www.adriangb.com/test/1.csv", &["https://www.adriangb.com/test/4.csv"]).await?,
        true
    );

    // Test are_files_equal
    assert_eq!(
        are_files_equal("https://www.adriangb.com/test/1.csv", &["https://www.adriangb.com/test/2.csv"]).await?,
        true
    );
    assert_eq!(
        are_files_equal("https://www.adriangb.com/test/1.csv", &["https://www.adriangb.com/test/3.csv"]).await?,
        false
    );
    assert_eq!(
        are_files_equal("https://www.adriangb.com/test/1.csv", &["https://www.adriangb.com/test/4.csv"]).await?,
        false
    );

    println!("All tests passed!");
    Ok(())
}