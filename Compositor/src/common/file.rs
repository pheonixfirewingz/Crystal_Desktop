use std::path::{Path, PathBuf};
use std::fs;
use std::io::Cursor;
use png::Decoder;

fn get_usr_share_path() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        // In debug mode, use `<Project_DIR>/../usr/local/share/`
        let project_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(project_dir).join("../usr/local/share/")
    }
    #[cfg(not(debug_assertions))]
    {
        // In release mode, use `/usr/share/`
        Path::new("/usr/share/").to_path_buf()
    }
}

pub fn read_from_usr_share(relative_path: &str) -> String {
    let usr_share_path = get_usr_share_path();
    let file_path = usr_share_path.join(relative_path);

    // Read the file and return its content as a String
    fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("Failed to read file {}: {}", file_path.display(), err))
}

pub fn read_from_usr_share_to_vec(relative_path: &str) -> Vec<u8> {
    let usr_share_path = get_usr_share_path();
    let file_path = usr_share_path.join(relative_path);
    fs::read(&file_path)
        .unwrap_or_else(|err| panic!("Failed to read file {}: {}", file_path.display(), err))
}

fn extract_rgba(png_data: &[u8]) -> Vec<u8> {
    // Decode the PNG data
    let decoder = Decoder::new(Cursor::new(png_data));
    let mut reader = decoder.read_info().expect("Failed to read PNG info");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("Failed to decode PNG frame");
    buf.truncate(info.buffer_size());
    buf
}