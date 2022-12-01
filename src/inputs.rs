use std::path::{Path, PathBuf};

pub struct AdvInput<'a> {
    pub day: u32,
    pub year: i32,
    /// Path to store the input files in
    pub directory: Option<&'a Path>,
}

impl<'a> AdvInput<'a> {
    pub fn new(day: u32, year: i32) -> Self {
        Self {
            day,
            year,
            directory: None,
        }
    }
    pub fn set_directory(mut self, directory: &'a Path) -> Self {
        self.directory = Some(directory);
        self
    }
    fn filename(&self) -> String {
        format!("day{}.input", self.day)
    }
    pub fn path(&self) -> Option<PathBuf> {
        Some(
            self.directory?
                .join(self.year.to_string())
                .join(self.filename()),
        )
    }
    pub fn request_url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        )
    }
}
