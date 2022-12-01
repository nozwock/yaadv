use std::path::{Path, PathBuf};

pub struct AdvInput<'a> {
    pub day: u32,
    pub year: i32,
    /// Path to store the input files in
    directory: &'a Path,
}

impl<'a> AdvInput<'a> {
    pub fn new(day: u32, year: i32, directory: &'a Path) -> Self {
        Self {
            day,
            year,
            directory,
        }
    }
    pub fn set_directory(mut self, directory: &'a Path) -> Self {
        self.directory = directory;
        self
    }
    pub fn directory(&self) -> &Path {
        &self.directory
    }
    fn filename(&self) -> String {
        format!("day{}.input", self.day)
    }
    pub fn path(&self) -> PathBuf {
        self.directory.join(self.filename())
    }
    pub fn request_url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        )
    }
}
