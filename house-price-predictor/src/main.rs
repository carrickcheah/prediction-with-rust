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

    // 2. Load file from disk into memory
    let df = load_data(&csv_file_path)?;


    // randomly split the data into training and testing sets
    let (train_df, test_df) = df.random_split(0.8, None);

    // 3. Prepare the data



    // 4. Train an XGBoost model with this data



    
    // 5. Push this model to an AWS S3 bucket (model registry)




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


// Load the CSV file from disk into Polaris DataFrame

fn load_csv_file(file_path: &str) -> anyhow::Result<polaris::prelude::DataFrame> {
    let df = polaris::prelude::DataFrame::read_csv(file_path)?;

    Ok(df)
}