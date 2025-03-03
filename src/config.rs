pub struct JournalConfig {
    pub name: String,
    pub field: String,
    pub description: String,
}

impl Default for JournalConfig {
    fn default() -> Self {
        Self {
            name: String::from(
                "African Academic Union - African Journal of Educational Technology",
            ),
            field: String::from("Educational Technology"),
            description: String::from("A leading journal in educational technology..."),
        }
    }
}

pub fn get_journal_config() -> JournalConfig {
    JournalConfig::default()
}
