use csv::Reader;
use serde::Deserialize;
use std::collections::{HashSet, HashMap};
#[derive(Debug, Deserialize)]
struct Rating{
    #[serde(rename = "userId")]
    user_id: u32,
    #[serde(rename = "movieId")]
    movie_id: u32,
    rating: f32,
    timestamp: i64

}
#[derive(Debug, Deserialize)]
struct Movie{
    #[serde(rename = "movieId")]
    movie_id:u32,
    title: String,
    genres: String
}
fn load_ratings(path: &str) -> Result<Vec<Rating>, Box<dyn std::error::Error>>{
    let file = std::fs::File::open(path)?;
    let mut read = Reader::from_reader(file);
    let mut rate_vec:Vec<Rating> = Vec::new();
    for result in read.deserialize(){
        let record:Rating = result?;
        rate_vec.push(record);
    }
    Ok(rate_vec)
    
}
fn load_movies(path:&str) -> Result<Vec<Movie>, Box<dyn std::error::Error>>{
    let file = std::fs::File::open(path)?;
    let mut read = Reader::from_reader(file);
    let mut vec_mov:Vec<Movie> = Vec::new();
    for result in read.deserialize(){
        let record:Movie = result?;
        vec_mov.push(record);
    }
    Ok(vec_mov)
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let rate_vec = load_ratings("data/ml-latest-small/ratings.csv")?;
    let mov_vec = load_movies("data/ml-latest-small/movies.csv")?;
    let mut users:HashSet<u32> = HashSet::new();
    let mut movies:HashSet<u32> = HashSet::new();
    let mut count_movie_rates:HashMap<u32,usize> = HashMap::new();
    let mut movie_names:HashMap<u32, &String> = HashMap::new();
    let mut counts = [0usize;11];

    for i in &rate_vec{
        users.insert(i.user_id);
        movies.insert(i.movie_id);
        let index = (i.rating * 2.0) as usize;
        counts[index] +=1;
        let counter = count_movie_rates.entry(i.movie_id).or_insert(0);
        *counter+=1;
    }
    for i in &mov_vec{
        movie_names.insert(i.movie_id, &i.title);
    }

    let mut vec_movie_rates: Vec<(u32, usize)> = count_movie_rates.into_iter().collect();
    vec_movie_rates.sort_by(|a, b| b.1.cmp(&a.1));
    for i in vec_movie_rates.into_iter().take(10){
        let mut movie_name = String::new();
        if let Some(title) = movie_names.get(&i.0){
            movie_name = title.to_string();
        }
         println!("{} - {} оценок.", movie_name, i.1)
    }
    println!("{}", mov_vec.len());
    let fullness = (rate_vec.len() as f64) / ((users.len() as f64) * (movies.len() as f64));
    println!("Пользователи: {}, Фильмы: {}, Заполненность матрицы: {:.2} %", users.len(), movies.len(), fullness * 100.0);
    for i in 1..=10{
        println!("Оценка :{}, Количество оценок: {}",i as f32 / 2.0, counts[i]);
    }
   
    Ok(())
}
