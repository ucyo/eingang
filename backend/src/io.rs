use crate::{BASE_FOLDER, NOTE_FOLDER, TASK_FOLDER, THREAD_FOLDER};
use eingang::models::{Note, Task, Thread, Idable};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

/// Information about the location of each content type
pub enum Location {
    Note,
    Thread,
    Task,
}

impl Location {
    pub fn get_basefolder(&self) -> PathBuf {
        match self {
            Location::Note => Path::new(BASE_FOLDER).join(NOTE_FOLDER),
            Location::Task => Path::new(BASE_FOLDER).join(TASK_FOLDER),
            Location::Thread => Path::new(BASE_FOLDER).join(THREAD_FOLDER),
        }
    }
    pub fn create_filename(&self, uuid: &String) -> PathBuf {
        let filename = format!("{}.json", uuid);
        match self {
            Location::Note => Location::Note.get_basefolder().join(filename),
            Location::Task => Location::Task.get_basefolder().join(filename),
            Location::Thread => Location::Thread.get_basefolder().join(filename),
        }
    }
}

pub fn save_task(task: &Task) {
    let file = Location::Task.create_filename(&task.get_uuid().to_string());
    let buffer = File::create(file).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &task).unwrap();
    writer.flush().unwrap();
}

pub fn save_note(note: &Note) {
    let file = Location::Note.create_filename(&note.get_uuid().to_string());
    let buffer = File::create(file).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &note).unwrap();
    writer.flush().unwrap();
}

pub fn read_task_filepath(file: &PathBuf) -> Result<Task, serde_json::Error> {
    let buffer = File::open(file).unwrap();
    let rdr = std::io::BufReader::new(buffer);
    serde_json::from_reader(rdr)
}

pub fn read_note_filepath(file: &PathBuf) -> Result<Note, serde_json::Error> {
    let buffer = File::open(file).unwrap();
    let rdr = std::io::BufReader::new(buffer);
    serde_json::from_reader(rdr)
}

pub fn read_task(uuid: &String) -> Result<Task, serde_json::Error> {
    let file = Location::Task.create_filename(uuid);
    read_task_filepath(&file)
}

pub fn read_note(uuid: &String) -> Result<Note, serde_json::Error> {
    let file = Location::Note.create_filename(uuid);
    read_note_filepath(&file)
}

pub fn read_thread(uuid: &String) -> Result<Thread, serde_json::Error> {
    let file = Location::Thread.create_filename(uuid);
    read_thread_filepath(&file)
    // TODO only difference to other read methods is the output
    // TODO Maybe using actual UUID is better
}

pub fn read_thread_filepath(file: &PathBuf) -> Result<Thread, serde_json::Error> {
    let buffer = File::open(file).unwrap();
    let rdr = std::io::BufReader::new(buffer);
    serde_json::from_reader(rdr)
    // TODO only difference to other read methods is the output
}

pub fn save_thread(thread: &Thread) {
    let file = Location::Thread.create_filename(&thread.get_uuid().to_string());
    let buffer = File::create(file).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &thread).unwrap();
    writer.flush().unwrap();
    // TODO only difference to other save methods is the input type
}
