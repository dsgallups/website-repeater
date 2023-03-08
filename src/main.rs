#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs, io};
use url::Url;
mod duplicate_site;

const NEW_PROMPT_SPACING: &str = "\n\n\n";

#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
    url: Option<Url>,
}

impl Workspace {
    fn new(path: PathBuf) -> Result<Workspace, io::Error> {
        let path = fs::canonicalize(path)?;
        let mut serialized_workspace_file = File::create(path.join("workspace.json"))?;
        let workspace = Workspace { url: None };
        let serialized_workspace = serde_json::to_string(&workspace)?;
        serialized_workspace_file.write_all(serialized_workspace.as_bytes())?;
        Ok(workspace)
    }
    fn new_with_url(url: Url, path: PathBuf) -> Result<Workspace, io::Error> {
        let path = fs::canonicalize(path)?;
        let mut serialized_workspace_file = File::create(path.join("workspace.json"))?;
        let workspace = Workspace { url: Some(url) };
        let serialized_workspace = serde_json::to_string(&workspace)?;
        serialized_workspace_file.write_all(serialized_workspace.as_bytes())?;
        Ok(workspace)
    }
    fn from(path: PathBuf) -> Result<Workspace, io::Error> {
        //Try to open up the workspace folder
        //If it doesn't exist, return an error
        let path = fs::canonicalize(path)?;
        let mut serialized_workspace_file = File::open(path.join("workspace.json"))?;
        let mut serialized_workspace = String::new();
        serialized_workspace_file.read_to_string(&mut serialized_workspace)?;

        let workspace: Workspace = serde_json::from_str(&serialized_workspace)?;

        Ok(workspace)
    }

    fn save(&self) -> Result<(), io::Error> {
        //Save the workspace to the workspace folder
        //if it doesn't exist
        let mut file = File::open("workspace.json")?;
        let serialized_self = serde_json::to_string(&self)?;
        file.write_all(serialized_self.as_bytes())?;
        Ok(())
    }
}

fn main() {
    loop {
        let main_menu_selection = main_menu();
        print!("{}", NEW_PROMPT_SPACING);

        match main_menu_selection {
            1 => {
                let path = get_workspace_path();
                let _workspace = Workspace::from(path.clone());
                let _url = get_url();
                print!("{}", NEW_PROMPT_SPACING);
            }
            2 => {
                println!("Quitting...");
                break;
            }
            3 => {
                println!("Quitting...");
                break;
            }
            _ => {}
        }
    }
}

fn main_menu() -> u8 {
    loop {
        println!("1. Easy Setup\n2. Use Existing Workspace\n3. Quit");
        let mut selection = String::new();

        if io::stdin().read_line(&mut selection).is_ok() {
            if let Ok(num) = selection.trim().parse::<u8>() {
                if (1..=3).contains(&num) {
                    return num;
                }
            };
        }
        print!("Invalid selection!{}", NEW_PROMPT_SPACING);
    }
}

fn get_url() -> Url {
    loop {
        println!("Enter the URL of the website you want to scrape:");
        let mut url = String::new();

        match io::stdin().read_line(&mut url) {
            Ok(_) => {
                match Url::parse(url.trim()) {
                    Ok(url) => return url,
                    Err(e) => {
                        print!("Invalid URL! ({:?})!{}", e, NEW_PROMPT_SPACING);
                        continue;
                    }
                };
            }
            Err(e) => print!("Invalid URL ({:?})!{}", e, NEW_PROMPT_SPACING),
        }
    }
}

fn get_workspace_path() -> PathBuf {
    loop {
        println!(
            "Enter the directory to save this workspace (default: \"./repeater_workspace/\"):"
        );
        let mut working_dir = String::new();
        println!("Enter the name of this project (default: \"/repeater_workspace/\"):");

        match io::stdin().read_line(&mut working_dir) {
            Ok(_) => {
                let path = PathBuf::from(working_dir.trim());
                if path.exists() {
                    return path;
                } else {
                    print!("Invalid path!{}", NEW_PROMPT_SPACING);
                    continue;
                }
            }
            Err(e) => print!("Invalid URL ({:?})!{}", e, NEW_PROMPT_SPACING),
        }
    }
}
