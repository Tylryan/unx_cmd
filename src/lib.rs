use std::fs::DirBuilder;
// use std::io::prelude::*;  // Read to string
use std::path::Path;
use regex::Regex;
extern crate glob;
use self::glob::glob;

pub fn pwd() -> String
{ 
    return std::env::current_dir()
                        .unwrap()
                        .display()
                        .to_string()
}

pub fn touch(path: &Path)
{
    std::fs::File::create(path).expect("Could not create file");
}

pub fn cd(new_dir: &str)
{
    std::env::current_dir()
        .unwrap()
        .display()
        .to_string();
    std::env::set_current_dir(new_dir).unwrap();
}

pub fn rm(file: &Path) { std::fs::remove_file(file).unwrap(); }

pub fn cp(from: &Path, to: &Path) { std::fs::copy(from, to).unwrap(); }

pub fn rename(from: &Path, to: &Path) { std::fs::rename(from, to).unwrap(); }

pub fn mkdir(dir_name: &Path)
{
    let mut dir  = DirBuilder::new();
    dir.recursive(true); // Idk how to use this but it exists
    dir.create(dir_name).unwrap();
}

pub fn ls(path: &Path) -> Vec<String>
{
    // let dir_contents: Vec<String> = Vec::new();

    let glob_dir = path
        .display()
        .to_string() + "/*";

    let globbed_dir_contents = glob(&glob_dir)
        .expect("Directory Not Found")
        .into_iter();

    let dir_contents: Vec<String> = globbed_dir_contents
        .map(|x| x
             .unwrap()
             .display()
             .to_string()
             )
        .collect::<Vec<String>>();

    return dir_contents;
}
pub fn grep(text: &str, regex:&str) -> String
{
    let array_lines = text
        .split("\n")                    // Array of lines
        .into_iter()
        .filter(|x| has_match(x, regex)) // return only lines where regex match exists
        .collect::<Vec<&str>>()          // Turn it back to a Vector
        .join("\n");                     // Turn it back to a string.

    return array_lines;
}

// Removes and Replaces
pub fn sed_g(text: &str, old:&str, new:&str) -> String 
{
    let mut replaced_text = String::new();
    if has_match(text, old)
    {
        let re = Regex::new(old).unwrap();
        replaced_text = re.replace_all(text, new).to_string();
    }

    return replaced_text;

}

// Not a unix command, but super helpful
pub fn has_match(text: &str, regex: &str) -> bool
{
    let re = Regex::new(regex).unwrap();
    let found_match: bool = re.is_match(text);

    return found_match;
}
