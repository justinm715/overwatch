// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::time::UNIX_EPOCH;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileNode {
    id: String,
    name: String,
    is_dir: bool,
    size: u64,
    file_type: String,
    last_modified: u64,
    children: Vec<String>,
}

#[tauri::command]
fn scan_directory(path: String) -> Result<HashMap<String, FileNode>, String> {
    let mut map = HashMap::new();
    let root_path = std::path::Path::new(&path);
    if !root_path.exists() { return Err("Path not found".into()); }
    scan_recursive(root_path, &mut map)?;
    Ok(map)
}

#[tauri::command]
fn get_recent_files(path: String, limit: usize) -> Result<Vec<FileNode>, String> {
    let mut map = HashMap::new();
    let root_path = std::path::Path::new(&path);
    if !root_path.exists() { return Err("Path not found".into()); }
    
    // We scan to get all nested files
    scan_recursive(root_path, &mut map)?;
    
    let mut all_nodes: Vec<FileNode> = map.into_values().collect();
    // Sort newest first
    all_nodes.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    
    Ok(all_nodes.into_iter().take(limit).collect())
}

fn scan_recursive(path: &std::path::Path, map: &mut HashMap<String, FileNode>) -> Result<(), String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let id = path.to_string_lossy().to_string();
    let mut children = Vec::new();

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let child_path = entry.path();
                children.push(child_path.to_string_lossy().to_string());
                let _ = scan_recursive(&child_path, map);
            }
        }
    }

    map.insert(id.clone(), FileNode {
        id: id.clone(),
        name: path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or(id),
        is_dir: path.is_dir(),
        size: metadata.len(),
        file_type: if path.is_dir() { "Folder".into() } else { 
            path.extension().map(|e| e.to_string_lossy().to_uppercase()).unwrap_or("FILE".into()) 
        },
        last_modified: metadata.modified().unwrap_or(UNIX_EPOCH)
            .duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64,
        children,
    });
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_directory, get_recent_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}