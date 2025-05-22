use crate::models::Employee;
use csv::{ReaderBuilder, WriterBuilder};
use std::io::{Read, Write};

/// Imports employees from a CSV data source.
///
/// # Arguments
/// * `reader` - A type that implements `std::io::Read` (e.g., a file or a byte slice).
///
/// # Returns
/// A `Result` containing a `Vec<Employee>` on success, or a `csv::Error` on failure.
pub fn import_employees_from_csv<R: Read>(reader: R) -> Result<Vec<Employee>, csv::Error> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(reader);
    let mut employees = Vec::new();
    for result in rdr.deserialize() {
        let employee: Employee = result?;
        employees.push(employee);
    }
    Ok(employees)
}

/// Exports a slice of employees to a CSV data sink.
///
/// # Arguments
/// * `employees` - A slice of `Employee` structs to export.
/// * `writer` - A type that implements `std::io::Write` (e.g., a file or a `Vec<u8>`).
///
/// # Returns
/// A `Result` indicating success or a `csv::Error` on failure.
pub fn export_employees_to_csv<W: Write>(
    employees: &[Employee],
    writer: W,
) -> Result<(), csv::Error> {
    let mut wtr = WriterBuilder::new().from_writer(writer);
    for employee in employees {
        wtr.serialize(employee)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Employee; // Already using crate::models::Employee

    fn get_sample_employees() -> Vec<Employee> {
        vec![
            Employee {
                user_id: "user1".to_string(),
                pr_group_2025: "Group A".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                current_position: "Developer".to_string(),
                current_temp_position: None,
                pr_2021: Some(4.0),
                pr_2022: Some(4.1),
                pr_2023: Some(4.2),
                pr_2024: Some(4.3),
                user_9box_2024: Some("Top Talent".to_string()),
                user_9box_2025: Some("Growth Potential".to_string()),
                notes: Some("High performer".to_string()),
                current_label: Some("Senior".to_string()),
                email: Some("john.doe@example.com".to_string()),
                manager_id: Some("manager1".to_string()),
                department: Some("Engineering".to_string()),
                location: Some("New York".to_string()),
                hire_date: Some("2020-01-15".to_string()),
            },
            Employee {
                user_id: "user2".to_string(),
                pr_group_2025: "Group B".to_string(),
                first_name: "Jane".to_string(),
                last_name: "Smith".to_string(),
                current_position: "Manager".to_string(),
                current_temp_position: Some("Acting Director".to_string()),
                pr_2021: None,
                pr_2022: Some(4.5),
                pr_2023: Some(4.6),
                pr_2024: None,
                user_9box_2024: None,
                user_9box_2025: Some("Key Player".to_string()),
                notes: None,
                current_label: None,
                email: Some("jane.smith@example.com".to_string()),
                manager_id: None,
                department: Some("Management".to_string()),
                location: Some("London".to_string()),
                hire_date: Some("2018-05-20".to_string()),
            },
        ]
    }

    #[test]
    fn test_import_simple_csv() {
        let csv_data = r#"User ID,PR Group 2025,First Name,Last Name,Current Position,Current Temp Position,PR2021,PR2022,PR2023,PR2024,User 9Box 2024,User 9Box 2025,Notes,Current Label,Email,Manager ID,Department,Location,Hire Date
user1,Group A,John,Doe,Developer,,4.0,4.1,4.2,4.3,Top Talent,Growth Potential,High performer,Senior,john.doe@example.com,manager1,Engineering,New York,2020-01-15
user2,Group B,Jane,Smith,Manager,Acting Director,,,4.5,4.6,,Key Player,,,jane.smith@example.com,,Management,London,2018-05-20
"#;
        let result = import_employees_from_csv(csv_data.as_bytes());
        assert!(result.is_ok(), "CSV import failed: {:?}", result.err());

        let employees = result.unwrap();
        assert_eq!(employees.len(), 2);

        let emp1 = &employees[0];
        assert_eq!(emp1.user_id, "user1");
        assert_eq!(emp1.first_name, "John");
        assert_eq!(emp1.pr_group_2025, "Group A");
        assert_eq!(emp1.pr_2024, Some(4.3));
        assert_eq!(emp1.notes, Some("High performer".to_string()));
        assert_eq!(emp1.current_temp_position, None);
        assert_eq!(emp1.email, Some("john.doe@example.com".to_string()));
        assert_eq!(emp1.manager_id, Some("manager1".to_string()));
        assert_eq!(emp1.department, Some("Engineering".to_string()));
        assert_eq!(emp1.location, Some("New York".to_string()));
        assert_eq!(emp1.hire_date, Some("2020-01-15".to_string()));


        let emp2 = &employees[1];
        assert_eq!(emp2.user_id, "user2");
        assert_eq!(emp2.first_name, "Jane");
        assert_eq!(emp2.pr_group_2025, "Group B");
        assert_eq!(emp2.pr_2021, None);
        assert_eq!(emp2.current_temp_position, Some("Acting Director".to_string()));
        assert_eq!(emp2.email, Some("jane.smith@example.com".to_string()));
        assert_eq!(emp2.manager_id, None);
    }

    #[test]
    fn test_export_simple_csv() {
        let employees = get_sample_employees();
        let mut buffer = Vec::new();

        let result = export_employees_to_csv(&employees, &mut buffer);
        assert!(result.is_ok(), "CSV export failed: {:?}", result.err());

        let csv_output = String::from_utf8(buffer).expect("CSV output is not valid UTF-8");

        // Construct expected CSV string carefully, matching Serde's output order and empty fields for None
        let expected_csv_header = "User ID,PR Group 2025,First Name,Last Name,Current Position,Current Temp Position,PR2021,PR2022,PR2023,PR2024,User 9Box 2024,User 9Box 2025,Notes,Current Label,Email,Manager ID,Department,Location,Hire Date\n";
        let expected_csv_emp1 = "user1,Group A,John,Doe,Developer,,4.0,4.1,4.2,4.3,Top Talent,Growth Potential,High performer,Senior,john.doe@example.com,manager1,Engineering,New York,2020-01-15\n";
        let expected_csv_emp2 = "user2,Group B,Jane,Smith,Manager,Acting Director,,,4.5,4.6,,Key Player,,,,jane.smith@example.com,,Management,London,2018-05-20\n";
        let expected_csv_data = format!("{}{}{}", expected_csv_header, expected_csv_emp1, expected_csv_emp2);

        assert_eq!(csv_output, expected_csv_data);
    }

    #[test]
    fn test_csv_round_trip() {
        let original_employees = get_sample_employees();

        // Export to CSV
        let mut buffer = Vec::new();
        let export_result = export_employees_to_csv(&original_employees, &mut buffer);
        assert!(export_result.is_ok(), "Export failed: {:?}", export_result.err());

        // Import from CSV
        let import_result = import_employees_from_csv(buffer.as_slice());
        assert!(import_result.is_ok(), "Import failed: {:?}", import_result.err());

        let imported_employees = import_result.unwrap();
        
        // Assert equality (requires Employee to derive PartialEq)
        assert_eq!(original_employees, imported_employees);
    }
}
