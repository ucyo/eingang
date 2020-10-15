use crate::{BASE_FOLDER, NOTE_FOLDER};
use eingang::models::Note;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub(crate) fn create_filepath(uuid: String) -> PathBuf {
    let basename = format!("{}.json", uuid);
    Path::new(BASE_FOLDER).join(NOTE_FOLDER).join(basename)
}

pub fn save_note(note: &Note) {
    let file = create_filepath(note.meta.uuid.to_string());
    let buffer = File::create(file).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &note).unwrap();
    writer.flush().unwrap();
}

pub fn read_note_filepath(file: &PathBuf) -> Note {
    let buffer = File::open(file).unwrap();
    let rdr = std::io::BufReader::new(buffer);
    let note: Note = serde_json::from_reader(rdr).unwrap();
    note
}

pub fn read_note(uuid: String) -> Note {
    let file = create_filepath(uuid);
    read_note_filepath(&file)
}
