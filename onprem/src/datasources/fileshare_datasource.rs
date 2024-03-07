use walkdir::WalkDir;
use std::collections::HashSet;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::Metadata;
use tokio_util::sync::CancellationToken;
use super::super::on_prem::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::io::{self, BufRead};


pub fn explore_location(request: ExploreRequest, token: CancellationToken) -> Result<ExploreOutput, String> {
    let walk_dir = WalkDir::new(&request.root_path);
    let valid_extensions: HashSet<&str> = request.valid_extensions.iter().map(|x| x.as_str()).collect();

    let locations = find_file_locations(walk_dir, &valid_extensions);

    let response = ExploreOutput {
        file_paths: locations.collect(),
    };
    Ok(response)
}

fn find_file_locations<'a>(walk_dir: WalkDir, valid_extensions: &'a HashSet<&str>) -> impl Iterator<Item=String> + 'a {
    walk_dir.into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|file| is_valid_extension(file.path().extension(), valid_extensions))
        .filter(|file| file.path().to_str().is_some())
        .map(|file| file.path().to_str().unwrap().to_string())
}

fn is_valid_extension<'a, 'b>(location: Option<&OsStr>, valid_extensions: &'a HashSet<&'b str>) -> bool {
    match location {
        None => false,
        Some(l) => {
            let s = l.to_str().unwrap();
            valid_extensions.contains(s)
        }
    }
}

pub fn analyze_files<'a>(request: AnalysisRequest, token: CancellationToken) -> Result<Vec<String>, String> {
    unimplemented!();
    /*

    let results = request.file_paths.into_iter().map(|file_path| {
        let file = File::open(&file_path)?;
        io::Read::bytes(file).

        //let path = Path::new(&file_path);
        //let mut file = match File::open(&path) {
        //    Err(why) => panic!("couldn't open {}: {}", file_path, why),
        //    Ok(file) => file,
        //};

        //// Read the file contents into a string, returns `io::Result<usize>`
        //let mut content = String::new();
        //match file.read_to_string(&mut content) {
        //    Err(why) => panic!("couldn't read {}: {}", file_path, why),
        //    Ok(_) => print!("{} contains:\n{}", file_path, content),
        //}
    });
    Ok(results.collect())

    //potentially we need those folder mappings, if that is still quite used...

    //get file with all information...
    //meta data...

    */
}

pub fn check_for_deleted_files(request: CheckForDeletedFilesRequest, token: CancellationToken) -> Result<Vec<String>, String> {
    let results = request.files.into_iter()
        .map(|f| {
            let path = Path::new(&f.file_path);
    
            let fp = f.file_path.to_owned();
            let file = match File::open(&path) {
                Err(why) => (f, Err(why)),
                Ok(file) => (f, Ok(file)),
            };
            file
        });
        

    //TODO only for testing - however we might need some way of passing on the errors or something...
    for (file, result) in results.to_owned() {
        let fp = file.file_path.to_owned();
        match result {
            Err(e) => println!("couldn't open {fp:?}: {e:?}"),
            Ok(m) => println!("file found: {fp:?}"),
        }
    }

    let files_not_found = results.filter(|(_, result)| match result {
        Ok(m) => false,
        Err(e) => match e.raw_os_error() { 
            Some(2) => true, //not sure this is reliable across operating system...
            _ => false
        }, 
    }).map(|(file, _)| file.id);
    //TODO could change this to return either Ok(deleted), Ok(file found) or Err(error message)
    Ok(files_not_found.collect())
}

//delete files
pub fn delete_files(request: DeleteFilesRequest, token: CancellationToken) {
    unimplemented!();
}
