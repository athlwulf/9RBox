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
    // Add other messages as UI interactions are defined
}