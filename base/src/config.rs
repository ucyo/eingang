
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
}

pub mod frontend {
    pub const HOST: &str = "http://localhost";
    pub const PORT: &str = "8080";

    pub const KEY: &str = "eingang.model.store";
}
