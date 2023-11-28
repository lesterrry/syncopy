use flate2::write::GzEncoder;
use flate2::Compression;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tar::Builder;
use walkdir::WalkDir;

extern crate progress;

pub fn pack_folder<P: AsRef<Path>>(
    input_directory: P,
    output_file: P,
    show_progress: bool,
) -> Result<(), Box<dyn Error>> {
    let mut circle: progress::SpinningCircle = progress::SpinningCircle::new();
    if show_progress {
        circle.set_job_title("Packing...");
    }

    let folder_path = input_directory.as_ref();
    let output_file = output_file.as_ref();

    let tar_gz = File::create(output_file)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    let mut total_count: i128 = 0;

    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(folder_path)?;

        if path.is_file() {
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

    if show_progress {
        circle.jobs_done();
    }

    Ok(())
}
