use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use regex::Regex;

fn main() {
  // Args
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  // Regix
  let re_joined_room = Regex::new(r"([0-9\.]+ [0-9:]+).+Joining or Creating Room: (.+)").unwrap();
  let re_player_joined = Regex::new(r"([0-9\.]+ [0-9:]+).+OnPlayerJoined (.+)").unwrap();
  let re_player_left = Regex::new(r"([0-9\.]+ [0-9:]+).+OnPlayerLeft (.+)").unwrap();

  // File
  let in_f = File::open(filename).expect("file not found");
  let reader = BufReader::new(in_f);
  let mut out_f = OpenOptions::new().create(true).append(true).open("VRChat_usrlog.txt").unwrap();

  // Result
  let mut result = String::new();

  for line in reader.lines() {
    let line = line.unwrap();
    if re_joined_room.is_match(&line) {
      let caps = re_joined_room.captures(&line).unwrap();
      result.push_str(format!("{} Joined World: {}\n", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()).as_str());
    } else if re_player_joined.is_match(&line) {
      let caps = re_player_joined.captures(&line).unwrap();
      result.push_str(format!("{}   Joined User: {}\n", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()).as_str());
    } else if re_player_left.is_match(&line) {
      let caps = re_player_left.captures(&line).unwrap();
      result.push_str(format!("{}   Left User: {}\n", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()).as_str());
    }
  }
  out_f.write_all(result.as_bytes()).unwrap();
}
