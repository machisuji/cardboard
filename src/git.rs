extern crate git2;

use self::git2::{Repository, Oid, Signature, Error, IndexAddOption, ResetType};
use std::path::Path;

pub fn open_or_init(path_name: &str) -> Repository {
    match Repository::open(path_name) {
        Ok(repo) => repo,
        Err(_) => init(path_name)
    }
}

pub fn init(path_name: &str) -> Repository {
    match Repository::init(path_name) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to init git repo at `{}`: {}", path_name, e)
    }
}

pub fn open(path_name: &str) -> Repository {
    match Repository::open(path_name) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open git repo at `{}`: {}", path_name, e)
    }
}

pub fn commit_file(file_name: &str, message: &str, repo: &Repository) -> Result<Oid, Error> {
    let author = Signature::now("Cardboard", "cardboard@goldsaucer.co.uk").unwrap();
    let oid = repo.head().ok().and_then(|state| state.target()).unwrap();
    let parent = repo.find_commit(oid).unwrap();

    let mut index = repo.index().unwrap();

    index.add_path(Path::new(file_name))?;

    let index_oid = index.write_tree_to(&repo).unwrap();
    let tree = repo.find_tree(index_oid).unwrap();

    let result = repo.commit(Some("HEAD"), &author, &author, message, &tree, &[&parent]);

    result.and_then(|oid| {
        repo.find_object(oid, None).and_then(|object| {
            repo.reset(&object, ResetType::Hard, None).map(|_| oid)
        })
    })
}

pub fn initial_commit(path_name: &str, message: &str, repo: &Repository) -> Result<Oid, Error> {
    let author = Signature::now("Cardboard", "cardboard@goldsaucer.co.uk").unwrap();
    let mut index = repo.index().unwrap();

    index.add_all([path_name].iter(), IndexAddOption::empty(), None)?;

    let index_oid = index.write_tree_to(&repo).unwrap();
    let tree = repo.find_tree(index_oid).unwrap();

    let result = repo.commit(Some("HEAD"), &author, &author, message, &tree, &[]);

    result.and_then(|oid| {
        repo.find_object(oid, None).and_then(|object| {
            repo.reset(&object, ResetType::Hard, None).map(|_| oid)
        })
    })
}
