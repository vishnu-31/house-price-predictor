use anyhow::Ok;
use polars::prelude::IndexOrder::C;
use polars::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use xgboost::{parameters, Booster, DMatrix };

pub fn download_csv_file() -> anyhow::Result<String> {
    let url = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;

    let file_path = "boston_housing.csv";
    std::fs::write(file_path, bytes)?;
    Ok(file_path.to_string())
}

pub fn load_csv(file_path: &str) -> anyhow::Result<DataFrame> {
    let df = CsvReader::from_path(file_path)?.finish()?;

    println!(
        "Loaded with {} rows and {} columns",
        df.height(),
        df.width()
    );
    println!("{:?}", df.head(Some(5)));

    Ok(df)
}

pub fn download_nyc_data() -> anyhow::Result<String> {
    // https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2024-01.parquet
    let nyc_url = "https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2024-0";

    for n in 1..10 {
        let response = reqwest::blocking::get(format!("{}{}.parquet", nyc_url, n))?;
        let bytes = response.bytes()?;
        let nyc_data_path = format!("nyc_data_0{}.parquet", n);
        std::fs::write(nyc_data_path, bytes)?;
    }
    Ok("NYC data is downloaded for the current year".to_string())
}

pub fn train_test_split(
    df: &DataFrame,
    test_size_percent: f64,
) -> anyhow::Result<(DataFrame, DataFrame)> {
    let mut indices: Vec<usize> = (0..df.height()).collect();
    //Shuffle the indices
    let mut rng = thread_rng();

    indices.shuffle(&mut rng);

    let split_ind = (df.height() as f64 * (1.0 - test_size_percent)) as usize;

    let train_indices = indices[0..split_ind].to_vec();
    let test_indices = indices[split_ind..].to_vec();

    let train_indices_ca =
        UInt32Chunked::from_vec("", train_indices.iter().map(|&i| i as u32).collect());
    let test_indices_ca =
        UInt32Chunked::from_vec("", test_indices.iter().map(|&i| i as u32).collect());

    let train_df = df.take(&train_indices_ca)?;
    let test_df = df.take(&test_indices_ca)?;

    println!("Train DF shape: {:?}", train_df.height());
    println!("Test DF shape: {:?}", test_df.height());

    Ok((train_df, test_df))
}

pub fn split_features_and_target(df: &DataFrame) -> anyhow::Result<(DataFrame, DataFrame)> {
    let features = df.select(&vec![
        "crim", "zn", "indus", "chas", "nox", "rm", "age", "dis", "rad", "tax", "ptratio", "b",
        "lstat",
    ])?;
    let target = df.select(vec!["medv"])?;

    Ok((features, target))
}

pub fn triain_xgboost_model(
    x_train: &DataFrame,
    y_train: &DataFrame,
    x_test: &DataFrame,
    y_test: &DataFrame,
) -> anyhow::Result<String> {
    let x_train_nda = x_train.to_ndarray::<Float32Type>(C).unwrap();
    let y_train_nda = y_train.to_ndarray::<Float32Type>(C).unwrap();
    let x_test_nda = x_test.to_ndarray::<Float32Type>(C).unwrap();
    let y_test_nda = y_test.to_ndarray::<Float32Type>(C).unwrap();

    let x_train_slice = x_train_nda.as_slice().unwrap();
    let y_train_slice = y_train_nda.as_slice().unwrap();
    let y_test_slice = y_test_nda.as_slice().unwrap();
    let x_test_slice = x_test_nda.as_slice().unwrap();

    let mut dtrain = DMatrix::from_dense(x_train_slice, x_train.height())?;
    dtrain.set_labels(y_train_slice)?;

    let mut dtest = DMatrix::from_dense(x_test_slice, x_test.height())?;
    dtest.set_labels(y_test_slice)?;

    let evaluation_sets = &[(&dtrain, "train"), (&dtest, "test")];

    let parameters = parameters::TrainingParametersBuilder::default()
        .dtrain(&dtrain)
        .evaluation_sets(Some(evaluation_sets))
        .build().unwrap();

    let model = Booster::train(&parameters)?;

    print!("Test score: {:?}", model.predict(&dtest).unwrap());

    let model_path = "xgboost_model.bin";
    model.save(model_path)?;

    Ok(model_path.to_string())
}
