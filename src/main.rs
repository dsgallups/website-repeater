#![allow(dead_code, unused_variables)]
#![feature(file_create_new)]

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
    //make sure that the path is a directory
    fn new(path: &PathBuf) -> Result<Workspace, io::Error> {
        std::fs::create_dir_all(&path)?;
        let mut serialized_workspace_file = File::create_new(path.join("workspace.json"))?;
        let workspace = Workspace { url: None };
        let serialized_workspace = serde_json::to_string(&workspace)?;
        serialized_workspace_file.write_all(serialized_workspace.as_bytes())?;
        Ok(workspace)
    }
    //make sure that the path is a directory
    fn new_with_url(path: &PathBuf, url: Url) -> Result<Workspace, io::Error> {
        std::fs::create_dir_all(&path)?;
        let mut serialized_workspace_file = File::create_new(path.join("workspace.json"))?;
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
                let workspace = create_workspace();
                print!("{}", NEW_PROMPT_SPACING);
            }
            2 => {
                let workspace = match open_existing_workspace() {
                    Some(workspace) => workspace,
                    None => {
                        print!("{}", NEW_PROMPT_SPACING);
                        continue;
                    }
                };
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
/*
 * Returns a workspace if one exists, otherwise returns None
*/
fn open_existing_workspace() -> Option<Workspace> {
    loop {
        println!(
            "Enter the directory of the workspace to open (default: \"./repeater_workspace/\"):"
        );
        let default_dir = String::from("./repeater_workspace/");
        let mut working_dir = String::new();

        match io::stdin().read_line(&mut working_dir) {
            Ok(_) => {
                let path: PathBuf = if working_dir.trim().is_empty() {
                    PathBuf::from(default_dir.trim())
                } else {
                    let new_path = PathBuf::from(working_dir.trim());
                    let new_directory = format!("./{}/", new_path.to_str().unwrap());
                    PathBuf::from(new_directory)
                };

                println!("Path: {:?}", &path);

                match Workspace::from(path) {
                    Ok(workspace) => return Some(workspace),
                    Err(e) => {
                        println!("Error opening workspace! ({:?}){}", e, NEW_PROMPT_SPACING);
                        println!("Try again? (y/n)");
                        let mut try_again = String::new();
                        match io::stdin().read_line(&mut try_again) {
                            Ok(_) => {
                                if try_again.trim().to_lowercase() == "y" {
                                    continue;
                                } else {
                                    return None;
                                }
                            }
                            Err(e) => {
                                println!("Error reading input! ({:?})", e);
                                return None;
                            }
                        }
                    }
                };
            }
            Err(e) => print!("Invalid Path! ({:?}){}", e, NEW_PROMPT_SPACING),
        }
    }
}
fn create_workspace() -> Workspace {
    'create_workspace: loop {
        println!(
            "Enter the new directory to save this workspace (default: \"./repeater_workspace/\"):"
        );
        let default_dir = String::from("./repeater_workspace/");
        let mut working_dir = String::new();

        match io::stdin().read_line(&mut working_dir) {
            Ok(_) => {
                let path: PathBuf = if working_dir.trim().is_empty() {
                    PathBuf::from(default_dir.trim())
                } else {
                    let new_path = PathBuf::from(working_dir.trim());
                    let new_directory = format!("./{}/", new_path.to_str().unwrap());
                    PathBuf::from(new_directory)
                };

                println!("Path: {:?}", &path);
                println!("Would you like to scrape a URL now? [y/N]");

                'scrape_url: loop {
                    let mut scrape_now = String::new();
                    if io::stdin().read_line(&mut scrape_now).is_ok() {
                        match scrape_now.trim().to_ascii_lowercase().as_str() {
                            "y" => {
                                let url = get_url();
                                match Workspace::new_with_url(&path, url) {
                                    Ok(workspace) => return workspace,
                                    Err(e) => {
                                        print!("Error! ({:?})!{}", e, NEW_PROMPT_SPACING);
                                        continue 'create_workspace;
                                    }
                                };
                            }
                            &_ => {
                                match Workspace::new(&path) {
                                    Ok(workspace) => return workspace,
                                    Err(e) => {
                                        print!("Error! ({:?})!{}", e, NEW_PROMPT_SPACING);
                                        continue 'create_workspace;
                                    }
                                };
                            }
                        }
                    }
                }
            }
            Err(e) => print!("Invalid Path! ({:?}){}", e, NEW_PROMPT_SPACING),
        }
    }
}
