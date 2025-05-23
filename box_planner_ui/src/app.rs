use crate::messages::Message;
use crate::views::view_app;
// Corrected to import the function that expects a Reader
use box_planner_core::csv_processing::import_employees_from_csv; 
use box_planner_core::models::{AppSettings, Employee, GridState};
use box_planner_core::persistence::{load_app_settings, save_app_settings};
use iced::{Command, Element, Theme}; // Removed Executor and Subscription
use std::fs::File; // Added File
use std::io::BufReader; // Added BufReader
use std::path::Path;

const SETTINGS_FILE_PATH: &str = "box_planner_ui/app_settings.json";
const SAMPLE_EMPLOYEES_CSV_PATH: &str = "box_planner_ui/sample_employees.csv";

pub struct App {
    pub employees: Vec<Employee>,
    pub grid_state: GridState,
    pub selected_employee_id: Option<String>,
    pub view_scale: f32,
    pub app_settings: AppSettings, // Added app_settings field
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
        }
    }

    // Fallback function to load dummy employees
    fn load_dummy_employees() -> Vec<Employee> {
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
                if let Some(employee_id) = self.selected_employee_id.clone() {
                    // Remove employee from any previous box
                    for (_b_id, emp_ids) in self.grid_state.assignments.iter_mut() {
                        if let Some(pos) = emp_ids.iter().position(|id| *id == employee_id) {
                            emp_ids.remove(pos);
                        }
                    }
                    // Remove boxes with empty assignments
                    self.grid_state.assignments.retain(|_, emp_ids| !emp_ids.is_empty());

                    // Add employee to the new box
                    self.grid_state.assignments
                        .entry(box_id.clone())
                        .or_default()
                        .push(employee_id.clone());
                    
                    println!("Assigned employee {} to box {}", employee_id, box_id);

                    // Attempt to persist grid_state
                    // let persistence_path = "box_planner_ui/grid_data.json"; // Example path
                    // match box_planner_core::persistence::save_grid_state(&self.grid_state, persistence_path) {
                    //     Ok(_) => println!("Grid state saved to {}", persistence_path),
                    //     Err(e) => eprintln!("Failed to save grid state: {}. Proceeding with in-memory state.", e),
                    // }
                    println!("Persistence: 'save_grid_state' function not found in core/src/persistence.rs. Skipping file save. State is in-memory only.");


                    self.selected_employee_id = None; // Clear selection
                } else {
                    println!("Box {} clicked, but no employee selected.", box_id);
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
}