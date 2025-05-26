#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabId {
    Employees,
    Skills,
}

#[derive(Debug, Clone)]
pub enum Message {
    EmployeeSelected(String), // Parameter is employee_id
    BoxClicked(String),       // Parameter is box_id (e.g., "1A", "2B")
    ScaleChanged(f32),
    TabSelected(TabId),
    CardClicked(String), // New: String is employee_id
    NotesChanged(String, String), // New: (employee_id, new_notes)
    RemoveSkillTag(String, String), // New: (employee_id, skill_id)
    CardDragStarted(String), // Parameter is employee_id
    CardDroppedOnBox(String, String, Option<(f32, f32)>), // Parameters are employee_id, box_id, drop_coordinates
    SkillDragStarted(String),      // New: String is skill_id of the skill being dragged
    SkillDroppedOnCard(String, String), // New: (dragged_skill_id, target_employee_id)
    ClearBoxHighlight(String),    // New: String is box_id
    ClearCardHighlight(String),   // New: String is employee_id
    RefreshData,                  // New
    CardPressed(String),          // New: User has pressed the mouse button on an employee card (employee_id)
    HandleGlobalEvent(iced::Event), // New: For events from the subscription
    DragCancelled,                 // New: If drag is cancelled (e.g., by Esc)
    // Add other messages as UI interactions are defined
}