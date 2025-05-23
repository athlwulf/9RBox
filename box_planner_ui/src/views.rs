use crate::app::App;
use crate::messages::Message;
// Added Rule to the import list, removed redundant Button, Column, Container, Row, Text which are covered by explicit imports later or not used.
// The explicit individual imports like `Button, Column, Container, Row, Text` are fine,
// but `rule` (the module) was being imported, not `Rule` (the struct).
// Removed unused lowercase 'button' and 'text' module aliases.
use iced::widget::{column, container, row, scrollable, Button, Column, Container, Row, Rule, Text}; 
use iced::{Element, Length};

pub fn view_app(app: &App) -> Element<Message> {
    // Employee List display
    let mut employee_list_content = Column::new().spacing(5);
    for employee in &app.employees {
        let mut full_name = format!("{} {}", employee.first_name, employee.last_name);
        if Some(employee.user_id.clone()) == app.selected_employee_id {
            full_name = format!("> {}", full_name); // Add prefix for selected employee
        }
        let button = Button::new(Text::new(full_name.clone()))
            .on_press(Message::EmployeeSelected(employee.user_id.clone()))
            .width(Length::Fill);

        employee_list_content = employee_list_content.push(button);
    }

    // Main content: A row with three columns
    let content = row![
        // Employee List Section
        Container::new(
            scrollable(
                column![
                    Text::new("Employee List").size(20),
                    employee_list_content // Display the list of buttons
                ]
                .spacing(10)
            )
        )
        .width(Length::FillPortion(1)) // Takes 1/4 of the space
        .height(Length::Fill)
        .padding(10),

        // Vertical Separator
        Rule::vertical(10), // Corrected call

        // 9-Box Grid Section
        Container::new(
            view_9box_grid(app) // Call the new function to render the grid
        )
        .width(Length::FillPortion(2)) // Takes 2/4 of the space
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(10),

        // Vertical Separator
        Rule::vertical(10), // Corrected call

        // Details Panel Section
        Container::new({
            let details_content = if let Some(selected_id) = &app.selected_employee_id {
                if let Some(employee) = app.employees.iter().find(|e| e.user_id == *selected_id) {
                    let name = format!("{} {}", employee.first_name, employee.last_name);
                    let pr_2024_str = employee.pr_2024.map_or("N/A".to_string(), |p| p.to_string());
                    let box_2024_str = employee.user_9box_2024.clone().unwrap_or_else(|| "N/A".to_string());
                    let temp_pos_str = employee.current_temp_position.clone().unwrap_or_else(|| "N/A".to_string());
                    let notes_str = employee.notes.clone().unwrap_or_else(|| "".to_string());


                    column![
                        Text::new("Employee Details").size(20),
                        Rule::horizontal(5), // Corrected call
                        Text::new(name).size(18),
                        Text::new(format!("ID: {}", employee.user_id)),
                        Text::new(format!("Position: {}", employee.current_position)),
                        Text::new(format!("Temporary Position: {}", temp_pos_str)),
                        Text::new(format!("PR Group 2025: {}", employee.pr_group_2025)),
                        Text::new(format!("PR 2024: {}", pr_2024_str)),
                        Text::new(format!("9-Box 2024: {}", box_2024_str)),
                        Text::new("Notes:"),
                        scrollable(Text::new(notes_str).width(Length::Fill)) // Make notes scrollable if long
                    ]
                    .spacing(5)
                    .padding(5)
                    .width(Length::Fill)
                    .align_items(iced::Alignment::Start)
                } else {
                    column![Text::new("Employee not found.")]
                        .width(Length::Fill)
                        .height(Length::Fill) // Ensure it fills space to center vertically
                        .align_items(iced::Alignment::Center)
                        // .justify_content(iced::alignment::Vertical::Center) // Removed
                }
            } else {
                column![Text::new("Select an employee to view details.")]
                    .width(Length::Fill)
                    .height(Length::Fill) // Ensure it fills space to center vertically
                    .align_items(iced::Alignment::Center)
                    // .justify_content(iced::alignment::Vertical::Center) // Removed
            };
            scrollable(details_content) // Wrap the entire details_content in a scrollable
        })
        .width(Length::FillPortion(1)) // Takes 1/4 of the space
        .height(Length::Fill)
        .padding(10)
    ]
    .spacing(10) // Spacing between the main sections and rules
    .align_items(iced::Alignment::Start); // Align items to the top

    // Wrap content in a container for the main window
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

// New function to render the 9-Box Grid
fn view_9box_grid(app: &App) -> Element<Message> {
    let box_ids_labels = [
        [("1A", "High Perf / High Pot"), ("1B", "High Perf / Med Pot"), ("1C", "High Perf / Low Pot")],
        [("2A", "Med Perf / High Pot"),  ("2B", "Med Perf / Med Pot"),  ("2C", "Med Perf / Low Pot")],
        [("3A", "Low Perf / High Pot"),  ("3B", "Low Perf / Med Pot"),  ("3C", "Low Perf / Low Pot")],
    ];

    let mut grid_column = Column::new().spacing(5).align_items(iced::Alignment::Center);

    for row_of_boxes in box_ids_labels.iter() {
        let mut grid_row_element = Row::new().spacing(5).align_items(iced::Alignment::Start);
        for (box_id, box_label) in row_of_boxes.iter() {
            let mut employee_names_in_box = Vec::new();
            if let Some(employee_ids) = app.grid_state.assignments.get(*box_id) {
                for emp_id in employee_ids {
                    if let Some(employee) = app.employees.iter().find(|e| e.user_id == *emp_id) {
                        employee_names_in_box.push(format!("- {} {}", employee.first_name, employee.last_name));
                    } else {
                        employee_names_in_box.push(format!("- (ID: {})", emp_id)); // Fallback
                    }
                }
            }

            let mut box_content_column = Column::new()
                .push(Text::new(*box_label).size(14)) // Use descriptive label
                .spacing(3)
                .align_items(iced::Alignment::Start); // Align text to the start

            for name in &employee_names_in_box { // Changed to iterate by reference
                box_content_column = box_content_column.push(Text::new(name.clone()).size(11)); // Added clone() as name is now &String
            }
            
            // Ensure there's always some content for consistent height if no employees
            if employee_names_in_box.is_empty() { // This check is now valid
                 box_content_column = box_content_column.push(Text::new(" ").size(11)); // Add a space to ensure height
            }

            let grid_box_button = Button::new(
                Container::new(scrollable(box_content_column)) // Make content scrollable if it overflows
                    .width(Length::Fixed(150.0 * app.view_scale)) // Fixed size for boxes, scaled
                    .height(Length::Fixed(100.0 * app.view_scale))
                    .padding(5)
                    .center_x() // Center content horizontally
                    // .style(theme::Container::Bordered) // Example for styling, needs theme setup
            )
            .on_press(Message::BoxClicked(box_id.to_string()))
            .width(Length::Fixed(150.0 * app.view_scale)) // Scaled button width
            .height(Length::Fixed(100.0 * app.view_scale)); // Scaled button height
            
            grid_row_element = grid_row_element.push(grid_box_button);
        }
        grid_column = grid_column.push(grid_row_element);
    }
    
    // Add a slider for scaling the view
    let scale_slider = iced::widget::slider(0.5..=2.0, app.view_scale, Message::ScaleChanged)
        .step(0.1);

    column![
        Text::new("9-Box Grid").size(24),
        scale_slider, // Add slider to control scale
        Text::new(format!("Zoom: {:.1}x", app.view_scale)).size(12),
        grid_column,
        Text::new(format!("Selected Employee: {:?}", app.selected_employee_id)).size(12),
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .into()
}
