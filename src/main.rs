#![feature(duration_float)]
use std::time::Instant;
use std::fs;
use std::collections::HashMap;

pub fn count_words(mut hash_map: HashMap<String, u32>, word: String) -> HashMap<String, u32> {
    {
        let c = hash_map.entry(word).or_insert(0);
        *c += 1;
    }

    hash_map
}

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), count_words)
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let mut reviews = Vec::new();

    for entry in fs::read_dir("./aclImdb/train/pos")? {
        let entry = entry?;
        let path = entry.path();

        let contents = fs::read_to_string(path).unwrap();
        reviews.push(contents);
    }
    for entry in fs::read_dir("./aclImdb/train/neg")? {
        let entry = entry?;
        let path = entry.path();

        let contents = fs::read_to_string(path).unwrap();
        reviews.push(contents);
    }
    for entry in fs::read_dir("./aclImdb/test/pos")? {
        let entry = entry?;
        let path = entry.path();

        let contents = fs::read_to_string(path).unwrap();
        reviews.push(contents);
    }
    for entry in fs::read_dir("./aclImdb/test/neg")? {
        let entry = entry?;
        let path = entry.path();

        let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
        reviews.push(contents);
    }
    println!("read finished {}", now.elapsed().as_secs_f32());
    let now = Instant::now();

    let reviews = &reviews[..].concat();
    println!("concat finished {}", now.elapsed().as_secs_f32());
    let now = Instant::now();

    let _r = word_count(&reviews);
    println!("count finished {}", now.elapsed().as_secs_f32());

    Ok(())
}

fn main() {
    run().unwrap();
}