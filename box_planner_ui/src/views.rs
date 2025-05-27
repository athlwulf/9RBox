use crate::app::App;
use crate::messages::{Message, TabId};
use crate::widgets::employee_card; // New import
// Added Rule to the import list, removed redundant Button, Column, Container, Row, Text which are covered by explicit imports later or not used.
// The explicit individual imports like `Button, Column, Container, Row, Text` are fine,
// but `rule` (the module) was being imported, not `Rule` (the struct).
// Removed unused lowercase 'button' and 'text' module aliases.
use iced::widget::{column, container, row, scrollable, Button, Column, Container, Row, Rule, Text}; 
use iced::{Element, Length };
use crate::styles::{BoxHighlightStyle, DefaultBoxStyle, AppBackground, PrimaryButton, CardStyle};

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
            .width(Length::Fill)
            .style(iced::theme::Button::Custom(Box::new(PrimaryButton)));

        employee_list_content = employee_list_content.push(button);
    }

    // Tab buttons
    let employees_tab_button = Button::new(Text::new("Employees"))
        .on_press(Message::TabSelected(TabId::Employees))
        .width(Length::Fill)
        .style(iced::theme::Button::Custom(Box::new(PrimaryButton)));
    let skills_tab_button = Button::new(Text::new("Skills"))
        .on_press(Message::TabSelected(TabId::Skills))
        .width(Length::Fill)
        .style(iced::theme::Button::Custom(Box::new(PrimaryButton)));

    let tab_buttons = row![
        employees_tab_button,
        skills_tab_button
    ]
    .spacing(5);

    // Conditional content based on active tab
    let left_panel_content = match app.active_tab {
        TabId::Employees => {
            column![
                Text::new("Employee List").size(20),
                scrollable(employee_list_content) // Display the list of buttons
            ]
            .spacing(10)
        }
        TabId::Skills => {
            // NEW: Skills Palette View
            let mut skills_list_content = Column::new().spacing(5).padding(5);
            for skill in &app.available_skills {
                let skill_button = Button::new(Text::new(skill.name.clone()))
                    .on_press(Message::SkillDragStarted(skill.id.clone()))
                    .width(Length::Fill)
                    .style(iced::theme::Button::Custom(Box::new(PrimaryButton)));
                skills_list_content = skills_list_content.push(skill_button);
            }

            column![
                Text::new("Available Skills").size(20),
                scrollable(skills_list_content) // Make it scrollable if many skills
            ]
            .spacing(10)
        }
    };

    // Main content: A row with three columns
    let content = row![
        // Left Panel Section (with tabs)
        Container::new(
            column![
                tab_buttons,
                left_panel_content
            ]
            .spacing(10)
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

    let refresh_button = Button::new(Text::new("Refresh Data"))
        .on_press(Message::RefreshData)
        .padding(5)
        .style(iced::theme::Button::Custom(Box::new(PrimaryButton)));

    // New main column including the refresh button and the existing main_layout_row (renamed to 'content_row')
    let content_with_refresh = column![
        refresh_button,
        content // 'content' is the existing main_layout_row
    ]
    .spacing(10)
    .padding(10); // Add some padding around the whole content

    let mut layered_content = content_with_refresh;

    if let (Some(dragged_id), Some((x, y))) = (&app.dragged_employee_id, app.dragged_item_current_pos) {
        if let Some(employee_data) = app.employees.iter().find(|e| e.user_id == *dragged_id) {
            let ghost_card = Container::new(employee_card(
                employee_data,
                false,
                app.dragged_skill_id.as_ref(),
                false,
            ))
            .width(Length::Fixed(150.0 * app.view_scale))
            .padding(5)
            .style(iced::theme::Container::Custom(Box::new(CardStyle)));

            // Wrap ghost_card in a container; can't position with .translate, so let it float at the bottom
            let ghost_overlay = Container::new(ghost_card)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .padding(0)
                .style(iced::theme::Container::Transparent); // No background

            layered_content = layered_content.push(ghost_overlay);
        }
    }

    // Wrap content in a container for the main window
    container(layered_content)
        .style(iced::theme::Container::Custom(Box::new(AppBackground)))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        //.center_y() // Centering Y might not be desired if content is scrollable or long
        .into()
}

// New function to render the 9-Box Grid
fn view_9box_grid(app: &App) -> Element<Message> {
    let box_ids_labels = [
        [("1A", "Solid Professional"), ("1B", "Top Talent"), ("1C", "Growth Leader")],
        [("2A", "Core Player"),        ("2B", "High Potential"), ("2C", "Rising Star")],
        [("3A", "Under Performer"),    ("3B", "Developing"),     ("3C", "Emerging Contributor")],
    ];

    let mut grid_column = Column::new().spacing(5).align_items(iced::Alignment::Center);

    for row_of_boxes in box_ids_labels.iter() {
        let mut grid_row_element = Row::new().spacing(5).align_items(iced::Alignment::Start);
        for (box_id, box_label) in row_of_boxes.iter() {
            let mut box_content_column = Column::new()
                .push(Text::new(*box_label).size(14))
                .spacing(3)
                .align_items(iced::Alignment::Start);

            let mut employee_found_in_box = false;
            if let Some(employee_ids) = app.grid_state.assignments.get(*box_id) {
                for emp_id in employee_ids {
                    if let Some(employee) = app.employees.iter().find(|e| e.user_id == *emp_id) {
                        let is_expanded = app.expanded_card_id.as_ref() == Some(&employee.user_id);
                        let is_card_highlighted = app.highlighted_employee_id.as_ref() == Some(&employee.user_id);
                        // Wrap the entire card (including employee_card and indicators) in a Container with CardStyle
                        let mut card_column = Column::new()
                            .push(employee_card(
                                employee,
                                is_expanded,
                                app.dragged_skill_id.as_ref(),
                                is_card_highlighted,
                            ));

                        let mut indicators_row = Row::new().spacing(5);
                        // Icons for risk and impact could go here
                        if let Some(notes) = &employee.notes {
                            if !notes.trim().is_empty() {
                                indicators_row = indicators_row.push(Text::new("📝").size(16));
                            }
                        }
                        card_column = card_column.push(indicators_row);

                        let styled_card = Container::new(card_column)
                            .padding(5)
                            .style(iced::theme::Container::Custom(Box::new(CardStyle)));

                        box_content_column = box_content_column.push(styled_card);

                        employee_found_in_box = true;
                    } else {
                        // Fallback for missing employee data
                        box_content_column = box_content_column.push(Text::new(format!("Employee ID: {} (Not Found)", emp_id)).size(11));
                        employee_found_in_box = true; // Still counts as content for the box
                    }
                }
            }
            
            // Ensure there's always some content for consistent height if no employees and no error messages
            if !employee_found_in_box {
                 box_content_column = box_content_column.push(Text::new(" ").size(11)); // Add a space to ensure height
            }

            
            let is_box_highlighted = app.highlighted_box_id.as_ref() == Some(&box_id.to_string());
            let box_container_style = if is_box_highlighted {
                iced::theme::Container::Custom(Box::new(BoxHighlightStyle))
            } else {
                iced::theme::Container::Custom(Box::new(DefaultBoxStyle))
            };

            let grid_box_button = Button::new(
                Container::new(scrollable(box_content_column)) // Make content scrollable if it overflows
                    .width(Length::Fixed(150.0 * app.view_scale)) // Fixed size for boxes, scaled
                    .height(Length::Fixed(100.0 * app.view_scale)) // Ensure this height allows for some content before scrolling
                    .padding(5)
                    .center_x() // Center content horizontally
                    .style(box_container_style) // Apply conditional style to the container inside the button
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
