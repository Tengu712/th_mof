use super::scenes::game::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec::*,
};

pub struct Dialogue {
    logs: Vec<LogSection>,
}

struct LogSection {
    mode: Mode,
    strs: Vec<(String, bool)>,
}

impl Dialogue {
    /// Constructor.
    pub fn new(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| e.to_string() + "\nFailed to open. : " + path)?;
        let lines = BufReader::new(file)
            .lines()
            .filter_map(|n| n.ok())
            .collect::<Vec<String>>()
            .into_iter()
            .filter_map(|n| if n.len() > 0 { Some(n) } else { None })
            .collect::<Vec<String>>();
        let logs = Dialogue::get_logs(&lines, 0, Vec::new())?;
        Ok(Self { logs })
    }
    /// Private function. Get sections by recursing.
    fn get_logs(
        lines: &Vec<String>,
        idx: usize,
        logs: Vec<LogSection>,
    ) -> Result<Vec<LogSection>, String> {
        if idx >= lines.len() {
            return Ok(logs);
        }
        let words = lines[idx].split(' ').collect::<Vec<&str>>();
        if words.len() < 2 || words[0] != "#" {
            return Err(lines[idx].clone() + "\nSyntax error.");
        }
        let mode = if words[1] == "story" && words.len() >= 3 {
            let num = words[2]
                .parse()
                .map_err(|_| format!("{} : {}", "Syntax error. It must be a number.", words[2]))?;
            Mode::Story(num)
        } else {
            return Err(lines[idx].clone() + "\nSyntax error. Couldn't identify mode.");
        };
        let (next_idx, strs) = Dialogue::get_strs(lines, idx + 1, Vec::new())?;
        let mut logs = logs;
        logs.push(LogSection { mode, strs });
        Dialogue::get_logs(lines, next_idx, logs)
    }
    /// Private function. Get speechies in each section.
    fn get_strs(
        lines: &Vec<String>,
        idx: usize,
        strs: Vec<(String, bool)>,
    ) -> Result<(usize, Vec<(String, bool)>), String> {
        if idx >= lines.len() {
            Ok((idx, strs))
        } else {
            let words = lines[idx].split(' ').collect::<Vec<&str>>();
            if words.len() != 0 && words[0] == "#" {
                Ok((idx, strs))
            } else if words.len() >= 2 {
                let mut strs = strs;
                strs.push((words[1].to_string(), words[0] == "r"));
                Dialogue::get_strs(lines, idx + 1, strs)
            } else {
                Err(lines[idx].clone() + "\nSyntax error. Couldn't identify l or r.")
            }
        }
    }
    /// A method to get dialogue based on mode and count.
    pub fn get_dialogue(&self, mode: &Mode, count: &u32) -> Option<(String, bool)> {
        let count = count.clone() as usize;
        for i in self.logs.iter() {
            if mode == &i.mode && count < i.strs.len() {
                return Some(i.strs[count].clone());
            }
        }
        None
    }
    /// A method to get dialogue len based on mode.
    pub fn get_dialogue_len(&self, mode: &Mode) -> usize {
        for i in self.logs.iter() {
            if mode == &i.mode {
                return i.strs.len();
            }
        }
        0
    }
}
