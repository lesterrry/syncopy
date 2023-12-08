use flate2::write::GzEncoder;
use flate2::Compression;
use ignore::gitignore::Gitignore;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use tar::Builder;
use walkdir::WalkDir;

extern crate better_progress;

pub fn pack_paths(
    input_paths: &Vec<String>,
    output_file: &String,
    exclude: Gitignore,
    show_progress: bool,
) -> Result<u64, Box<dyn Error>> {
    let mut circle: better_progress::SpinningCircle = better_progress::SpinningCircle::new();
    if show_progress {
        circle.set_job_title("Packing...");
    }

    let archive = File::create(&output_file)?;
    let enc = GzEncoder::new(archive, Compression::default());
    let mut tar = Builder::new(enc);
    let mut total_count: i128 = 0;

    let mut inc_circle = || {
        total_count += 1;
        if show_progress {
            circle.set_job_title(&format!("{} files packed", total_count));
            circle.tick();
        }
    };

    let should_ignore = |path: &Path, is_dir: bool| {
        if let ignore::Match::Ignore(_) = exclude.matched_path_or_any_parents(path, is_dir) {
            true
        } else {
            false
        }
    };

    for path in input_paths {
        let path = Path::new(&path);
        let is_dir = path.is_dir();
        let container = path.parent().unwrap_or(Path::new("/"));
        let name = path.strip_prefix(container)?;

        if should_ignore(path, is_dir) {
            continue;
        }

        if is_dir {
            tar.append_dir(name, path)?;
            for entry in WalkDir::new(path) {
                let entry = entry?;
                let entry_path = entry.path();
                let name = entry_path.strip_prefix(container)?;
                let is_file = entry_path.is_file();

                if should_ignore(entry_path, is_dir) {
                    continue;
                }

                if is_file {
                    tar.append_path_with_name(entry_path, name)?;
                } else if name.as_os_str().len() != 0 {
                    tar.append_dir(name, entry_path)?;
                }
                inc_circle();
            }
        } else if path.is_file() {
            tar.append_path_with_name(path, name)?;
            inc_circle();
        }
    }

    if total_count < 1 {
        return Err("Nothing was packed".into());
    }

    tar.finish()?;

    let metadata = fs::metadata(&output_file)?;

    if show_progress {
        circle.jobs_done(true);
    }

    Ok(metadata.len())
}
