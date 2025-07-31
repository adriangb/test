use reqwest;
use std::error::Error;

async fn are_csv_schemas_equal(file_urls: &[&str]) -> anyhow::Result<bool> {
    todo!();
}

// async fn are_files_equal(file_urls: &[&str]) -> anyhow::Result<bool> {
//     todo!();
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Test are_csv_schemas_equal
    assert_eq!(
        are_csv_schemas_equal(&[
            "https://www.adriangb.com/test/1.csv",
            "https://www.adriangb.com/test/2.csv"
        ])
        .await?,
        true
    );
    assert_eq!(
        are_csv_schemas_equal(&[
            "https://www.adriangb.com/test/1.csv",
            "https://www.adriangb.com/test/3.csv"
        ])
        .await?,
        false
    );
    assert_eq!(
        are_csv_schemas_equal(&[
            "https://www.adriangb.com/test/1.csv",
            "https://www.adriangb.com/test/4.csv"
        ])
        .await?,
        true
    );

    // // Test are_files_equal
    // assert_eq!(
    //     are_files_equal(&[
    //         "https://www.adriangb.com/test/1.csv",
    //         "https://www.adriangb.com/test/2.csv"
    //     ])
    //     .await?,
    //     true
    // );
    // assert_eq!(
    //     are_files_equal(&[
    //         "https://www.adriangb.com/test/1.csv",
    //         "https://www.adriangb.com/test/3.csv"
    //     ])
    //     .await?,
    //     false
    // );
    // assert_eq!(
    //     are_files_equal(&[
    //         "https://www.adriangb.com/test/1.csv",
    //         "https://www.adriangb.com/test/4.csv"
    //     ])
    //     .await?,
    //     false
    // );

    println!("All tests passed!");
    Ok(())
}
