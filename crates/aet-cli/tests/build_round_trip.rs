use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root should resolve")
}

fn snapshot_root() -> PathBuf {
    workspace_root().join("tests/baseline/phase1/generated")
}

struct TestWorkspace {
    root: PathBuf,
}

impl TestWorkspace {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "aet-phase1-baseline-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir(&root).expect("fresh test workspace should be created");
        copy_directory(
            &workspace_root().join("data/articles"),
            &root.join("data/articles"),
        );
        Self { root }
    }
}

impl Drop for TestWorkspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn copy_directory(source: &Path, destination: &Path) {
    std::fs::create_dir_all(destination).expect("fixture directory should be created");
    for entry in std::fs::read_dir(source).expect("fixture directory should be readable") {
        let entry = entry.expect("fixture directory entry should be readable");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_directory(&source_path, &destination_path);
        } else {
            std::fs::copy(&source_path, &destination_path)
                .expect("fixture file should be copied into the test workspace");
        }
    }
}

fn run_aet(current_dir: &Path, args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_aet"))
        .current_dir(current_dir)
        .args(args)
        .output()
        .expect("aet command should run");

    assert!(
        output.status.success(),
        "aet {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn deterministic_files(root: &Path) -> Vec<PathBuf> {
    fn visit(root: &Path, current: &Path, files: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(current).expect("snapshot directory should be readable") {
            let path = entry.expect("directory entry should be readable").path();
            if path.is_dir() {
                visit(root, &path, files);
            } else if path.file_name().and_then(|name| name.to_str()) != Some("vocabulary.pdf") {
                files.push(
                    path.strip_prefix(root)
                        .expect("file should be beneath comparison root")
                        .to_path_buf(),
                );
            }
        }
    }

    let mut files = Vec::new();
    visit(root, root, &mut files);
    files.sort();
    files
}

fn assert_matches_snapshot(dist_root: &Path, relative_output: &str) {
    let generated_root = dist_root.join(relative_output);
    let expected_root = snapshot_root().join(relative_output);
    let generated_files = deterministic_files(&generated_root);
    let expected_files = deterministic_files(&expected_root);

    assert_eq!(
        generated_files, expected_files,
        "deterministic output file set changed for {relative_output}"
    );

    for relative_file in expected_files {
        let generated = std::fs::read(generated_root.join(&relative_file))
            .expect("generated output should be readable");
        let expected = std::fs::read(expected_root.join(&relative_file))
            .expect("baseline snapshot should be readable");
        assert_eq!(
            generated,
            expected,
            "generated output changed: {}",
            relative_file.display()
        );
    }
}

#[test]
fn build_round_trip_creates_outputs_for_both_phase1_fixtures() {
    let test_workspace = TestWorkspace::new();
    let dist_root = test_workspace.root.join("dist");
    for article in ["vietnam-two-child-policy", "dengue-bangladesh"] {
        let article_path = format!("data/articles/{article}");
        run_aet(&test_workspace.root, &["validate", &article_path]);
        run_aet(&test_workspace.root, &["build", &article_path]);

        let out_dir = dist_root.join(article);
        for file in [
            "vocabulary.pdf",
            "anki-basic.tsv",
            "anki-cloze.tsv",
            "anki-production.tsv",
        ] {
            assert!(
                out_dir.join(file).is_file(),
                "expected {}",
                out_dir.join(file).display()
            );
        }

        assert_matches_snapshot(&dist_root, article);
    }

    run_aet(&test_workspace.root, &["build-topic", "health-medicine"]);
    let topic_output = dist_root.join("topics/health-medicine");
    assert!(topic_output.join("vocabulary.pdf").is_file());
    assert_matches_snapshot(&dist_root, "topics/health-medicine");
}
