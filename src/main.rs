/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/cham
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;
use std::os::unix::process::CommandExt;

fn read_config_file() -> Result<HashMap<String, String>, String> {
    let home = env::var("HOME").map_err(|_| "$HOME is not set".to_string())?;
    let filename = format!("{}/.config/cham.conf", home);
    let file = File::open(filename.as_str()).map_err(|_| format!("can not open file: {}", filename))?;
    let file = BufReader::new(file);

    let mut result = HashMap::new();

    for line in file.lines() {
        let line = line.map_err(|e| format!("{:?}", e))?;
        let data: Vec<_> = line.split("=>")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if data.len() == 0 {
            continue;
        } else if data.len() != 2 {
            Err(format!("invalid line: {} :: {}", line, data.len()))?;
        } else {
            result.insert(data[0].to_string(), data[1].to_string());
        }
    }
    Ok(result)
}

fn search_project_type(config: HashMap<String, String>, path: &Path) -> Result<String, String> {
    for entry in fs::read_dir(path).map_err(|e |format!("can not read directory: {:?}", e))? {
        let entry = entry.map_err(|e| format!("{:?}", e))?;
        let path = entry.path();
        let filename = match path.file_name() {
            Some(filename) => filename.to_str().ok_or(format!("invalid Unicode!"))?.to_string(),
            None => continue,
        };
        if let Some(cmd) = config.get(&filename) {
            return Ok(cmd.clone());
        }
    }
    match path.parent() {
        Some(path) => search_project_type(config, path),
        None => Err(format!("not found any project file : {:?}", path))
    }
}

fn main() {
    let config = read_config_file().unwrap();
    let command = search_project_type(config, &Path::new(&env::current_dir().expect("can not find current directory"))).unwrap();
    let error = Command::new(command.as_str())
        .args(env::args().skip(1))
        .exec();
    panic!("something wrong!! {}", error);
}
