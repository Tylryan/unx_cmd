
#![allow(dead_code)]

use std::fs::DirBuilder;
// use std::io::prelude::*;  // Read to string
use std::path::Path;
use regex::Regex;
extern crate glob;
use self::glob::glob;
use std::fs::File;


pub fn pwd() -> String
{ 
    return std::env::current_dir()
                        .unwrap()
                        .display()
                        .to_string()
}

pub fn touch(path: &Path) -> Result<File, std::io::Error>
{
    std::fs::File::create(path)
}

pub fn cd(new_dir: &str) -> Result<(), std::io::Error>
{
    std::env::current_dir()
        .unwrap()
        .display()
        .to_string();
    return std::env::set_current_dir(new_dir);
}

pub fn rm(file: &Path) -> Result<(), std::io::Error>
{ 
    return std::fs::remove_file(file);
}

pub fn cp(from: &Path, to: &Path) -> Result<u64, std::io::Error>
{ 
    return std::fs::copy(from, to)
}

pub fn rename(from: &Path, to: &Path) -> Result<(), std::io::Error>
{ 
    return std::fs::rename(from, to);
}

pub fn mkdir(dir_name: &Path) -> Result<(), std::io::Error>
{
    let mut dir  = DirBuilder::new();
    dir.recursive(true); // Idk how to use this but it exists
    return dir.create(dir_name);
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

fn cut(text: String, delim: &str, field: usize) -> Option<Vec<String>>
{
    let output = text
        .lines()
        .map(|x| x
             .split(delim)
             .collect::<Vec<&str>>()
             .get(field)
             .unwrap()
             .to_string())
        .collect::<Vec<String>>();
    return Some(output);
}

// Not a unix command, but super helpful
pub fn has_match(text: &str, regex: &str) -> bool
{
    let re = Regex::new(regex).unwrap();
    let found_match: bool = re.is_match(text);

    return found_match;
}

