use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::{stderr, BufRead, BufReader, Read, Write};
use std::num::NonZeroU32;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use glib::object::IsA;
use glib::prelude::ObjectExt;
use glib::subclass::SignalId;
use gtk4::prelude::WidgetExt;
use gtk4::Window;
use crate::finish_view::FinishView;
use crate::progress_bar::MyProgressBar;

pub fn run_dd_command(progress_bar: Arc<Mutex<MyProgressBar>>, iso: &OsStr, disk: &OsStr, bs: &OsStr,size: u64,finish_view: Arc<Mutex<FinishView>>) {
    /*let mut cmd = Command::new("dd")
        .arg("if=/dev/zero")
        .arg("of=/dev/null")
        .arg("bs=1M")
        .arg("count=100")
        .arg("status=progress")
        .arg("2>&1")
        .arg("/dev/stdout")*/
    let mut cmd = Command::new("/bin/bash")
        .arg("-c")
        .arg(format!("dd if={} of={} bs={} count={} status=progress && sync","/dev/zero","/dev/null","1M",100))
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start dd command");

    let stderr = cmd.stderr.take().expect("Failed to capture stderr");
    let error_reader = BufReader::new(stderr);

    println!("Reading dd output line by line");
    for line in error_reader.lines() {
        match line {
            Ok(line) => {
                if let Some(progress) = parse_dd_progress(&line, 0) {
                    let progress_bar = Arc::clone(&progress_bar);
                    let finish = Arc::clone(&finish_view);
                    glib::MainContext::default().spawn_local(async move {
                        let mut progress_bar = progress_bar.lock().unwrap();
                        let mut finish = finish.lock().unwrap();
                        progress_bar.set_progress(progress);
                        progress_bar.set_progress_text(&format!("{:.0}%", progress * 100.0));
                        if progress >= 1.0 {
                            progress_bar.set_status(true);
                            finish.show();
                        }
                    });
                }
            }
            Err(e) => {
                println!("Error reading line: {:?}", e);
            }
        }
    }
    cmd.wait().expect("Failed to wait on dd command");
}

fn parse_dd_progress(line: &str, size: u64) -> Option<f64> {
    if let Some(pos) = line.find("octets") {
        let parts: Vec<&str> = line[..pos].split_whitespace().collect();
        if let Some(bytes_str) = parts.last() {
            if let Ok(bytes) = bytes_str.parse::<f64>() {
                println!("Bytes: {}", bytes);
                return Some(bytes / (100.0 * 1024.0 * 1024.0)); // Assuming 100MB total for this example
            }
        }
    }
    None
}