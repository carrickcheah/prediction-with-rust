
// randomly split the data into training and testing sets
pub fn split_data(df: &DataFrame) -> (DataFrame, DataFrame) {
    let (train_df, test_df) = df.random_split(0.8, None);
    (train_df, test_df)
}