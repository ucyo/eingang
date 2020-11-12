use pulldown_cmark::Options;

pub mod backend {
    pub const HOST: &str = "127.0.0.1";
    pub const PORT: &str = "8081";

    pub const BASE_FOLDER: &str = "/tmp";
    pub const THREAD_FOLDER: &str = "threads";
    pub const NOTE_FOLDER: &str = "notes";
    pub const TASK_FOLDER: &str = "tasks";
}

pub mod models {
    pub const TIME: &str = "%Y-%m-%d";
    pub const RENDER_MARKDOWN_TABLES: bool = true;
    pub const RENDER_MARKDOWN_FOOTNOTES: bool = false;
    pub const RENDER_MARKDOWN_STRIKETHROUGH: bool = true;
    pub const RENDER_MARKDOWN_TASKLISTS: bool = true;
    pub const RENDER_MARKDOWN_SMART_PUNCTUATION: bool = false;
}

pub mod frontend {
    pub const HOST: &str = "http://localhost";
    pub const PORT: &str = "8080";

    pub const KEY: &str = "eingang.model.store";
}

pub fn get_markdown_options() -> Options {
    let mut options = Options::empty();
    options.set(Options::ENABLE_TABLES, models::RENDER_MARKDOWN_TABLES);
    options.set(Options::ENABLE_FOOTNOTES, models::RENDER_MARKDOWN_FOOTNOTES);
    options.set(Options::ENABLE_STRIKETHROUGH, models::RENDER_MARKDOWN_STRIKETHROUGH);
    options.set(Options::ENABLE_TASKLISTS, models::RENDER_MARKDOWN_TASKLISTS);
    options.set(Options::ENABLE_SMART_PUNCTUATION, models::RENDER_MARKDOWN_SMART_PUNCTUATION);
    options
}
