use crate::models::{self, AppSettings}; // Added AppSettings here
use serde_json;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

// Helper struct for JSON serialization of notes
#[derive(serde::Serialize, serde::Deserialize)]
struct NoteData {
    notes: String,
}

/// Saves an employee's note to a JSON file.
/// The note is stored in a structure: {"notes": "content..."}
///
/// # Arguments
/// * `notes_dir` - The directory where notes should be stored (e.g., `<storage_path>/notes/`).
/// * `employee_id` - The unique identifier for the employee.
/// * `note_content` - The content of the note to save.
///
/// # Returns
/// `Ok(())` on success, or an error message `String` on failure.
pub fn save_employee_note(
    notes_dir: &Path,
    employee_id: &str,
    note_content: &str,
) -> Result<(), String> {
    if !notes_dir.exists() {
        fs::create_dir_all(notes_dir)
            .map_err(|e| format!("Failed to create notes directory: {}", e))?;
    }

    let note_file_path = notes_dir.join(format!("{}.json", employee_id));
    let note_data = NoteData {
        notes: note_content.to_string(),
    };

    let json_string = serde_json::to_string_pretty(&note_data)
        .map_err(|e| format!("Failed to serialize note to JSON: {}", e))?;

    let mut file = File::create(&note_file_path)
        .map_err(|e| format!("Failed to create note file {:?}: {}", note_file_path, e))?;

    file.write_all(json_string.as_bytes())
        .map_err(|e| format!("Failed to write to note file {:?}: {}", note_file_path, e))?;

    Ok(())
}

/// Loads an employee's note from a JSON file.
/// Expects the note to be stored in a structure: {"notes": "content..."}
///
/// # Arguments
/// * `notes_dir` - The directory where notes are stored.
/// * `employee_id` - The unique identifier for the employee.
///
/// # Returns
/// `Ok(Some(String))` if the note is found and loaded,
/// `Ok(None)` if the note file does not exist,
/// or an error message `String` on other failures.
pub fn load_employee_note(
    notes_dir: &Path,
    employee_id: &str,
) -> Result<Option<String>, String> {
    let note_file_path = notes_dir.join(format!("{}.json", employee_id));

    if !note_file_path.exists() {
        return Ok(None);
    }

    let mut file = File::open(&note_file_path)
        .map_err(|e| format!("Failed to open note file {:?}: {}", note_file_path, e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read note file {:?}: {}", note_file_path, e))?;

    let note_data: NoteData = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to deserialize note from JSON {:?}: {}", note_file_path, e))?;

    Ok(Some(note_data.notes))
}

/// Saves the application settings to a JSON file.
///
/// # Arguments
/// * `settings_file` - The path to the settings JSON file.
/// * `settings` - A reference to the `AppSettings` struct to save.
///
/// # Returns
/// `Ok(())` on success, or an error message `String` on failure.
pub fn save_app_settings(
    settings_file: &Path,
    settings: &models::AppSettings,
) -> Result<(), String> {
    // Ensure parent directory exists
    if let Some(parent_dir) = settings_file.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)
                .map_err(|e| format!("Failed to create parent directory for settings: {}", e))?;
        }
    }

    let json_string = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize AppSettings to JSON: {}", e))?;

    let mut file = File::create(settings_file)
        .map_err(|e| format!("Failed to create settings file {:?}: {}", settings_file, e))?;

    file.write_all(json_string.as_bytes())
        .map_err(|e| format!("Failed to write to settings file {:?}: {}", settings_file, e))?;

    Ok(())
}

/// Loads the application settings from a JSON file.
/// If the file does not exist, returns `AppSettings::default()`.
///
/// # Arguments
/// * `settings_file` - The path to the settings JSON file.
///
/// # Returns
/// `Ok(AppSettings)` on success (either loaded or default),
/// or an error message `String` on failure to read/parse an existing file.
pub fn load_app_settings(settings_file: &Path) -> Result<models::AppSettings, String> {
    if !settings_file.exists() {
        return Ok(models::AppSettings::default());
    }

    let mut file = File::open(settings_file)
        .map_err(|e| format!("Failed to open settings file {:?}: {}", settings_file, e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read settings file {:?}: {}", settings_file, e))?;

    let settings: models::AppSettings = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to deserialize AppSettings from JSON {:?}: {}", settings_file, e))?;

    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AppSettings}; // Employee is not used here directly
    use std::fs as std_fs; // Renamed to avoid conflict with persistence::fs
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_employee_note() {
        let dir = tempdir().expect("Failed to create temp dir");
        let notes_dir = dir.path().join("notes");

        let employee_id = "emp123";
        let note_content = "This is a test note.";

        // Save note
        let save_result = save_employee_note(&notes_dir, employee_id, note_content);
        assert!(save_result.is_ok(), "Failed to save note: {:?}", save_result.err());

        // Assert file creation
        let note_file_path = notes_dir.join(format!("{}.json", employee_id));
        assert!(note_file_path.exists(), "Note file was not created");

        // Load note
        let load_result = load_employee_note(&notes_dir, employee_id);
        assert!(load_result.is_ok(), "Failed to load note: {:?}", load_result.err());
        assert_eq!(load_result.unwrap(), Some(note_content.to_string()));

        // Test loading non-existent note
        let non_existent_load_result = load_employee_note(&notes_dir, "non_existent_emp");
        assert!(non_existent_load_result.is_ok(), "Error when loading non-existent note: {:?}", non_existent_load_result.err());
        assert_eq!(non_existent_load_result.unwrap(), None);
    }

    #[test]
    fn test_save_and_load_app_settings() {
        let dir = tempdir().expect("Failed to create temp dir");
        let settings_file_path = dir.path().join("settings.json");

        let mut settings = AppSettings::default();
        settings.theme_preference = "dark".to_string();
        settings.auto_save_enabled = true;
        settings.department_colors.insert("Engineering".to_string(), "#FF0000".to_string());

        // Save settings
        let save_result = save_app_settings(&settings_file_path, &settings);
        assert!(save_result.is_ok(), "Failed to save settings: {:?}", save_result.err());

        // Assert file creation
        assert!(settings_file_path.exists(), "Settings file was not created");

        // Load settings
        let load_result = load_app_settings(&settings_file_path);
        assert!(load_result.is_ok(), "Failed to load settings: {:?}", load_result.err());
        assert_eq!(load_result.unwrap(), settings);
    }

    #[test]
    fn test_load_default_app_settings_if_file_not_exists() {
        let dir = tempdir().expect("Failed to create temp dir");
        let non_existent_settings_file = dir.path().join("non_existent_settings.json");

        // Load settings from non-existent file
        let load_result = load_app_settings(&non_existent_settings_file);
        assert!(load_result.is_ok(), "Failed to load default settings: {:?}", load_result.err());
        assert_eq!(load_result.unwrap(), AppSettings::default());
    }

    #[test]
    fn test_save_note_creates_directory() {
        let dir = tempdir().expect("Failed to create temp dir");
        // notes_dir itself does not exist yet, save_employee_note should create it.
        let notes_dir = dir.path().join("new_notes_dir"); 
        
        let employee_id = "emp001";
        let note_content = "Test content.";

        let save_result = save_employee_note(&notes_dir, employee_id, note_content);
        assert!(save_result.is_ok(), "Failed to save note: {:?}", save_result.err());
        assert!(notes_dir.exists(), "Notes directory was not created by save_employee_note");
        assert!(notes_dir.join(format!("{}.json", employee_id)).exists(), "Note file was not created in new directory");
    }

     #[test]
    fn test_save_settings_creates_parent_directory() {
        let dir = tempdir().expect("Failed to create temp dir");
        // settings_file is in a subdirectory that doesn't exist yet
        let parent_dir = dir.path().join("new_config_parent");
        let settings_file = parent_dir.join("settings.json"); 
        
        let settings = AppSettings::default();

        let save_result = save_app_settings(&settings_file, &settings);
        assert!(save_result.is_ok(), "Failed to save settings: {:?}", save_result.err());
        assert!(parent_dir.exists(), "Parent directory for settings was not created.");
        assert!(settings_file.exists(), "Settings file was not created in new parent directory.");
    }
}
