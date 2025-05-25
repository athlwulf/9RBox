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
    CardDragStarted(String),       // New: String is employee_id of the card being dragged
    CardDroppedOnBox(String, String), // New: (dragged_employee_id, target_box_id)
    SkillDragStarted(String),      // New: String is skill_id of the skill being dragged
    SkillDroppedOnCard(String, String), // New: (dragged_skill_id, target_employee_id)
    ClearBoxHighlight(String),    // New: String is box_id
    ClearCardHighlight(String),   // New: String is employee_id
    RefreshData,                  // New
    // Add other messages as UI interactions are defined
}