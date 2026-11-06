use csv::Reader;
use serde::Deserialize;
use std::collections::HashSet;
#[derive(Debug, Deserialize)]
struct Rating{
    #[serde(rename = "userId")]
    user_id: u32,
    #[serde(rename = "movieId")]
    movie_id: u32,
    rating: f32,
    timestamp: i64

}

fn load_ratings(path: &str) -> Result<Vec<Rating>, Box<dyn std::error::Error>>{
    let file = std::fs::File::open(path)?;
    let mut read = Reader::from_reader(file);
    let mut vec_res:Vec<Rating> = Vec::new();
    for result in read.deserialize(){
        let record:Rating = result?;
        vec_res.push(record);
    }
    Ok(vec_res)
    
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let res_vec = load_ratings("data/ml-latest-small/ratings.csv")?;
    let mut users:HashSet<u32> = HashSet::new();
    let mut movies:HashSet<u32> = HashSet::new();

    for i in &res_vec{
        users.insert(i.user_id);
        movies.insert(i.movie_id);
    }
    let fullness = (res_vec.len() as f64) / ((users.len() as f64) * (movies.len() as f64));
    println!("Пользователи: {}, Фильмы: {}, Заполненность матрицы: {:.2} %", users.len(), movies.len(), fullness * 100.0);
    Ok(())
}
