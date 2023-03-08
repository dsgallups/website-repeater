#![allow(dead_code, unused_variables)]
#![feature(file_create_new)]
use std::io;
use std::path::PathBuf;
use workspace::Workspace;
mod duplicate_site;
mod workspace;

const NEW_PROMPT_SPACING: &str = "\n\n\n";

fn main() {
    loop {
        let main_menu_selection = main_menu();
        print!("{}", NEW_PROMPT_SPACING);

        let mut workspace = match main_menu_selection {
            1 => create_workspace(),
            2 => match open_existing_workspace() {
                Some(workspace) => workspace,
                None => {
                    print!("{}", NEW_PROMPT_SPACING);
                    continue;
                }
            },
            3 => {
                println!("Quitting...");
                break;
            }
            _ => {
                continue;
            }
        };
        print!("{}", NEW_PROMPT_SPACING);

        //Now that the workspace is open, we provide them with a menu of options
        println!("Workspace Loaded!");
        //Provide them with a menu of options. However, this should be managed in the workspace
        if workspace.url.is_none() {
            //The workspace should ask the user if they want to set a URL
            println!("Looks like you haven't set a URL yet! Would you like to do so now? (y/N)");
            let mut selection = String::new();
            if io::stdin().read_line(&mut selection).is_ok() {
                if selection.trim().to_lowercase() == "y" {
                    workspace.modify_url();
                    workspace.save();
                }
            }
        }
        workspace.enter_workspace();
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

                let mut new_workspace = match Workspace::new(&path) {
                    Ok(workspace) => workspace,
                    Err(e) => {
                        print!("Error! ({:?})!{}", e, NEW_PROMPT_SPACING);
                        continue 'create_workspace;
                    }
                };

                println!("Would you like to scrape a URL now? [y/N]");

                loop {
                    let mut scrape_now = String::new();
                    if io::stdin().read_line(&mut scrape_now).is_ok() {
                        match scrape_now.trim().to_ascii_lowercase().as_str() {
                            "y" => new_workspace.modify_url(),
                            &_ => {}
                        }
                        break;
                    }
                }

                new_workspace.save();
                return new_workspace;
            }
            Err(e) => print!("Invalid Input! ({:?}){}", e, NEW_PROMPT_SPACING),
        }
    }
}
