use expect_test::expect_file;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{parse, SyntaxError};

#[test]
fn parse_smoke_test() {
    let source = "func main() {}";
    let parse = parse(&source);
    assert!(parse.ok().is_ok());
}

#[test]
fn parser_tests() {
    dir_tests(&test_data_dir(), &["ok"], "hast", |text, path| {
        let parse = parse(&text);
        let errors = parse.errors();
        assert_errors_are_absent(errors, path);
        parse.debug_tree()
    })
}

fn assert_errors_are_absent(errors: &[SyntaxError], path: &Path) {
    assert_eq!(errors, &[] as &[SyntaxError], "There should be no errors in the file {:?}", path.display(),);
}

fn test_data_dir() -> PathBuf {
    project_root().join("crates/parser/test_data")
}

/// Returns the path to the root directory of `hyper-lang` project.
pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).parent().unwrap().parent().unwrap().to_owned()
}

/// Calls callback `f` with input code and file paths for each `.rs` file in `test_data_dir`
/// subdirectories defined by `paths`.
///
/// If the content of the matching output file differs from the output of `f()`
/// the test will fail.
///
/// If there is no matching output file it will be created and filled with the
/// output of `f()`, but the test will fail.
fn dir_tests<F>(test_data_dir: &Path, paths: &[&str], outfile_extension: &str, f: F)
where
    F: Fn(&str, &Path) -> String,
{
    for (path, input_code) in collect_hyper_files(test_data_dir, paths) {
        let actual = f(&input_code, &path);
        let path = path.with_extension(outfile_extension);
        expect_file![path].assert_eq(&actual)
    }
}

/// Collects all `.rs` files from `dir` subdirectories defined by `paths`.
fn collect_hyper_files(root_dir: &Path, paths: &[&str]) -> Vec<(PathBuf, String)> {
    paths
        .iter()
        .flat_map(|path| {
            let path = root_dir.to_owned().join(path);
            hyper_files_in_dir(&path).into_iter()
        })
        .map(|path| {
            let text = fs::read_to_string(&path).expect("Could not read hyper file");
            (path, text)
        })
        .collect()
}

/// Collects paths to all `.rs` files from `dir` in a sorted `Vec<PathBuf>`.
fn hyper_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut acc = Vec::new();
    for file in fs::read_dir(&dir).unwrap() {
        let file = file.unwrap();
        let path = file.path();

        if path.to_str().unwrap_or_default().contains(".skip") {
            continue;
        }

        if path.extension().unwrap_or_default() == "hyper" {
            acc.push(path);
        }
    }
    acc.sort();
    acc
}
