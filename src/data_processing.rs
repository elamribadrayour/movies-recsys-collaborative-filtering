use std::error::Error;

use std::fs::File;
use std::io::Write;

use std::collections::HashSet;
use zip::ZipArchive;

use crate::matrix::Matrix;

// Download functions
async fn download_dataset() -> Result<String, Box<dyn Error>> {
    // Download movielens dataset
    let file_path = "dataset/ml-latest-small.zip";
    let url = "https://files.grouplens.org/datasets/movielens/ml-latest-small.zip";

    log::trace!(
        "downloading movielens dataset from {} to {}",
        url,
        file_path
    );
    let response = reqwest::get(url).await?;
    let mut file = File::create(file_path)?;
    file.write_all(&response.bytes().await?)?;
    Ok(file_path.to_string())
}

async fn unzip_dataset(file_path: &String) -> Result<String, Box<dyn Error>> {
    // Unzip movielens dataset
    let output_path = "dataset/";
    log::trace!("unzipping movielens dataset to {}", output_path);
    let zip_file = File::open(file_path)?;
    let mut zip_archive = ZipArchive::new(zip_file)?;
    zip_archive.extract(output_path)?;
    Ok(output_path.to_string() + "ml-latest-small/")
}

async fn get_ratings_file(dir_path: &str) -> Result<File, Box<dyn Error>> {
    let file_path: String = dir_path.to_string() + "ratings.csv";
    log::trace!("reading ratings file {}", file_path);
    let file = File::open(&file_path)?;
    Ok(file)
}

async fn get_movies_file(dir_path: &str) -> Result<File, Box<dyn Error>> {
    let file_path: String = dir_path.to_string() + "movies.csv";
    log::trace!("reading movies file {}", file_path);
    let file = File::open(&file_path)?;
    Ok(file)
}

async fn get_number_of_users(dir_path: &str) -> Result<usize, Box<dyn Error>> {
    // Get number of users from movielens dataset using the ratings file
    let file = get_ratings_file(dir_path).await?;
    let mut reader = csv::Reader::from_reader(file);
    let mut users: HashSet<u32> = HashSet::new();
    for result in reader.records() {
        let record = result?;
        let user_id = record[0].parse()?;
        users.insert(user_id);
    }
    Ok(users.len() + 1)
}

async fn get_number_of_items(dir_path: &str) -> Result<usize, Box<dyn Error>> {
    // Get number of items from movielens dataset using the movies file
    let file = get_movies_file(dir_path).await?;
    let mut reader = csv::Reader::from_reader(file);
    let mut items: HashSet<u32> = HashSet::new();
    for result in reader.records() {
        let record = result?;
        let item_id = record[0].parse()?;
        items.insert(item_id);
    }
    Ok(items.len() + 1)
}

async fn get_ratings(dir_path: &str) -> Result<Matrix, Box<dyn Error>> {
    // Get ratings from movielens dataset
    let file = get_ratings_file(dir_path).await?;
    let mut reader = csv::Reader::from_reader(file);

    let num_users = get_number_of_users(dir_path).await?;
    let num_items = get_number_of_items(dir_path).await?;
    log::trace!("building matrix with {num_users} users and {num_items} items...");

    let mut matrix = Matrix::new(num_users, num_items);
    for result in reader.records() {
        let record = result?;
        let user_id = record[0].parse()?;
        let item_id = record[1].parse()?;
        let rating = record[2].parse()?;
        matrix.set_user(user_id);
        matrix.set_item(item_id);
        matrix.set_rating(user_id, item_id, rating);
    }
    Ok(matrix)
}

pub async fn get_dataset() -> Result<Matrix, Box<dyn Error>> {
    // Download and unzip movielens dataset
    let zip_path: String = download_dataset().await?;
    let dir_path: String = unzip_dataset(&zip_path).await?;
    get_ratings(&dir_path).await
}
