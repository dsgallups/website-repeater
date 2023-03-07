use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
use url::{ParseError, Url};
mod duplicate_site;

const NEW_PROMPT_SPACING: &str = "\n\n\n";

#[derive(Serialize, Deserialize)]
struct Workspace {
    url: Url,
    path: PathBuf,
    url_duplicated: bool,
}

impl Workspace {
    fn new(url: Url, path: PathBuf) -> Self {
        Self {
            url,
            path,
            url_duplicated: false,
        }
    }

    fn duplicate_url(&mut self) -> Result<(), ParseError> {
        duplicate_site::duplicate_site(self.url.clone())
    }
}

fn main() {
    loop {
        let main_menu_selection = main_menu();
        print!("{}", NEW_PROMPT_SPACING);

        match main_menu_selection {
            1 => {
                let mut url = get_url();
                print!("{}", NEW_PROMPT_SPACING);
                let mut path = get_path();
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

        match io::stdin().read_line(&mut selection) {
            Ok(_) => {
                match selection.trim().parse::<u8>() {
                    Ok(num) => {
                        if (1..=3).contains(&num) {
                            return num;
                        }
                    }
                    Err(_) => {
                        print!("Invalid selection!{}", NEW_PROMPT_SPACING);
                        continue;
                    }
                };
            }
            Err(_) => {}
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
                        print!("Invalid URL! ({:?}){}", e, NEW_PROMPT_SPACING);
                        continue;
                    }
                };
            }
            Err(e) => print!("Invalid URL ({:?})!{}", e, NEW_PROMPT_SPACING),
        }
    }
}

fn get_path() -> PathBuf {
    loop {
        println!("Enter the directory to save this workspace (default: \"./\"):");
        let mut working_dir = String::new();

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
