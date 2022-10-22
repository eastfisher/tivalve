use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::LineWriter;
use std::sync::Mutex;

lazy_static! {
    pub static ref ADD_TABLE_WRITER: Mutex<LineWriter<File>> = Mutex::new(LineWriter::new(OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open("/tidb-deploy/tivalve/add_table.txt").unwrap()));
        // .open("/tmp/tivalve/add_table.txt").unwrap()));
}

lazy_static! {
    pub static ref ROW_EVENT_WRITER: Mutex<LineWriter<File>> = Mutex::new(LineWriter::new(OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open("/tidb-deploy/tivalve/row_event.txt").unwrap()));
}
