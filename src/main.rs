#![feature(duration_float)]
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Instant;

use crossbeam::crossbeam_channel::{unbounded, bounded, Receiver, Sender};
use rayon::prelude::*;

type Map = HashMap<String, u32>;

fn main() {
    let now = Instant::now();

    let (chan_path_s, chan_path_r) = unbounded(); // channel to store all the paths to read
    let (chan_map_s, chan_map_r) = bounded(16); // channel to store maps of word count

    thread::spawn(|| gen_paths(chan_path_s));

    for _ in 0..16 {
        let r = chan_path_r.clone();
        let s = chan_map_s.clone();
        thread::spawn(|| read_and_count(r, s));
    }
    drop(chan_map_s); // close map channel from current thead, so to allow the collection on receiver to continue 

    // read all from map channel into a list
    let maps: Vec<Map> = chan_map_r.iter().collect();
    println!("number of maps: {}", maps.len());

    // run parallel to reduce into a single map
    let result = maps.into_par_iter().reduce_with(merge_maps).unwrap();
    println!("number of unique words: {}", result.len());

    println!("count finished {}", now.elapsed().as_secs_f32());
}
// output:
// number of unique words: 101703
// count finished 0.75196064

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

fn read_and_count(chan_path_r: Receiver<PathBuf>, chan_map_s: Sender<Map>) {
    let mut map = Map::new();
    while let Ok(path) = chan_path_r.recv() {
        let contents = fs::read_to_string(path).expect("file reading error");
        let map2 = word_count(contents.as_ref());
        map = merge_maps(map, map2);
    }
    chan_map_s.send(map).unwrap();
    drop(chan_map_s); // end of sending, close the channel
}

fn merge_maps(mut a: Map, b: Map) -> Map {
    for (word, count) in b {
        *a.entry(word).or_insert(0) += count
    }
    a
}

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), count)
}

fn count(mut map: HashMap<String, u32>, word: String) -> HashMap<String, u32> {
    {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
}
