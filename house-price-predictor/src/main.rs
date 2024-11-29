// Training script entry point
// Steps
// 1. Download external CSV file to disk
// 2. Load file from disk into memory
// 3. Prepare the data
// 4. Train an XGBoost model with this data
// 5. Push this model to an AWS S3 bucket (model registry)
fn main() -> anyhow::Result<()> {
    println!("Starting training script...");

    // 1. Download external CSV file to disk
    let csv_file_path = download_csv_file()?;

    Ok(())
}


// Downloads the external CSV file to disk
fn download_csv_file() -> anyhow::Result<String> {

    let url = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";
    
    // Get the response from the URL
    let response = reqwest::blocking::get(url)?;
    
    // Get the bytes from the response into memory
    let bytes = response.bytes()?;

    let file_path = "boston_housing.csv";

    // Copy these bytes to a file on disk
    std::fs::write(file_path, bytes)?;

    Ok(file_path.to_string())
}


