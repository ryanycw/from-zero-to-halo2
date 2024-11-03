// Final Challenge: Create a simple grade management system that combines
// all the concepts we've learned. The system should:
// 1. Store student grades using appropriate data types
// 2. Calculate average grades
// 3. Assign letter grades
// 4. Handle invalid inputs
// 5. Generate reports

#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<f64>,
    letter_grade: Option<char>,
}

impl Student {
    fn new(name: String) -> Student {
        Student {
            name,
            grades: Vec::new(),
            letter_grade: None,
        }
    }

    fn add_grade(&mut self, grade: f64) -> Result<(), String> {
        if grade < 0.0 || grade > 100.0 {
            return Err("Grade must be between 0 and 100".to_string());
        }
        self.grades.push(grade);
        self.calculate_letter_grade();
        Ok(())
    }

    fn calculate_average(&self) -> Option<f64> {
        if self.grades.is_empty() {
            None
        } else {
            Some(self.grades.iter().sum::<f64>() / self.grades.len() as f64)
        }
    }

    fn calculate_letter_grade(&mut self) {
        self.letter_grade = self.calculate_average().map(|avg| match avg {
            avg if avg >= 90.0 => 'A',
            avg if avg >= 80.0 => 'B',
            avg if avg >= 70.0 => 'C',
            avg if avg >= 60.0 => 'D',
            _ => 'F',
        });
    }

    fn generate_report(&self) -> String {
        let avg = self
            .calculate_average()
            .map(|a| a.to_string())
            .unwrap_or("No grades yet".to_string());

        let letter = self
            .letter_grade
            .map(|l| l.to_string())
            .unwrap_or("N/A".to_string());

        format!(
            "Student: {}\nGrades: {:?}\nAverage: {}\nLetter Grade: {}",
            self.name, self.grades, avg, letter
        )
    }
}

fn main() {
    let mut students = vec![
        Student::new("Alice".to_string()),
        Student::new("Bob".to_string()),
    ];

    // Add grades for Alice
    if let Err(e) = students[0].add_grade(85.0) {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = students[0].add_grade(92.0) {
        eprintln!("Error: {}", e);
    }

    // Add grades for Bob
    if let Err(e) = students[1].add_grade(75.0) {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = students[1].add_grade(88.0) {
        eprintln!("Error: {}", e);
    }

    // Generate reports
    for student in &students {
        println!("\n{}", student.generate_report());
    }
}
