use flate2::write::GzEncoder;
use flate2::Compression;
use ignore::gitignore::Gitignore;
use std::error::Error;
use std::fs::{File, self};
use std::path::Path;
use tar::Builder;
use walkdir::WalkDir;

extern crate better_progress;

pub fn pack_folder<P: AsRef<Path>>(
    input_directory: P,
    output_file: P,
    exclude: Gitignore,
    show_progress: bool,
) -> Result<u64, Box<dyn Error>> {
    let mut circle: better_progress::SpinningCircle = better_progress::SpinningCircle::new();
    if show_progress {
        circle.set_job_title("Packing...");
    }

    let folder_path = input_directory.as_ref();
    let output_file = output_file.as_ref();

    let archive = File::create(output_file)?;
    let enc = GzEncoder::new(archive, Compression::default());
    let mut tar = Builder::new(enc);
    let mut total_count: i128 = 0;

    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(folder_path)?;
        let is_file = path.is_file();

        match exclude.matched_path_or_any_parents(path, !is_file) {
            ignore::Match::Ignore(_) => continue,
            _ => (),
        }

        if is_file {
            tar.append_path_with_name(path, name)?;
        } else if name.as_os_str().len() != 0 {
            tar.append_dir(name, path)?;
        }

        if show_progress {
            circle.set_job_title(&format!("{} files packed", total_count));
            circle.tick();
        }

        total_count += 1;
    }

    tar.finish()?;

    let metadata = fs::metadata(output_file)?;

    if show_progress {
        circle.jobs_done(true);
    }

    Ok(metadata.len())
}
