// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{read_dir, self}, io, path::Path};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn get_files(dir: &str) -> Vec<String> {
    let entries = match fs::read_dir(dir){
        Ok(entries)=>entries,
        Err(_error)=> return vec![]
    };
    let res = entries.filter_map(|entry| {
        entry.ok().and_then(|e| {
            if e.path().is_dir() {
                Some(get_files(e.path().to_str().unwrap()))
            } else {
                Some(vec![e.path().to_str().unwrap().to_owned()])
            }
        })
    })
    .flatten()
    .filter(|name| name.ends_with(".png"))
    .collect();
    res
}


#[tauri::command]
fn scan(name: &str) -> Vec<String> {
    get_files(name)
}


#[tauri::command]
fn remove_file(str: &str) -> String {
    let file_names: Vec<&str> = str.split(',').collect();
    for file_name in file_names {
        if let Err(err) = fs::remove_file(file_name) {

        }
    }
    format!("delete successfully")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,scan,remove_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

