use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    #[serde(rename = "User ID")]
    pub user_id: String,
    #[serde(rename = "PR Group 2025")]
    pub pr_group_2025: String,
    #[serde(rename = "First Name")]
    pub first_name: String,
    #[serde(rename = "Last Name")]
    pub last_name: String,
    #[serde(rename = "Current Position")]
    pub current_position: String,
    #[serde(rename = "Current Temp Position")]
    pub current_temp_position: Option<String>,
    #[serde(rename = "PR2021")]
    pub pr_2021: Option<f64>,
    #[serde(rename = "PR2022")]
    pub pr_2022: Option<f64>,
    #[serde(rename = "PR2023")]
    pub pr_2023: Option<f64>,
    #[serde(rename = "PR2024")]
    pub pr_2024: Option<f64>,
    #[serde(rename = "User 9Box 2024")]
    pub user_9box_2024: Option<String>,
    #[serde(rename = "User 9Box 2025")]
    pub user_9box_2025: Option<String>,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,
    #[serde(rename = "Current Label")]
    pub current_label: Option<String>,
    // Additional fields based on typical employee data, adjust as needed
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "Manager ID")]
    pub manager_id: Option<String>,
    #[serde(rename = "Department")]
    pub department: Option<String>,
    #[serde(rename = "Location")]
    pub location: Option<String>,
    #[serde(rename = "Hire Date")]
    pub hire_date: Option<String>, // Consider using a date/time type if appropriate
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub id: String,
    pub name: String,
}

// Grid-Related Structures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridBox {
    pub id: String,
    pub label: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GridState {{
    pub assignments: std::collections::HashMap<String, Vec<String>>,
}}

// Application Settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSettings {
    pub theme_preference: String,
    pub department_colors: std::collections::HashMap<String, String>,
    pub auto_save_enabled: bool,
    pub view_scale: Option<f32>,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            theme_preference: "system".to_string(),
            department_colors: std::collections::HashMap::new(),
            auto_save_enabled: false,
            view_scale: Some(1.0),
        }
    }
}
