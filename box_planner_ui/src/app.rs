use crate::messages::{Message, TabId};
use crate::views::view_app;
// Corrected to import the function that expects a Reader
use box_planner_core::csv_processing::import_employees_from_csv; 
use box_planner_core::models::{AppSettings, Employee, GridState, Skill, get_predefined_skills}; // Added Skill, get_predefined_skills
use box_planner_core::persistence::{load_app_settings, save_app_settings};
use iced::{Command, Element, Theme}; // Removed Executor and Subscription
use std::fs::File; // Added File
use std::io::BufReader; // Added BufReader
use std::path::Path;
use std::time::Duration; // New import

const SETTINGS_FILE_PATH: &str = "box_planner_ui/app_settings.json";
const SAMPLE_EMPLOYEES_CSV_PATH: &str = "box_planner_ui/sample_employees.csv";

pub struct App {
    pub employees: Vec<Employee>,
    pub grid_state: GridState,
    pub selected_employee_id: Option<String>,
    pub view_scale: f32,
    pub app_settings: AppSettings, // Added app_settings field
    pub expanded_card_id: Option<String>, // New field
    pub dragged_employee_id: Option<String>, // New field
    pub available_skills: Vec<Skill>, // New field
    pub dragged_skill_id: Option<String>, // New field
    pub highlighted_box_id: Option<String>,     // New field
    pub highlighted_employee_id: Option<String>, // New field
    pub active_tab: TabId,
}

impl App {
    pub fn new() -> Self {
        let settings_path = Path::new(SETTINGS_FILE_PATH);
        let mut app_settings = match load_app_settings(settings_path) {
            Ok(settings) => {
                println!("Successfully loaded settings from {:?}", settings_path);
                settings
            }
            Err(e) => {
                eprintln!("Failed to load settings from {:?}: {}. Using defaults and attempting to save.", settings_path, e);
                let default_settings = AppSettings::default();
                if let Err(save_err) = save_app_settings(settings_path, &default_settings) {
                    eprintln!("Failed to save default settings to {:?}: {}", settings_path, save_err);
                } else {
                    println!("Successfully saved default settings to {:?}", settings_path);
                }
                default_settings
            }
        };

        let initial_view_scale = app_settings.view_scale.unwrap_or_else(|| {
            println!("View scale not found in settings, using default 1.0 and updating settings.");
            let default_scale = 1.0;
            app_settings.view_scale = Some(default_scale);
            // Attempt to save the updated settings immediately
            if let Err(save_err) = save_app_settings(settings_path, &app_settings) {
                eprintln!("Failed to save updated settings (with default view_scale) to {:?}: {}", settings_path, save_err);
            }
            default_scale
        });

        let employees_load_result = File::open(SAMPLE_EMPLOYEES_CSV_PATH)
            .map_err(|e| format!("Failed to open CSV file '{}': {}", SAMPLE_EMPLOYEES_CSV_PATH, e))
            .and_then(|file| {
                let reader = BufReader::new(file);
                // Assuming import_employees_from_csv now correctly takes a Read implementor
                // and returns Result<Vec<Employee>, E> where E can be converted to our error string.
                import_employees_from_csv(reader)
                    .map_err(|e| format!("Failed to parse CSV from '{}': {}", SAMPLE_EMPLOYEES_CSV_PATH, e.to_string()))
            });

        let employees = match employees_load_result {
            Ok(loaded_employees) => {
                if loaded_employees.is_empty() {
                    println!("No employees loaded from CSV, using dummy data.");
                    Self::load_dummy_employees() 
                } else {
                    println!("Successfully loaded {} employees from CSV.", loaded_employees.len());
                    loaded_employees
                }
            }
            Err(e) => {
                eprintln!("Error loading employees from CSV: {}. Using dummy data instead.", e);
                Self::load_dummy_employees()
            }
        };
        let mut grid_state = GridState::default();

        // Sample assignments - ensure employees are loaded first
        if !employees.is_empty() {
            grid_state.assignments.insert("1A".to_string(), vec![employees[0].user_id.clone()]);
            if employees.len() > 1 {
                 grid_state.assignments.insert("2B".to_string(), vec![employees[1].user_id.clone()]);
            }
            if employees.len() > 2 {
                grid_state.assignments.insert("1A".to_string(), vec![employees[0].user_id.clone(), employees[2].user_id.clone()]); // Add a second employee to 1A
            }
            if employees.len() > 3 {
                grid_state.assignments.insert("3C".to_string(), vec![employees[3].user_id.clone()]);
            }
        }

        Self {
            employees,
            grid_state,
            selected_employee_id: None,
            view_scale: initial_view_scale, // Use loaded or default scale
            app_settings, // Store loaded/default settings
            expanded_card_id: None, // New
            dragged_employee_id: None, // New
            available_skills: get_predefined_skills(), // Initialize the new field
            dragged_skill_id: None, // New
            highlighted_box_id: None,     // New
            highlighted_employee_id: None, // New
            active_tab: TabId::Employees,
        }
    }

    // Fallback function to load dummy employees
    fn load_dummy_employees() -> Vec<Employee> {
        let all_skills = get_predefined_skills(); // Get all skills
        vec![
            Employee {
                user_id: "1".to_string(),
                pr_group_2025: "GroupA".to_string(),
                first_name: "John (Dummy)".to_string(),
                last_name: "Doe".to_string(),
                current_position: "Developer".to_string(),
                current_temp_position: None,
                pr_2021: None, pr_2022: None, pr_2023: None, pr_2024: Some(4.5),
                user_9box_2024: Some("1A".to_string()), user_9box_2025: None,
                notes: None, current_label: None, email: None, manager_id: None,
                department: None, location: None, hire_date: None, 
                skills: if all_skills.len() >= 2 { // Check if enough skills exist
                            vec![all_skills[0].clone(), all_skills[1].clone()] // Example: assign Rust and GUI Design
                       } else {
                            vec![]
                       },
            },
            Employee {
                user_id: "2".to_string(),
                pr_group_2025: "GroupB".to_string(),
                first_name: "Jane (Dummy)".to_string(),
                last_name: "Smith".to_string(),
                current_position: "Designer".to_string(),
                current_temp_position: None,
                pr_2021: None, pr_2022: None, pr_2023: None, pr_2024: Some(4.2),
                user_9box_2024: Some("2B".to_string()), user_9box_2025: None,
                notes: None, current_label: None, email: None, manager_id: None,
                department: None, location: None, hire_date: None, 
                skills: if all_skills.len() >= 3 {
                            vec![all_skills[2].clone()] // Example: assign Project Management
                        } else {
                            vec![]
                        },
            },
        ]
    }
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Box Planner")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        // Placeholder for message handling logic
        // This will be expanded in subsequent steps
        match message {
            Message::EmployeeSelected(id) => {
                println!("Employee selected: {}", id);
                self.selected_employee_id = Some(id);
            }
            Message::BoxClicked(box_id) => {
                if let Some(dragged_emp_id) = self.dragged_employee_id.take() {
                    // This is a drop from a card drag
                    for (_b_id, emp_ids_in_box) in self.grid_state.assignments.iter_mut() {
                        if let Some(pos) = emp_ids_in_box.iter().position(|id| *id == dragged_emp_id) {
                            emp_ids_in_box.remove(pos);
                        }
                    }
                    self.grid_state.assignments.retain(|_b_id, emp_ids_in_box| !emp_ids_in_box.is_empty());
                    self.grid_state.assignments
                        .entry(box_id.clone())
                        .or_default()
                        .push(dragged_emp_id.clone());
                    
                    println!("Dropped (via BoxClicked) employee {} to box {}", dragged_emp_id, box_id);
                    self.highlighted_box_id = Some(box_id.clone());
                    return Command::perform(
                        async { tokio::time::sleep(Duration::from_millis(500)).await },
                        move |_| Message::ClearBoxHighlight(box_id)
                    );

                } else if let Some(selected_emp_id) = self.selected_employee_id.clone() {
                    // Original BoxClicked logic: assign from main employee list
                    for (_b_id, emp_ids_in_box) in self.grid_state.assignments.iter_mut() {
                        if let Some(pos) = emp_ids_in_box.iter().position(|id| *id == selected_emp_id) {
                            emp_ids_in_box.remove(pos);
                        }
                    }
                    self.grid_state.assignments.retain(|_b_id, emp_ids_in_box| !emp_ids_in_box.is_empty());
                    self.grid_state.assignments
                        .entry(box_id.clone())
                        .or_default()
                        .push(selected_emp_id.clone());
                    println!("Assigned (via BoxClicked) employee {} to box {}", selected_emp_id, box_id);
                    self.selected_employee_id = None; // Clear selection from main list
                } else {
                    println!("Box {} clicked, but no employee selected for assignment and no drag operation in progress.", box_id);
                }
            }
            Message::ScaleChanged(new_scale) => {
                println!("Scale changed: {}", new_scale);
                self.view_scale = new_scale;
                self.app_settings.view_scale = Some(new_scale);
                
                let settings_path = Path::new(SETTINGS_FILE_PATH);
                match save_app_settings(settings_path, &self.app_settings) {
                    Ok(_) => println!("Successfully saved settings to {:?}", settings_path),
                    Err(e) => eprintln!("Failed to save settings to {:?}: {}", settings_path, e),
                }
            }
            Message::TabSelected(tab_id) => {
                self.active_tab = tab_id;
                println!("Tab selected: {:?}", tab_id);
            }
            Message::CardClicked(employee_id) => {
                if self.expanded_card_id.as_ref() == Some(&employee_id) {
                    self.expanded_card_id = None; // Toggle off if already expanded
                } else {
                    self.expanded_card_id = Some(employee_id);
                }
            }
            Message::NotesChanged(employee_id, new_notes) => {
                if let Some(employee) = self.employees.iter_mut().find(|e| e.user_id == employee_id) {
                    employee.notes = Some(new_notes);
                }
            }
            Message::RemoveSkillTag(employee_id, skill_id_to_remove) => {
                if let Some(employee) = self.employees.iter_mut().find(|e| e.user_id == employee_id) {
                    employee.skills.retain(|skill| skill.id != skill_id_to_remove);
                }
            }
            Message::CardDragStarted(employee_id) => {
                self.dragged_employee_id = Some(employee_id.clone());
                // self.selected_employee_id = None; // Optional: clear main list selection
                println!("Card drag started: {}", employee_id);
            }
            Message::CardDroppedOnBox(dragged_employee_id, target_box_id) => {
                println!("Card dropped: {} on box {}", dragged_employee_id, target_box_id);
                // Ensure an employee was actually being dragged and it's the same one
                if self.dragged_employee_id.as_ref() == Some(&dragged_employee_id) {
                    // 1. Remove employee from any previous box in grid_state.assignments
                    for (_b_id, emp_ids_in_box) in self.grid_state.assignments.iter_mut() {
                        if let Some(pos) = emp_ids_in_box.iter().position(|id| *id == dragged_employee_id) {
                            emp_ids_in_box.remove(pos);
                        }
                    }
                    // Remove boxes with empty assignments after removal
                    self.grid_state.assignments.retain(|_b_id, emp_ids_in_box| !emp_ids_in_box.is_empty());

                    // 2. Add employee to the new target_box_id
                    self.grid_state.assignments
                        .entry(target_box_id.clone())
                        .or_default()
                        .push(dragged_employee_id.clone());
                    
                    println!("Assigned employee {} to box {}", dragged_employee_id, target_box_id);
                    
                    // 3. Clear the dragged_employee_id state
                    self.dragged_employee_id = None; // Cleared after successful drop
                    
                    // Set highlight and schedule clearing
                    self.highlighted_box_id = Some(target_box_id.clone());
                    return Command::perform(
                        async { tokio::time::sleep(Duration::from_millis(500)).await },
                        move |_| Message::ClearBoxHighlight(target_box_id)
                    );
                } else {
                    eprintln!("CardDroppedOnBox called but no/mismatched employee was being dragged. Current dragged_employee_id: {:?}", self.dragged_employee_id);
                     // Important: If there was a mismatch or no drag, but CardDroppedOnBox was called,
                    // we should probably clear dragged_employee_id to prevent unintended drops later.
                    self.dragged_employee_id = None; // Ensure it's cleared on mismatch too
                }
                // If logic falls through (e.g., mismatch), return Command::none()
                return Command::none(); 
            }
            Message::SkillDragStarted(skill_id) => {
                self.dragged_skill_id = Some(skill_id.clone());
                // Potentially clear other drag states if necessary
                // self.dragged_employee_id = None; 
                println!("Skill drag started: {}", skill_id);
            }
            Message::SkillDroppedOnCard(skill_id, employee_id) => {
                if self.dragged_skill_id.as_ref() == Some(&skill_id) {
                    if let Some(employee) = self.employees.iter_mut().find(|e| e.user_id == employee_id) {
                        // Check if employee already has this skill (by id)
                        if !employee.skills.iter().any(|s| s.id == skill_id) {
                            // Find the skill from available_skills to get its full details (name, id)
                            if let Some(skill_to_add) = self.available_skills.iter().find(|s| s.id == skill_id).cloned() {
                                employee.skills.push(skill_to_add.clone()); // Push cloned skill
                                println!("Added skill {} to employee {}", skill_id, employee_id);
                                
                                // Set highlight and schedule clearing
                                self.highlighted_employee_id = Some(employee_id.clone());
                                self.dragged_skill_id = None; // Clear after successful processing
                                return Command::perform(
                                    async { tokio::time::sleep(Duration::from_millis(500)).await },
                                    move |_| Message::ClearCardHighlight(employee_id)
                                );
                            } else {
                                eprintln!("Skill {} not found in available_skills.", skill_id);
                            }
                        } else {
                            println!("Employee {} already has skill {}.", employee_id, skill_id);
                        }
                    } else {
                        eprintln!("Employee {} not found for skill drop.", employee_id);
                    }
                    self.dragged_skill_id = None; // Ensure dragged skill state is cleared
                } else {
                    eprintln!("SkillDroppedOnCard called but no/mismatched skill was being dragged.");
                    self.dragged_skill_id = None; // Also clear if there's a mismatch
                }
            }
            Message::ClearBoxHighlight(box_id_to_clear) => {
                if self.highlighted_box_id.as_ref() == Some(&box_id_to_clear) {
                    self.highlighted_box_id = None;
                }
            }
            Message::ClearCardHighlight(employee_id_to_clear) => {
                if self.highlighted_employee_id.as_ref() == Some(&employee_id_to_clear) {
                    self.highlighted_employee_id = None;
                }
            }
            Message::RefreshData => {
                println!("Refreshing data...");

                // Reload employees (similar to App::new)
                let employees_load_result = File::open(SAMPLE_EMPLOYEES_CSV_PATH)
                    .map_err(|e| format!("Failed to open CSV file '{}': {}", SAMPLE_EMPLOYEES_CSV_PATH, e))
                    .and_then(|file| {
                        let reader = BufReader::new(file);
                        import_employees_from_csv(reader)
                            .map_err(|e| format!("Failed to parse CSV from '{}': {}", SAMPLE_EMPLOYEES_CSV_PATH, e.to_string()))
                    });

                self.employees = match employees_load_result {
                    Ok(loaded_employees) => {
                        if loaded_employees.is_empty() {
                            println!("No employees loaded from CSV during refresh, using dummy data.");
                            Self::load_dummy_employees()
                        } else {
                            println!("Successfully reloaded {} employees from CSV.", loaded_employees.len());
                            loaded_employees
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reloading employees from CSV: {}. Using dummy data instead.", e);
                        Self::load_dummy_employees()
                    }
                };
                
                // Reset grid_state (similar to App::new, including any default assignments)
                let mut new_grid_state = GridState::default();
                if !self.employees.is_empty() { // Re-apply initial assignments based on newly loaded employees
                    new_grid_state.assignments.insert("1A".to_string(), vec![self.employees[0].user_id.clone()]);
                    if self.employees.len() > 1 {
                         new_grid_state.assignments.insert("2B".to_string(), vec![self.employees[1].user_id.clone()]);
                    }
                    if self.employees.len() > 2 {
                         // Assuming the same logic as App::new: add second employee to 1A if available
                         if let Some(existing_1a) = new_grid_state.assignments.get_mut("1A") {
                            existing_1a.push(self.employees[2].user_id.clone());
                         } else { // Should not happen if first employee was added
                            new_grid_state.assignments.insert("1A".to_string(), vec![self.employees[2].user_id.clone()]);
                         }
                    }
                    if self.employees.len() > 3 {
                        new_grid_state.assignments.insert("3C".to_string(), vec![self.employees[3].user_id.clone()]);
                    }
                }
                self.grid_state = new_grid_state;

                // Clear UI state variables
                self.selected_employee_id = None;
                self.expanded_card_id = None;
                self.dragged_employee_id = None;
                self.dragged_skill_id = None;
                self.highlighted_box_id = None;
                self.highlighted_employee_id = None;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Delegate to the view_app function in views.rs
        view_app(self)
    }

    // subscription method can be added later if needed for background tasks
    // fn subscription(&self) -> Subscription<Message> {
    //     Subscription::none()
    // }

    // theme method can be added if custom theming is desired
    // fn theme(&self) -> Self::Theme {
    //     Theme::default() // Or your custom theme
    // }

    fn overlay(&self) -> Option<Element<Message>> {
        if let Some(dragged_id) = &self.dragged_employee_id { // Use dragged_employee_id
            if let Some(employee_data) = self.employees.iter().find(|e| e.user_id == *dragged_id) {
                
                struct GhostStyle;
                impl iced::widget::container::StyleSheet for GhostStyle {
                    type Style = iced::Theme;
                    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
                        iced::widget::container::Appearance {
                            background: Some(iced::Background::Color(iced::Color::from_rgba(0.85, 0.85, 0.85, 0.75))), // Light gray, semi-transparent
                            border: iced::Border {
                                color: iced::Color::from_rgba(0.5, 0.5, 0.5, 0.75),
                                width: 1.0,
                                radius: iced::border::Radius::from(4.0),
                            },
                            shadow: Default::default(),
                            text_color: None,
                        }
                    }
                }

                let ghost_card_content = crate::widgets::employee_card(
                    employee_data, 
                    false, // is_expanded
                    false, // is_highlighted_for_skill_drop (not a drop target itself)
                    false, // is_drag_source (the original card is the source)
                    // No is_ghost parameter needed here as styling is done by the wrapper
                );

                let ghost_element = iced::widget::Container::new(ghost_card_content)
                    .width(iced::Length::Fixed(150.0 * self.view_scale)) 
                    .height(iced::Length::Fixed(100.0 * self.view_scale)) 
                    .style(iced::theme::Container::Custom(Box::new(GhostStyle)));
                
                // Basic positioning: This will likely appear at the top-left of the overlay.
                // To position at cursor, a different approach is needed, possibly involving
                // a full-screen transparent container with alignment, or Canvas.
                // For now, just returning the styled ghost card.
                return Some(ghost_element.into());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    // AppSettings is already imported via super::* if App itself is, 
    // but being explicit for models can be clearer.
    // Employee and GridState are part of App struct, so super::* covers them.
    // use box_planner_core::models::{Employee, GridState, AppSettings}; // Not strictly needed if super::* is used well.

    // Helper to create a basic App for testing.
    // It relies on App::new()'s existing behavior for handling missing files
    // (falling back to dummy employees and default settings).
    fn setup_app() -> App {
        // To prevent tests from creating/modifying actual files in the project directory,
        // ideally, paths should be configurable or mocked.
        // For now, we accept that App::new() might try to read/write
        // "box_planner_ui/app_settings.json" and read "box_planner_ui/sample_employees.csv".
        // The current App::new() logic falls back to defaults/dummies, which is testable.
        App::new()
    }

    // Note: The test for `dragged_employee_id` was for the old implementation.
    // The field is now `drag_source_card_id`.
    // The test `test_card_drag_and_drop_logic` should be updated or replaced
    // once the full drag-and-drop logic with overlay and mouse release is in place.
    // For now, I will keep the existing test as it tests the press-and-click-box logic.
    // A new test for the overlay itself is not straightforward as it's a visual element.

    #[test]
    fn test_initial_state() {
        let app = setup_app();
        assert_eq!(app.selected_employee_id, None, "Selected employee ID should be None initially.");
        assert_eq!(app.active_tab, TabId::Employees, "Default active tab should be Employees.");
        
        // Check view_scale based on AppSettings default or loaded settings.
        // AppSettings::default() sets view_scale to Some(1.0).
        // App::new() uses this or a value from the file.
        assert_eq!(app.view_scale, app.app_settings.view_scale.unwrap_or(1.0), "View scale should match settings or default.");
        assert!(app.view_scale > 0.0, "View scale must be positive.");

        assert!(!app.employees.is_empty(), "Employees list should not be empty (dummy data should load).");
        
        // Check default app_settings state (theme_preference is a good indicator)
        // This implicitly tests that AppSettings::default() was called if file was missing.
        let default_settings = AppSettings::default();
        if !Path::new(SETTINGS_FILE_PATH).exists() {
            assert_eq!(app.app_settings.theme_preference, default_settings.theme_preference, "Theme preference should be default if no settings file.");
            assert_eq!(app.app_settings.view_scale, default_settings.view_scale, "View scale in settings should be default if no settings file.");
        } else {
            // If file exists, we can't easily know its content here without reading it again,
            // but we know app.app_settings was populated.
            println!("Note: test_initial_state assumes if settings file exists, it's valid or App::new handled errors.");
        }
    }

    #[test]
    fn test_employee_selected() {
        let mut app = setup_app();
        let test_emp_id = "emp_test_id_selected".to_string();
        app.update(Message::EmployeeSelected(test_emp_id.clone()));
        assert_eq!(app.selected_employee_id, Some(test_emp_id), "Selected employee ID was not set correctly.");
    }

    #[test]
    fn test_scale_changed() {
        let mut app = setup_app();
        let new_scale = 1.75;
        app.update(Message::ScaleChanged(new_scale));
        assert_eq!(app.view_scale, new_scale, "View scale in App struct was not updated.");
        assert_eq!(app.app_settings.view_scale, Some(new_scale), "View scale in AppSettings was not updated.");
        // We expect App::new to create a default settings file if it doesn't exist,
        // or load existing. Message::ScaleChanged should then save it.
        // We can check if the file reflects this after the test, but that's more of an integration test.
        // For unit test, checking the in-memory state is key.
    }

    #[test]
    fn test_assign_employee_to_box() {
        let mut app = setup_app();
        // App::new() loads dummy employees if CSV fails or is empty.
        // This test relies on at least one employee being available.
        assert!(!app.employees.is_empty(), "Prerequisite: Employee list is empty, cannot run test.");
        let test_emp_id = app.employees[0].user_id.clone();
        let target_box_id = "TestBox_Assign".to_string();

        app.update(Message::EmployeeSelected(test_emp_id.clone()));
        app.update(Message::BoxClicked(target_box_id.clone()));

        assert!(app.grid_state.assignments.get(&target_box_id).is_some(), "Target box should exist in assignments.");
        assert!(app.grid_state.assignments.get(&target_box_id).unwrap().contains(&test_emp_id), "Employee was not assigned to the target box.");
        assert_eq!(app.selected_employee_id, None, "Selected employee ID should be cleared after assignment.");
    }
    
    #[test]
    fn test_move_employee_between_boxes() {
        let mut app = setup_app();
        assert!(!app.employees.is_empty(), "Prerequisite: Employee list is empty, cannot run test.");
        let test_emp_id = app.employees[0].user_id.clone();
        let initial_box_id = "BoxAlpha_Move".to_string();
        let target_box_id = "BoxBeta_Move".to_string();

        // First assignment to initial_box_id
        app.update(Message::EmployeeSelected(test_emp_id.clone()));
        app.update(Message::BoxClicked(initial_box_id.clone()));
        
        // Now select again and move to target_box_id
        app.update(Message::EmployeeSelected(test_emp_id.clone()));
        app.update(Message::BoxClicked(target_box_id.clone()));

        assert!(app.grid_state.assignments.get(&target_box_id).is_some(), "Target box for move should exist.");
        assert!(app.grid_state.assignments.get(&target_box_id).unwrap().contains(&test_emp_id), "Employee was not moved to the target box.");
        
        // Check if employee is removed from the old box or if the old box is removed if empty
        let initial_box_assignments = app.grid_state.assignments.get(&initial_box_id);
        assert!(initial_box_assignments.map_or(true, |ids| !ids.contains(&test_emp_id)), "Employee was not removed from the initial box.");
        
        assert_eq!(app.selected_employee_id, None, "Selected employee ID should be cleared after moving.");
    }
    
    #[test]
    fn test_click_box_no_employee_selected() {
        let mut app = setup_app();
        // Clone initial state of assignments. 
        // App::new() might populate assignments with dummy data, so we capture that.
        let initial_assignments = app.grid_state.assignments.clone();
        let target_box_id = "AnyBox_NoSelect".to_string();
        
        app.update(Message::BoxClicked(target_box_id.clone()));
        
        // Assert that assignments are unchanged.
        // If the target_box_id was part of initial_assignments, its content should be the same.
        // If it wasn't, it should still not be there.
        assert_eq!(app.grid_state.assignments, initial_assignments, "Assignments should not change if no employee is selected.");
    }

    #[test]
    fn test_tab_selection() {
        let mut app = setup_app();

        // Assert initial state (already covered by test_initial_state but good for clarity)
        assert_eq!(app.active_tab, TabId::Employees, "Initial active tab should be Employees.");

        // Select Skills tab
        app.update(Message::TabSelected(TabId::Skills));
        assert_eq!(app.active_tab, TabId::Skills, "Active tab should be Skills after selection.");

        // Select Employees tab again
        app.update(Message::TabSelected(TabId::Employees));
        assert_eq!(app.active_tab, TabId::Employees, "Active tab should be back to Employees after selection.");
    }

    #[test]
    fn test_add_and_prevent_duplicate_skill() {
        let mut app = setup_app(); 
        assert!(!app.employees.is_empty(), "Prerequisite: Employee list is empty.");
        assert!(!app.available_skills.is_empty(), "Prerequisite: Available skills list is empty.");

        let employee_id = app.employees[0].user_id.clone();
        let skill_to_add = app.available_skills[0].clone();

        // Simulate starting a skill drag
        app.update(Message::SkillDragStarted(skill_to_add.id.clone()));
        assert_eq!(app.dragged_skill_id, Some(skill_to_add.id.clone()), "dragged_skill_id should be set after SkillDragStarted.");

        // Drop the skill on the card
        app.update(Message::SkillDroppedOnCard(skill_to_add.id.clone(), employee_id.clone()));
        
        let employee = app.employees.iter().find(|e| e.user_id == employee_id).unwrap();
        assert!(employee.skills.iter().any(|s| s.id == skill_to_add.id), "Employee should have the skill after first drop.");
        assert_eq!(app.dragged_skill_id, None, "dragged_skill_id should be cleared after drop.");

        let skill_count_before_duplicate_add = employee.skills.len();

        // Attempt to add the same skill again
        app.update(Message::SkillDragStarted(skill_to_add.id.clone())); // Start drag again
        app.update(Message::SkillDroppedOnCard(skill_to_add.id.clone(), employee_id.clone()));
        
        let employee_after_duplicate_add = app.employees.iter().find(|e| e.user_id == employee_id).unwrap();
        assert_eq!(employee_after_duplicate_add.skills.len(), skill_count_before_duplicate_add, "Skill count should not increase on duplicate add.");
        assert_eq!(app.dragged_skill_id, None, "dragged_skill_id should be cleared even if skill was not added due to duplication.");
    }

    #[test]
    fn test_skill_removal() {
        let mut app = setup_app();
        assert!(!app.employees.is_empty(), "Prerequisite: Employee list is empty.");
        assert!(!app.available_skills.is_empty(), "Prerequisite: Available skills list is empty.");

        let employee_id = app.employees[0].user_id.clone();
        let skill_to_manage = app.available_skills[0].clone();

        // First, add the skill to the employee
        app.update(Message::SkillDragStarted(skill_to_manage.id.clone()));
        app.update(Message::SkillDroppedOnCard(skill_to_manage.id.clone(), employee_id.clone()));
        
        let employee_with_skill = app.employees.iter().find(|e| e.user_id == employee_id).unwrap();
        assert!(employee_with_skill.skills.iter().any(|s| s.id == skill_to_manage.id), "Skill should be present before removal.");

        // Now, remove the skill
        app.update(Message::RemoveSkillTag(employee_id.clone(), skill_to_manage.id.clone()));
        
        let employee_after_removal = app.employees.iter().find(|e| e.user_id == employee_id).unwrap();
        assert!(!employee_after_removal.skills.iter().any(|s| s.id == skill_to_manage.id), "Skill should be removed.");
    }

    #[test]
    fn test_card_drag_and_drop_logic() {
        let mut app = setup_app();
        assert!(!app.employees.is_empty(), "Prerequisite: Employee list is empty.");
        
        let employee_to_drag_id = app.employees[0].user_id.clone();
        let initial_box_id = "TestBoxInit".to_string();
        let target_box_id = "TestBoxTarget".to_string();

        // Manually assign employee to an initial box for testing this specific logic
        app.grid_state.assignments.entry(initial_box_id.clone()).or_default().push(employee_to_drag_id.clone());
        
        // Simulate starting card drag
        app.update(Message::CardDragStarted(employee_to_drag_id.clone()));
        assert_eq!(app.dragged_employee_id, Some(employee_to_drag_id.clone()), "dragged_employee_id should be set after CardDragStarted.");

        // Simulate dropping on a new box (using BoxClicked, which should forward if drag is active)
        app.update(Message::BoxClicked(target_box_id.clone()));
        
        // Note: BoxClicked forwards to CardDroppedOnBox which handles the logic.
        // Need to check results after that internal forwarding.
        // The direct CardDroppedOnBox handler should have done the work.

        assert!(app.grid_state.assignments.get(&initial_box_id).map_or(true, |ids| !ids.contains(&employee_to_drag_id)), "Employee should be removed from the initial box.");
        assert!(app.grid_state.assignments.get(&target_box_id).map_or(false, |ids| ids.contains(&employee_to_drag_id)), "Employee should be added to the target box.");
        assert_eq!(app.dragged_employee_id, None, "dragged_employee_id should be cleared after drop.");
    }
    
    #[test]
    fn test_refresh_data_core_state_reset() {
        let mut app = setup_app();
        assert!(app.employees.len() >= 1, "Need at least one employee for this test");

        // 1. Modify some state
        let original_view_scale = app.view_scale; // Store for later comparison
        let employee_id_to_select = app.employees[0].user_id.clone();
        
        // Assign an employee to a non-default box
        let test_box_id = "TestBoxForRefresh".to_string();
        app.update(Message::EmployeeSelected(employee_id_to_select.clone()));
        app.update(Message::BoxClicked(test_box_id.clone()));
        assert!(app.grid_state.assignments.get(&test_box_id).is_some(), "Employee should be in test box before refresh.");

        // Set other UI states
        app.selected_employee_id = Some("some_selected_id".to_string());
        app.expanded_card_id = Some("some_expanded_id".to_string());
        app.dragged_employee_id = Some("some_dragged_emp_id".to_string());
        app.dragged_skill_id = Some("some_dragged_skill_id".to_string());
        app.highlighted_box_id = Some("some_highlighted_box".to_string());
        app.highlighted_employee_id = Some("some_highlighted_emp".to_string());

        // 2. Action: Send Message::RefreshData
        app.update(Message::RefreshData);

        // 3. Assertions
        // Check if grid_state.assignments are reset to initial state.
        // This depends on the default assignments in App::new() for the dummy data.
        // For example, dummy employee "1" is usually assigned to "1A".
        let first_dummy_employee_id = "1".to_string(); // Assuming this is a known ID from load_dummy_employees
        let is_first_dummy_in_1a_after_refresh = app.grid_state.assignments
            .get("1A")
            .map_or(false, |ids| ids.contains(&first_dummy_employee_id));
        
        // If your dummy data load doesn't always put employee "1" in "1A", adjust this assertion.
        // Or, check that the test_box_id assignment is gone.
        assert!(!app.grid_state.assignments.get(&test_box_id).map_or(false, |ids| ids.contains(&employee_id_to_select)), "Custom assignment should be cleared after refresh.");
        if app.employees.iter().any(|e| e.user_id == first_dummy_employee_id) { // Only assert if dummy employee "1" exists
             assert!(is_first_dummy_in_1a_after_refresh, "Employee '1' should be in box '1A' after refresh if dummy data is loaded.");
        }


        // Assert UI state variables are cleared
        assert_eq!(app.selected_employee_id, None, "selected_employee_id should be None.");
        assert_eq!(app.expanded_card_id, None, "expanded_card_id should be None.");
        assert_eq!(app.dragged_employee_id, None, "dragged_employee_id should be None.");
        assert_eq!(app.dragged_skill_id, None, "dragged_skill_id should be None.");
        assert_eq!(app.highlighted_box_id, None, "highlighted_box_id should be None.");
        assert_eq!(app.highlighted_employee_id, None, "highlighted_employee_id should be None.");

        // Assert that view_scale (part of AppSettings) is NOT reset
        assert_eq!(app.view_scale, original_view_scale, "View scale should not be reset by RefreshData.");
        assert_eq!(app.app_settings.view_scale, Some(original_view_scale), "View scale in AppSettings should not be reset.");

        // Assert that available_skills is NOT reset (it's loaded once in new())
        assert!(!app.available_skills.is_empty(), "Available skills should still be present.");
    }
}
