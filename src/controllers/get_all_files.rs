use std::{error::Error, fs::{metadata, read_dir, DirEntry }, path::PathBuf};

pub fn get_all_files(directory_started: String) -> Result<Vec<String>, Box<dyn Error>> {
    
    let mut recursive_files = Vec::<String>::new();
    
    let paths = read_dir(directory_started)?;

    for path in paths {

        let path_current = path?;
        let is_file = &path_current
            .file_type()?
            .is_file();

        let path_parsed_to_string = path_current
        .path()
        .to_str()
        .ok_or("")?
        .to_string();
     
        if is_file.clone()  {
            recursive_files.push(path_parsed_to_string.clone());
            continue;
        } 
     
        let response_directory = get_all_files(path_parsed_to_string)?;
    
        response_directory
            .iter()
            .for_each(|current_path| {
                recursive_files.push(current_path.clone())
        });

    }
    Ok(recursive_files)
}