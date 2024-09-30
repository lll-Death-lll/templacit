use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

pub fn move_file(file: &PathBuf, destination: &Path) -> Result<(), std::io::Error> {
    let name = file.file_name();
    if name.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Can't read file name",
        ));
    }

    let name = name.unwrap();
    let new_path = destination.join(name);

    fs::rename(file, new_path)?;

    Ok(())
}

pub fn create_subdir(name: &str, dir: &Path) -> Result<PathBuf, std::io::Error> {
    let path = dir.join(name);

    fs::create_dir_all(&path)?;

    if !path.exists() || !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Couldn't create directory",
        ));
    }

    Ok(path)
}

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn get_cit_dir() -> Result<PathBuf, std::io::Error> {
    let cit_dir = get_current_working_dir()?.join("minecraft/optifine/cit");
    if cit_dir.exists() && cit_dir.is_dir() {
        return Ok(cit_dir);
    }

    fs::create_dir_all(&cit_dir)?;
    if !cit_dir.exists() || !cit_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Couldn't create dir",
        ));
    }

    Ok(cit_dir)
}
