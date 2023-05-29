use dotenv::dotenv;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

        println!("{}", &database_url);

        Config {
            database_url
        }
    }
}