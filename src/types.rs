use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs, io};
use url::Url;
#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    url: Option<Url>,
}

impl Workspace {
    //make sure that the path is a directory
    pub fn new(path: &PathBuf) -> Result<Workspace, io::Error> {
        std::fs::create_dir_all(&path)?;
        let mut serialized_workspace_file = File::create_new(path.join("workspace.json"))?;
        let workspace = Workspace { url: None };
        let serialized_workspace = serde_json::to_string(&workspace)?;
        serialized_workspace_file.write_all(serialized_workspace.as_bytes())?;
        Ok(workspace)
    }
    //make sure that the path is a directory
    pub fn new_with_url(path: &PathBuf, url: Url) -> Result<Workspace, io::Error> {
        std::fs::create_dir_all(&path)?;
        let mut serialized_workspace_file = File::create_new(path.join("workspace.json"))?;
        let workspace = Workspace { url: Some(url) };
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

    pub fn save(&self) -> Result<(), io::Error> {
        //Save the workspace to the workspace folder
        //if it doesn't exist
        let mut file = File::open("workspace.json")?;
        let serialized_self = serde_json::to_string(&self)?;
        file.write_all(serialized_self.as_bytes())?;
        Ok(())
    }
}
