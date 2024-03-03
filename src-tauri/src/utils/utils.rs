#[tauri::command]
pub fn read_file(filename: String) -> tauri::Result<String> {
    let contents = std::fs::read_to_string(filename)?;
    Ok(contents)
}