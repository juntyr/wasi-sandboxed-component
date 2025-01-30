use std::{
    fs::{self, File},
    hash::{BuildHasher, DefaultHasher, Hasher, RandomState},
    io::{self, BufReader},
    path::Path,
};

use tempdir::TempDir;
use walkdir::{DirEntry, WalkDir};

pub use anyhow::Result;

#[expect(clippy::too_many_lines)]
#[expect(clippy::missing_errors_doc)]
pub fn check_is_locked(wit: impl AsRef<Path>) -> Result<()> {
    let wit = wit.as_ref();

    let deps_file = fs::read_to_string(wit.join("deps.toml"))?;
    let deps_lock = fs::read_to_string(wit.join("deps.lock"))?;

    let deps = TempDir::new("deps")?;

    let lock = wit_deps::lock(
        Some(&wit),
        deps_file,
        Some(deps_lock),
        deps.path().join("deps"),
    );
    let lock = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?
        .block_on(lock)?;

    if lock.is_some() {
        return Err(anyhow::anyhow!("lock file has changed"));
    }

    let old_wit = wit.join("deps");
    let new_wit = deps.path().join("deps");

    let mut old_deps = WalkDir::new(&old_wit)
        .min_depth(1)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();
    let mut new_deps = WalkDir::new(&new_wit)
        .min_depth(1)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();

    let (mut old_dep, mut new_dep) = (
        old_deps.next().transpose()?,
        new_deps.next().transpose()?,
    );

    loop {
        // skip indirect dependency deps files, lock files, and directories,
        //  since indirect dependencies are flattened into the main dendencies
        let skip = |dep: &Option<DirEntry>| match dep {
            Some(dep) if dep.path().ends_with("deps") && dep.file_type().is_dir() => {
                Some(Skip::Directory)
            },
            Some(dep)
                if (dep.path().ends_with("deps.toml") && dep.file_type().is_file())
                    || (dep.path().ends_with("deps.lock") && dep.file_type().is_file()) =>
            {
                Some(Skip::File)
            },
            _ => None,
        };

        if let Some(old_skip) = skip(&old_dep) {
            if matches!(old_skip, Skip::Directory) {
                old_deps.skip_current_dir();
            }
            old_dep = old_deps.next().transpose()?;
            continue;
        }
        if let Some(new_skip) = skip(&new_dep) {
            if matches!(new_skip, Skip::Directory) {
                new_deps.skip_current_dir();
            }
            new_dep = new_deps.next().transpose()?;
            continue;
        }

        // check that both have the same number of files
        let (some_old_dep, some_new_dep) = match (old_dep, new_dep) {
            (Some(old_dep), Some(new_dep)) => (old_dep, new_dep),
            (None, None) => break,
            (Some(extra), None) => {
                return Err(anyhow::anyhow!(
                    "{} is extraneous in deps",
                    extra.path().display()
                ))
            },
            (None, Some(missing)) => {
                return Err(anyhow::anyhow!(
                    "{} is missing from deps",
                    missing.path().display()
                ))
            },
        };

        // strip the file path prefixes to make them comparable
        let old_dep_path = some_old_dep
            .path()
            .strip_prefix(&old_wit)?;
        let new_dep_path = some_new_dep
            .path()
            .strip_prefix(&new_wit)?;

        // check that the next file path and type match
        if old_dep_path != new_dep_path {
            return Err(anyhow::anyhow!(
                "file name mismatch between {} and {} in deps",
                old_dep_path.display(),
                new_dep_path.display(),
            ));
        }
        if some_old_dep.file_type() != some_new_dep.file_type() {
            return Err(anyhow::anyhow!(
                "file type mismatch for {}",
                old_dep_path.display()
            ));
        }

        // we can only compare the binary contents of files
        if !some_old_dep.file_type().is_file() {
            old_dep = old_deps.next().transpose()?;
            new_dep = new_deps.next().transpose()?;

            continue;
        }

        let mut old_file = BufReader::new(File::open(some_old_dep.path())?);
        let mut new_file = BufReader::new(File::open(some_new_dep.path())?);

        let rng = RandomState::new();
        let mut old_hasher = HashWriter {
            hasher: rng.build_hasher(),
        };
        let mut new_hasher = HashWriter {
            hasher: rng.build_hasher(),
        };

        // hash the file contents
        io::copy(&mut old_file, &mut old_hasher)?;
        io::copy(&mut new_file, &mut new_hasher)?;

        let (old_hash, new_hash) = (old_hasher.hasher.finish(), new_hasher.hasher.finish());

        // check that the file content hashes match
        if old_hash != new_hash {
            return Err(anyhow::anyhow!(
                "file hash mismatch for {}",
                old_dep_path.display()
            ));
        }

        old_dep = old_deps.next().transpose()?;
        new_dep = new_deps.next().transpose()?;
    }

    deps.close()?;

    Ok(())
}

enum Skip {
    File,
    Directory,
}

struct HashWriter {
    hasher: DefaultHasher,
}

impl io::Write for HashWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, io::Error> {
        self.hasher.write(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
