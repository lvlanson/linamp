use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use walkdir::WalkDir;

/// provides all necessary information to
/// build music library
///
pub struct MusicLibrary {
    /// path to musiclibrary
    library_path_buf: PathBuf,
    /// vector of strings of valid music files
    music_files: Vec<String>,
}

impl MusicLibrary {
    /// builds new `MusicLibrary` object
    ///
    /// * `library_path`: `PathBuf` to the root folder of music library
    pub fn new(library_path: PathBuf) -> Self {
        MusicLibrary {
            library_path_buf: library_path,
            music_files: Vec::new(),
        }
    }

    /// scan `library_path` for valid music files
    /// and save them to `music_files`
    pub fn scan_to_library(&mut self) {
        // iterate over all files recursively in path
        for entry in WalkDir::new(&self.library_path_buf)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            // open file descriptor for path entry
            let file = match File::open(entry.path()) {
                Ok(t) => t,
                Err(err) => {
                    dbg!("{}", err);
                    continue;
                }
            };

            // establishing MediaSourceStream to probe if format is supported
            let media_stream = MediaSourceStream::new(Box::new(file), Default::default());

            // extracting extension
            let extension = entry.path().extension();
            let mut hint: Hint = Hint::new();

            if let Some(ext) = extension {
                if let Some(ext_str) = ext.to_str() {
                    dbg!(ext_str);
                    hint.with_extension(ext_str);
                }
            }

            // probe format
            let mut probed = match symphonia::default::get_probe().format(
                &hint,
                media_stream,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            ) {
                Ok(t) => t,
                Err(_) => {
                    continue;
                }
            };
            let file_path: String = entry.path().display().to_string();
            println!("added {}", file_path);
            self.music_files.push(file_path);
        }
    }

    /// returns the list of valid music files
    pub fn get_files(&self) -> &Vec<String> {
        &self.music_files
    }
}
