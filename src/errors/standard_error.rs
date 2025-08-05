use crate::lexing::position::Position;
use simply_colored::*;
use std::fmt::Display;

#[derive(Clone)]
pub struct StandardError {
    pub text: String,
    pub pos_start: Position,
    pub pos_end: Position,
    pub help: Option<String>,
}

impl StandardError {
    pub fn new(text: &str, pos_start: Position, pos_end: Position, help: Option<&str>) -> Self {
        Self {
            text: text.to_string(),
            pos_start,
            pos_end,
            help: if help.is_some() {
                Some(help.unwrap().to_string())
            } else {
                None
            },
        }
    }

    pub fn format_code_as_messup(
        &self,
        text: &str,
        pos_start: &Position,
        pos_end: &Position,
    ) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut result = String::new();

        for i in pos_start.line_num..=pos_end.line_num {
            if let Some(line) = lines.get(i as usize) {
                result.push_str("   | ");
                result.push_str(line);
                result.push('\n');

                let col_start = if i == pos_start.line_num {
                    pos_start.column_num as usize
                } else {
                    0
                };

                let col_end = if i == pos_end.line_num - 1 {
                    pos_end.column_num as usize
                } else {
                    line.len()
                };

                let arrow_len = if col_end > col_start {
                    col_end - col_start
                } else {
                    1
                };

                let arrow_line = " ".repeat(col_start) + &"^".repeat(arrow_len);
                result.push_str(format!("   | {BOLD}{}{RESET}", &arrow_line).as_str());
                result.push_str("\n   | ");
            }
        }

        result.replace('\t', "")
    }
}

impl Display for StandardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(
            format!(
                "{DIM_RED}{BOLD}error:{RESET} {}\n   in: {}:{}:{}",
                self.text,
                self.pos_start.filename,
                self.pos_start.line_num + 1,
                self.pos_start.column_num,
            )
            .as_str(),
        );

        // this will print the '^' indicating where the issue is
        output.push_str(
            format!(
                "\n   + \n   | \n{}",
                self.format_code_as_messup(
                    &self.pos_start.file_contents,
                    &self.pos_start,
                    &self.pos_end,
                )
            )
            .as_str(),
        );

        if let Some(msg) = &self.help {
            output.push_str(format!("\n   + - > {DIM_GREEN}{ITALIC}help:{RESET} {msg}").as_str());
        } else {
            output.push_str("\n   + ");
        }

        output.push_str(
            format!(
                "\n{DIM_YELLOW}{BOLD}process finished with exit code {}{RESET}",
                -1
            )
            .as_str(),
        );

        write!(f, "{output}{RESET}")
    }
}
