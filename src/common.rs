use anyhow::{anyhow, Result};
use std::path::Path;

/// Read files or directories of a directory. Not recursively
///
/// # Arguments
///
/// * `dir` - folder to read
/// * `text` - text a file/dir must contain to be returned
/// * `dir_filter` - Some(true): return found directories only, Some(false) return found files
///                  only, None: no filtering
///
pub fn read_dir(dir: &Path, text: Option<&str>, dir_filter: Option<bool>) -> Result<Vec<String>> {
    let readdir = std::fs::read_dir(dir);
    if readdir.is_err() {
        return Err(anyhow!("Folder not existing: {:?}", dir));
    }
    let mut res = Vec::new();
    for entry in readdir.unwrap() {
        let entry = entry.expect("error reading dir entry");
        if dir_filter == Some(true) && !entry.metadata().expect("error reading metadata").is_dir() {
            continue;
        }
        if dir_filter == Some(false) && !entry.metadata().expect("error reading metadata").is_file()
        {
            continue;
        }
        let filename = entry
            .path()
            .file_name()
            .expect("invalid file name")
            .to_string_lossy()
            .into_owned();
        if let Some(text) = text {
            if !filename.contains(text) {
                continue;
            }
        }
        res.push(filename);
    }
    Ok(res)
}

#[cfg(test)]
mod test {
    use std::fs::{create_dir_all, File};
    use std::io::prelude::*;
    use tempfile::{Builder, TempDir};

    use crate::read_dir;

    #[test]
    fn test_read_dir() {
        // arrange
        let temp = create_testdir();

        // act
        let dirs = read_dir(temp.path(), None, Some(true)).unwrap();
        let files = read_dir(temp.path(), None, Some(false)).unwrap();
        let both = read_dir(temp.path(), None, None).unwrap();

        // assert
        assert!(dirs.contains(&String::from("dir1")) && dirs.contains(&String::from("dir2")));
        assert!(dirs.len() == 2);
        assert!(files.contains(&String::from("file1")) && files.contains(&String::from("file2")));
        assert!(files.len() == 2);
        assert!(both.contains(&String::from("dir1")) && both.contains(&String::from("dir2")));
        assert!(both.contains(&String::from("file1")) && both.contains(&String::from("file2")));
        assert!(both.len() == 4);
    }

    fn create_testdir() -> TempDir {
        let tempdir = Builder::new()
            .prefix("rust-test-myrustutils")
            .tempdir()
            .unwrap();
        let dir1 = tempdir.path().join("dir1").join("subdir");
        create_dir_all(&dir1).unwrap();
        File::create(dir1.join("subfile1"))
            .unwrap()
            .write_all(b"test subfile 1")
            .unwrap();
        create_dir_all(tempdir.path().join("dir2")).unwrap();
        File::create(tempdir.path().join("file1"))
            .unwrap()
            .write_all(b"test file 1")
            .unwrap();
        File::create(tempdir.path().join("file2"))
            .unwrap()
            .write_all(b"test file 1")
            .unwrap();
        tempdir
    }
}
