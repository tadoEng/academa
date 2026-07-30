use std::path::{Path, PathBuf};
use std::process::Command;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn snapshot_root() -> PathBuf {
    workspace_root().join("tests/baseline/phase1/generated")
}

fn run_aet(args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_aet"))
        .current_dir(workspace_root())
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

fn assert_matches_snapshot(relative_output: &str) {
    let generated_root = workspace_root().join("dist").join(relative_output);
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
    let root = workspace_root();
    for article in ["vietnam-two-child-policy", "dengue-bangladesh"] {
        let article_path = format!("data/articles/{article}");
        run_aet(&["validate", &article_path]);
        run_aet(&["build", &article_path]);

        let out_dir = root.join("dist").join(article);
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

        assert_matches_snapshot(article);
    }

    run_aet(&["build-topic", "health-medicine"]);
    let topic_output = root.join("dist/topics/health-medicine");
    assert!(topic_output.join("vocabulary.pdf").is_file());
    assert_matches_snapshot("topics/health-medicine");
}
