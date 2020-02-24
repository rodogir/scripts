use exitfailure::ExitFailure;
use failure::ResultExt;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{ffi, fs, path};

pub fn run(path: path::PathBuf, write: bool) -> Result<(), ExitFailure> {
    let is_simulation = !write;
    if is_simulation {
        println!("Simulation mode only!");
    }
    let path_name = path.clone().into_os_string();
    let dir =
        fs::read_dir(path).with_context(|_| format!("Could not find directory {:?}", path_name))?;
    rename_in_dir(dir, is_simulation)?;

    Ok(())
}

fn rename_in_dir(dir: fs::ReadDir, is_simulation: bool) -> Result<(), ExitFailure> {
    for entry in dir {
        if let Ok(entry) = entry {
            if entry.file_type()?.is_file() {
                let from_path = entry.path();
                let from_file_name = from_path.file_name().unwrap().to_str().unwrap();
                let to_file_name = create_new_name(from_file_name);
                if from_file_name != to_file_name {
                    let to_osstr_file_name = ffi::OsStr::new(&*to_file_name);
                    let mut to_path = from_path.clone();
                    to_path.set_file_name(to_osstr_file_name);
                    println!("Renaming {:?} to {:?}", from_path, to_path);
                    if !is_simulation {
                        fs::rename(from_path, to_path)?
                    }
                }
            }
        }
    }
    Ok(())
}

fn create_new_name(name: &str) -> std::borrow::Cow<'_, str> {
    lazy_static! {
        static ref EPISODE_RE: Regex = Regex::new(r"- S(\d\d)E(\d+) -").unwrap();
    }
    EPISODE_RE.replace(name, |caps: &Captures| {
        let season = &caps[1];
        let length = caps[2].chars().count();
        let src_ep = caps[2].parse::<i32>().unwrap();
        let second_ep = src_ep * 2;
        let first_ep = second_ep - 1;
        format!(
            "- S{}E{:0len$}-E{:0len$} -",
            season,
            first_ep,
            second_ep,
            len = length
        )
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_create_new_name() {
        assert_eq!(
            create_new_name("TV Show - S01E01 - Title.mkv"),
            "TV Show - S01E01-E02 - Title.mkv",
            "first episode"
        );
        assert_eq!(
            create_new_name("TV Show - S01E04 - Title.mkv"),
            "TV Show - S01E07-E08 - Title.mkv",
            "non first episode"
        );
        assert_eq!(
            create_new_name("TV Show - S01E01-E02 - Title.mkv"),
            "TV Show - S01E01-E02 - Title.mkv",
            "already multi episode"
        );
    }

    const TEST_DATA_DIR: &str = ".tmp_test_data";
    const FILE_NAMES: &[&str; 3] = &[
        "TV Show - S01E01 - [1080p].mkv",
        "TV Show - S01E04 - [1080p].mkv",
        "TV Show - S01E03-E04 - [1080p].mkv",
    ];

    fn setup_test_data() -> Result<(), ExitFailure> {
        if path::Path::new(TEST_DATA_DIR).exists() {
            fs::remove_dir_all(TEST_DATA_DIR)?;
        }
        fs::create_dir(TEST_DATA_DIR)?;

        for file_name in FILE_NAMES {
            fs::File::create(&format!("{}/{}", TEST_DATA_DIR, file_name))?;
        }
        Ok(())
    }

    #[test]
    fn check_rename_in_dir() {
        let res_setup = setup_test_data();
        assert!(res_setup.is_ok());

        let dir = fs::read_dir(TEST_DATA_DIR);

        let res = rename_in_dir(dir.unwrap(), false);
        assert!(res.is_ok());

        for file_name in FILE_NAMES.iter() {
            let new_name = &*create_new_name(file_name);
            let new_path = &format!("{}/{}", TEST_DATA_DIR, new_name);
            assert!(
                path::Path::new(new_path).exists(),
                format!("{} was not found", new_path)
            );
        }
    }
}
