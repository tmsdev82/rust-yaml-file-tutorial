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
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{:?}", scrape_config);

    println!(
        "update_frequency_sec: {}",
        scrape_config.update_frequency_sec
    );

    for data_source in scrape_config.data_sources.iter() {
        println!("{}", data_source);
    }
}
