use std::path::PathBuf;

pub struct AdvInput<'a> {
    pub day: u32,
    pub year: i32,
    /// Path to store the input files in
    pub formatted_path: Option<&'a str>,
}

impl<'a> AdvInput<'a> {
    pub fn new(day: u32, year: i32) -> Self {
        Self {
            day,
            year,
            formatted_path: None,
        }
    }
    pub fn set_formatted_path(mut self, pattern: &'a str) -> Self {
        self.formatted_path = Some(pattern);
        self
    }
    fn filename(&self) -> String {
        format!("day{}.input", self.day)
    }
    pub fn path(&self) -> PathBuf {
        match self.eval_path() {
            Some(path) => PathBuf::from(path),
            // default condition
            None => PathBuf::from("./inputs")
                .join(self.year.to_string())
                .join(self.filename()),
        }
    }
    pub fn request_url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        )
    }
    fn eval_path(&self) -> Option<String> {
        Some(
            self.formatted_path?
                .replace("{{day}}", &self.day.to_string())
                .replace("{{year}}", &self.year.to_string()),
        )
    }
}
