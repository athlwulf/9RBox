// in box_planner_ui/src/widgets.rs
use iced::{Element, Length, Alignment}; // Alignment might be needed
use iced::widget::{Column, Text, Container, Button, TextInput, Row}; // Row added
use box_planner_core::models::Employee; // Skill import removed
use crate::messages::Message;

// New signature: note dragged_skill_id and is_highlighted parameters
pub fn employee_card<'a>(
    employee: &'a Employee, 
    is_expanded: bool, 
    dragged_skill_id: Option<&'a String>, // Pass current dragged skill ID
    is_highlighted: bool // New parameter for highlighting
) -> Element<'a, Message> {
    let full_name = format!("{} {}", employee.first_name, employee.last_name);
    let position = employee.current_position.clone();
    let pr_score = employee.pr_2024.map_or("N/A".to_string(), |score| score.to_string());

    let mut card_details = Column::new()
        .spacing(5)
        .push(Text::new(full_name).size(18))
        .push(Text::new(format!("Position: {}", position)))
        .push(Text::new(format!("PR 2024: {}", pr_score)));

    // Add skill tags display
    if !employee.skills.is_empty() {
        let mut skills_section = Column::new().spacing(2).align_items(Alignment::Start);
        // skills_section = skills_section.push(Text::new("Skills:").size(14)); // Optional title for skills
        for skill in &employee.skills {
            let skill_tag_row = Row::new()
                .spacing(4)
                .align_items(Alignment::Center)
                .push(Text::new(skill.name.clone()).size(12)) // Display skill name
                .push(
                    Button::new(Text::new("×").size(12)) // Small "x" button
                        .on_press(Message::RemoveSkillTag(employee.user_id.clone(), skill.id.clone()))
                        .padding(2)
                        // .style(theme::Button::Destructive) // Optional: style for remove button
                );
            skills_section = skills_section.push(skill_tag_row);
        }
        card_details = card_details.push(skills_section);
    }

    // Notes Toggle Button and TextInput section
    let notes_button = Button::new(Text::new(if is_expanded { "[Collapse Notes]" } else { "[Expand Notes]" }))
        .on_press(Message::CardClicked(employee.user_id.clone())) // This toggles notes
        .padding(2);
    
    card_details = card_details.push(notes_button); // Add notes button to the details column

    if is_expanded {
        card_details = card_details.push( // Add TextInput to details column if expanded
            TextInput::new(
                "Enter notes...",
                employee.notes.as_deref().unwrap_or(""),
            )
            .on_input(move |new_value| Message::NotesChanged(employee.user_id.clone(), new_value))
            .padding(5)
        );
    }
    
    // card_details is built up here...

    let interactive_card_area: Element<'a, Message> = if let Some(d_skill_id) = dragged_skill_id {
        // If a skill is being dragged, this card becomes a drop target (Button)
        Button::new(card_details) // card_details is the content of the button
            .on_press(Message::SkillDroppedOnCard(d_skill_id.clone(), employee.user_id.clone()))
            .padding(10)
            .width(Length::Fill)
            .into()
    } else {
        // Otherwise, it's just a passive container displaying card_details
        // Drag initiation for this card will be handled by global events, not an on_press here.
        Container::new(card_details) // card_details is the content of the container
            .padding(10) // Match button padding for consistent look
            .width(Length::Fill)
            .into()
    };

    // The rest of the styling container:
    Container::new(interactive_card_area)
        .width(Length::Fill)
        .style(if is_highlighted {
            struct CardHighlightStyle;
            impl iced::widget::container::StyleSheet for CardHighlightStyle {
                type Style = iced::Theme;
                fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
                    iced::widget::container::Appearance {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(0.8, 1.0, 0.8))), // Light green
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.0, 0.8, 0.0), // Darker green border
                            width: 1.0,
                            radius: iced::border::Radius::from(4.0), // Correct path
                        },
                        shadow: Default::default(),
                        text_color: None,
                    }
                }
            }
            iced::theme::Container::Custom(Box::new(CardHighlightStyle))
        } else {
            // Assuming iced::theme::Container::Box is the default/non-highlighted style.
            // If you have a specific normal card style, use that instead.
            iced::theme::Container::Box 
        })
        .into()
}