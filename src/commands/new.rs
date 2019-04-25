use std::fs;

pub fn execute_command(template: &str, output: &str) {
    fs::write(output, template).expect(format!("Unable to create file {}", output));
}