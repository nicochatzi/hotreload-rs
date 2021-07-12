use crate::err::{HotreloadError, HotreloadResult};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

fn now() -> HotreloadResult<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn make_newer_path(path_without_extension: &str, extension: &str) -> HotreloadResult<PathBuf> {
    Ok(PathBuf::from(&format!(
        "{}-{}{}",
        path_without_extension,
        now()?,
        extension
    )))
}

fn split_path_at_extension(path_str: &str) -> HotreloadResult<(&str, &str)> {
    Ok(path_str.split_at(
        path_str
            .rfind('.')
            .ok_or_else(|| HotreloadError::FileName(path_str.to_owned()))?,
    ))
}

pub fn duplicate(path: &Path) -> HotreloadResult<PathBuf> {
    let path_str = path
        .to_str()
        .ok_or_else(|| HotreloadError::InvalidPath(path.to_path_buf()))?;
    let splits = split_path_at_extension(path_str)?;
    let new_path = make_newer_path(splits.0, splits.1)?;
    fs::copy(path, &new_path)?;
    Ok(new_path)
}

fn dll_file_name(lib_name: &str) -> String {
    if std::cfg!(target_os = "macos") {
        format!("lib{}.dylib", lib_name)
    } else if std::cfg!(target_os = "windows") {
        format!("{}.dll", lib_name)
    } else if std::cfg!(target_os = "linux") {
        format!("lib{}.so", lib_name)
    } else {
        panic!("Unsupported OS")
    }
}

pub fn dll(folder_relative_path: &str, lib_name: &str) -> HotreloadResult<PathBuf> {
    Ok(std::env::current_dir()?
        .join(folder_relative_path)
        .join(&dll_file_name(lib_name)))
}

#[cfg(test)]
mod test {
    use super::*;

    fn pause() {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    #[test]
    fn can_split_path_at_extension() {
        let (path, ext) = split_path_at_extension("some/path/with.file").unwrap();
        assert_eq!(path, "some/path/with");
        assert_eq!(ext, ".file");
    }

    #[test]
    fn can_make_paths_with_unique_names() {
        let (path, ext) = split_path_at_extension("oi.yml").unwrap();
        let newer_path = || {
            make_newer_path(path, ext)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        };
        let path_a = newer_path();
        let (new_path_a, new_ext_a) = split_path_at_extension(&path_a).unwrap();

        pause();

        let path_b = newer_path();
        let (new_path_b, new_ext_b) = split_path_at_extension(&path_b).unwrap();

        assert_eq!(ext, new_ext_a);
        assert_eq!(ext, new_ext_b);

        assert_ne!(path, new_path_a);
        assert_ne!(path, new_path_b);
        assert_ne!(new_path_a, new_path_b);
    }

    #[test]
    fn can_duplicate_file() {}
}
