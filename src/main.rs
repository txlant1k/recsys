use csv::Reader;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Rating{
    #[serde(rename = "userId")]
    user_id: u32,
    #[serde(rename = "movieId")]
    movie_id: u32,
    rating: f32,
    timestamp: i64

}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = std::fs::File::open("data/ml-latest-small/ratings.csv")?;
    let mut read = Reader::from_reader(file);
    let mut vec_res:Vec<Rating> = Vec::new();
    for result in read.deserialize(){
        let record:Rating = result?;
        vec_res.push(record);
    }
    println!("{}, {:.?}", vec_res.len(), vec_res[0]);
    Ok(())
}
