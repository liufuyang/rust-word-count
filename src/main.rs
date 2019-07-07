#![feature(duration_float)]
use std::time::Instant;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;

use crossbeam::crossbeam_channel::{bounded, Sender, Receiver};

fn count(mut hash_map: HashMap<String, u32>, word: String) -> HashMap<String, u32> {
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
        .fold(HashMap::new(), count)
}

fn gen_paths(sender: Sender<PathBuf>) {
    for entry in fs::read_dir("./aclImdb/train/pos").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        sender.send(path).unwrap();
    }
    for entry in fs::read_dir("./aclImdb/train/neg").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        sender.send(path).unwrap();
    }
    for entry in fs::read_dir("./aclImdb/test/pos").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        sender.send(path).unwrap();
    }
    for entry in fs::read_dir("./aclImdb/test/neg").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        sender.send(path).unwrap();
    }
    drop(sender); // end of sending, close the channel
}

fn read_and_count(
    chan_path_r: Receiver<PathBuf>, 
    chan_map_s: Sender<HashMap<String, u32>>) {
    while let Ok(path) = chan_path_r.recv() {
        let contents = fs::read_to_string(path).expect("file reading error");
        let map = word_count(contents.as_ref());
        chan_map_s.send(map).unwrap();
    }
    drop(chan_map_s); // end of sending, close the channel
}

fn main() {
    let now = Instant::now();

    let (chan_path_s, chan_path_r) = bounded(100);
    let (chan_map_s, chan_map_r) = bounded(100);
    
    thread::spawn(|| gen_paths(chan_path_s));

    for _ in 0..8{
        let r = chan_path_r.clone();
        let s = chan_map_s.clone();
        thread::spawn(|| read_and_count(r, s));
    }
    drop(chan_map_s); // close map channel from main thread
    

    let v: Vec<_> = chan_map_r.iter().collect();

    println!("{}", v.len());

    println!("count finished {}", now.elapsed().as_secs_f32());
}