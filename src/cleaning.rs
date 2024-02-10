use std::fs::{self};
use std::io;
use std::path::{Component, Path, PathBuf};
use tracing::{debug, info};

fn dir_name_in_collection(dir_path: &Path, collection: &[String]) -> bool {
    if let Some(Component::Normal(x)) = dir_path.components().last() {
        if let Some(str_x) = x.to_str() {
            return collection.contains(&String::from(str_x));
        };
    }

    false
}

#[tracing::instrument(skip_all,parent = None)]
pub fn find_dirs(
    findings: &mut Vec<PathBuf>,
    dir: &Path,
    artifacts: &[String],
    ignore: &[String],
    max_depth: i32,
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

pub fn delete_all_artifact(findings: &[PathBuf]) -> io::Result<()> {
    info!("Starting deletion");
    todo!()
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
        assert!(dir_name_in_collection(&test_path, &artifacts))
    }

    #[test]
    fn run_is_cleanable_flag_no_match() {
        let test_path = PathBuf::from("/some/path/artifact_dir_1");
        let artifacts = vec![String::from("artifact_dir_2")];
        assert!(!dir_name_in_collection(&test_path, &artifacts))
    }

    #[test]
    fn run_find_dir() {
        let temp_dir = tempdir().expect("...");
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
        let temp_dir = tempdir().expect("...");
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
        let temp_dir = tempdir().expect("...");
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
}