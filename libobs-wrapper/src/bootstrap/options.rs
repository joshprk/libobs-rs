pub const GITHUB_REPO: &'static str = "sshcrack/libobs-builds";

#[derive(Debug, Clone)]
pub struct ObsBootstrapperOptions {
    pub(crate) repository: String,
    pub(crate) update: bool
}

impl ObsBootstrapperOptions {
    pub fn new() -> Self {
        ObsBootstrapperOptions {
            repository: GITHUB_REPO.to_string(),
            update: true
        }
    }

    pub fn set_repository(mut self, repository: &str) -> Self {
        self.repository = repository.to_string();
        self
    }

    pub fn get_repository(&self) -> &str {
        &self.repository
    }

    /// `true` if the updater should check for updates and download them if available.
    /// `false` if the updater should not check for updates and only install OBS if required.
    pub fn set_update(mut self, update: bool) -> Self {
        self.update = update;
        self
    }
}

impl Default for ObsBootstrapperOptions {
    fn default() -> Self {
        ObsBootstrapperOptions::new()
    }
}