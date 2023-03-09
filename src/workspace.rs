use crate::NEW_PROMPT_SPACING;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs, io};
use suckit::args::Args;
use suckit::scraper::Scraper;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub url: Option<Url>,
    path: PathBuf,
}

impl Workspace {
    //make sure that the path is a directory
    pub fn new(path: &PathBuf) -> Result<Workspace, io::Error> {
        std::fs::create_dir_all(&path)?;
        let mut serialized_workspace_file = File::create_new(path.join("workspace.json"))?;

        let workspace = Workspace {
            url: None,
            path: fs::canonicalize(path.clone()).unwrap(),
        };

        let serialized_workspace = serde_json::to_string(&workspace)?;
        serialized_workspace_file.write_all(serialized_workspace.as_bytes())?;

        Ok(workspace)
    }

    pub fn from(path: PathBuf) -> Result<Workspace, io::Error> {
        //Try to open up the workspace folder
        //If it doesn't exist, return an error
        let path = fs::canonicalize(path)?;
        let mut serialized_workspace_file = File::open(path.join("workspace.json"))?;
        let mut serialized_workspace = String::new();
        serialized_workspace_file.read_to_string(&mut serialized_workspace)?;

        let workspace: Workspace = serde_json::from_str(&serialized_workspace)?;

        Ok(workspace)
    }

    //we should deal with errors here because save will be called a lot
    pub fn save(&self) {
        //Save the workspace to the workspace folder
        //if it doesn't exist
        let mut file = match File::create(self.path.join("workspace.json")) {
            Ok(file) => file,
            Err(e) => {
                println!(
                    "Error: Saving workspace failed! [opening workspace.json] ({:?})",
                    e
                );
                return;
            }
        };
        let serialized_self = match serde_json::to_string(&self) {
            Ok(serialized) => serialized,
            Err(e) => {
                println!(
                    "Error: Saving workspace failed! [serializing workspace] ({:?})",
                    e
                );
                return;
            }
        };
        match file.write_all(serialized_self.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "Error: Saving workspace failed! [writing workspace to file] ({:?})",
                    e
                );
                return;
            }
        };
    }

    pub fn print_info(&self) {
        println!("Workspace Info:");
        println!("URL: {:?}", self.url);
    }

    pub fn enter_workspace(&mut self) {
        loop {
            match self.print_menu() {
                1 => {
                    if let None = &self.url {
                        println!("Error: No URL set! Please set a URL first.");
                        continue;
                    }
                    let url = self.url.as_mut().unwrap();

                    //Duplicate the site and save to workspace
                    self.clone_website();
                    //Host the website
                    //Open the website in our own browser
                }
                2 => {
                    let mut url = String::new();
                    println!("Enter new URL:");
                    if io::stdin().read_line(&mut url).is_ok() {
                        match Url::parse(&url) {
                            Ok(url) => {
                                self.url = Some(url);
                            }
                            Err(e) => {
                                println!("Error Parsing Inputted URL: {}", e);
                            }
                        }
                    }
                    self.save();
                }
                3 => {
                    self.print_info();
                }
                _ => {
                    println!("Exiting Workspace...");
                    return;
                }
            }
            print!("{}", NEW_PROMPT_SPACING);
        }
    }

    fn clone_website(&mut self) {
        //Run this within our app, not in a separate process
        println!("self path: {:?}", self.path);
        let path_to_save = self.path.join("static");
        println!("path to save: {:?}", path_to_save);
        let args = Args {
            origin: self.url.as_ref().unwrap().clone(),
            output: Some(self.path.join("static")),
            jobs: num_cpus::get(),
            depth: 1,
            ext_depth: 0,
            tries: 20,
            verbose: true,
            delay: 3,
            random_range: 3,
            user_agent: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"),
            include_visit: Regex::new(".*").unwrap(),
            exclude_visit: Regex::new("$^").unwrap(),
            include_download: Regex::new(".*").unwrap(),
            exclude_download: Regex::new("$^").unwrap(),
            visit_filter_is_download_filter: false,
            auth: Vec::new(),
            continue_on_error: false,
            dry_run: false,
        };
        let mut scraper = Scraper::new(args);

        scraper.run();
        println!("Done cloning website!");
    }
    fn print_menu(&self) -> u8 {
        loop {
            println!("1. Easy Repeat Site\n2. Change URL\n3. Print Info\n4. Exit");
            let mut selection = String::new();

            if io::stdin().read_line(&mut selection).is_ok() {
                if let Ok(num) = selection.trim().parse::<u8>() {
                    return num;
                }
            }
        }
    }

    pub fn modify_url(&mut self) {
        loop {
            println!("Enter the URL of the website you want to duplicate:");
            let mut url = String::new();

            match io::stdin().read_line(&mut url) {
                Ok(_) => {
                    match Url::parse(url.trim()) {
                        Ok(url) => {
                            self.url = Some(url);
                            break;
                        }
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

    pub fn get_url() -> Url {
        loop {
            println!("Enter the URL of the website you want to duplicate:");
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
}
