pub const GITHUB_REPO: &'static str = "sshcrack/libobs-builds";

#[derive(Debug, Clone)]
pub struct ObsDownloaderOptions {
    pub(crate) repository: String,
}

impl ObsDownloaderOptions {
    pub fn new() -> Self {
        ObsDownloaderOptions {
            repository: GITHUB_REPO.to_string(),
        }
    }

    pub fn set_repository(mut self, repository: &str) -> Self {
        self.repository = repository.to_string();
        self
    }

    pub fn get_repository(&self) -> &str {
        &self.repository
    }
}

impl Default for ObsDownloaderOptions {
    fn default() -> Self {
        ObsDownloaderOptions::new()
    }
}