use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    update_frequency_sec: u32,
    num_threads: u32,
    data_sources: Vec<String>,
}
fn main() {
    let f = std::fs::File::open("config.yml").expect("Could not open file.");
    let mut scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{:?}", scrape_config);

    println!(
        "update_frequency_sec: {}",
        scrape_config.update_frequency_sec
    );

    for data_source in scrape_config.data_sources.iter() {
        println!("{}", data_source);
    }

    scrape_config.num_threads = 2;

    scrape_config
        .data_sources
        .push("www.nytimes.com".to_string());
    scrape_config
        .data_sources
        .push("news.yahoo.com".to_string());

    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("new_config.yml")
        .expect("Couldn't open file");
    serde_yaml::to_writer(f, &scrape_config).unwrap();
}
