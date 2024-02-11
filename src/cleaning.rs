//! # Finding and cleaning logic
//!
//! Module contianing the code that is used for finding and removing directories
use std::fs::{self};
use std::io;
use std::path::{Component, Path, PathBuf};
use tracing::{debug, error, info};

fn dir_name_in_collection(dir_path: &Path, collection: &[String]) -> bool {
    if let Some(Component::Normal(x)) = dir_path.components().last() {
        if let Some(str_x) = x.to_str() {
            return collection.contains(&String::from(str_x));
        };
    }

    false
}

/// Find directories name in artifacts and filles passed vector findings
///
/// Traverse the filesystem starting from the passed dir. If the element
/// is a directory and no link, Three things can happend (in the given order):
/// 1. Is the directory name one of the items in artifacts. In that case, add
///    the path is added to the findings and continue with the next item.
/// 2. Is the directory name one of the items in ignore. In that case, continue
///     with the next item.
/// 3. Recurively call the function again with the directpry and the max_depth
///     reduced by one
#[tracing::instrument(skip_all,parent = None)]
pub fn find_dirs(
    findings: &mut Vec<PathBuf>,
    dir: &Path,
    artifacts: &[String],
    ignore: &[String],
    max_depth: u16,
) -> io::Result<()> {
    if max_depth == 0 {
        debug!("Hit max depth in {:?}", dir);
        return Ok(());
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && !path.is_symlink() {
                if dir_name_in_collection(&path, artifacts) {
                    debug!("Found {:?}", path);
                    findings.push(path.clone());
                } else if dir_name_in_collection(&path, ignore) {
                    debug!("Ignoring {:?}", path);
                } else {
                    find_dirs(findings, &path, artifacts, ignore, max_depth - 1)?;
                }
            }
        }
    }
    Ok(())
}

/// Delete all passed directores
pub fn delete_all_artifacts(findings: &[PathBuf]) {
    info!("Starting deletion of {:?} directory", findings.len());
    for dir in findings {
        match fs::remove_dir_all(dir) {
            Ok(()) => debug!("Deleted: {:?}", dir),
            Err(e) => error!("Deleting {:?} returned {:?}", dir, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn run_is_cleanable_flag_found_match() {
        let path_str = "/some/path/artifact_dir_1";
        let test_path = PathBuf::from(path_str);
        let artifacts = vec![String::from("artifact_dir_1")];
        assert!(dir_name_in_collection(&test_path, &artifacts));
    }

    #[test]
    fn run_is_cleanable_flag_no_match() {
        let test_path = PathBuf::from("/some/path/artifact_dir_1");
        let artifacts = vec![String::from("artifact_dir_2")];
        assert!(!dir_name_in_collection(&test_path, &artifacts));
    }

    #[test]
    fn run_find_dir() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();
        let sub_dir_path = dir_path.join("subdir");

        fs::create_dir(&sub_dir_path).expect("Failed to create directory");

        let artifact_dir_1 = sub_dir_path.join("artifact");
        let artifact_dir_2 = dir_path.join("artifact");

        fs::create_dir(&artifact_dir_1).expect("Failed to create directory");
        fs::create_dir(&artifact_dir_2).expect("Failed to create directory");

        let artifacts = vec![String::from("artifact")];
        let mut findings: Vec<PathBuf> = Vec::new();
        let ignore: Vec<String> = Vec::new();

        find_dirs(&mut findings, dir_path, &artifacts, &ignore, 2)
            .expect("Finding dirs did not work");

        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0], artifact_dir_1);
        assert_eq!(findings[1], artifact_dir_2);
    }

    #[test]
    fn run_find_dir_max_depth() {
        let temp_dir = tempdir().unwrap();
        let dir_path: &Path = temp_dir.path();
        let sub_dir_path = dir_path
            .join("subdir_1")
            .join("subdir_2")
            .join("subdir_3")
            .join("artifact");
        fs::create_dir_all(&sub_dir_path).expect("Failed to create directory");

        let artifacts = vec![String::from("artifact")];
        let mut findings: Vec<PathBuf> = Vec::new();
        let ignore: Vec<String> = Vec::new();
        find_dirs(&mut findings, dir_path, &artifacts, &ignore, 2)
            .expect("Finding dirs did not work");
        assert_eq!(findings.len(), 0);

        find_dirs(&mut findings, dir_path, &artifacts, &ignore, 5)
            .expect("Finding dirs did not work");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0], sub_dir_path);
    }

    #[test]
    fn run_find_dir_test_ignore() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();
        let sub_dir_path = dir_path.join("subdir");

        fs::create_dir(&sub_dir_path).expect("Failed to create directory");

        let artifact_dir_1 = sub_dir_path.join("artifact");
        let artifact_dir_2 = dir_path.join("ignore_dir").join("artifact");

        fs::create_dir_all(&artifact_dir_1).expect("Failed to create directory");
        fs::create_dir_all(&artifact_dir_2).expect("Failed to create directory");

        let artifacts = vec![String::from("artifact")];
        let mut findings: Vec<PathBuf> = Vec::new();
        let ignore: Vec<String> = vec![String::from("ignore_dir")];

        find_dirs(&mut findings, dir_path, &artifacts, &ignore, 10)
            .expect("Finding dirs did not work");

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0], artifact_dir_1);
    }

    #[test]
    fn run_delete_all_artifact() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();

        let artifact_dir_1 = dir_path.join("artifact");
        let artifact_dir_2 = dir_path.join("some_sub_folder").join("artifact");
        let artifact_dir_2_subdir = artifact_dir_2.join("some_other_dir");

        fs::create_dir_all(&artifact_dir_1).expect("Failed to create directory");
        fs::create_dir_all(&artifact_dir_2_subdir).expect("Failed to create directory");

        let findings = vec![artifact_dir_1.clone(), artifact_dir_2.clone()];

        assert!(artifact_dir_1.exists());
        assert!(artifact_dir_2.exists());

        delete_all_artifacts(&findings);

        assert!(!artifact_dir_1.exists());
        assert!(!artifact_dir_2.exists());
    }
}
