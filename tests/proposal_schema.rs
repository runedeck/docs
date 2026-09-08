use std::path::{Path, PathBuf};
use std::process::Command;

fn check_proposals(paths: &[&Path]) -> std::process::Output {
    Command::new("mdschema")
        .arg("check")
        .arg("--schema")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/proposal.mdschema"))
        .args(paths)
        .output()
        .expect("install mdschema before running the proposal schema regression tests")
}

fn rendered_proposal() -> String {
    include_str!("../templates/spec/proposal.md")
        .replace("${CHANGE_TITLE}", "Schema regression")
        .replace("${CAPABILITIES}", "- New capability `schema-regression`.")
}

#[test]
#[ignore = "requires the installed mdschema CLI"]
fn existing_proposals_pass_the_real_schema() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let paths: Vec<PathBuf> = [
        "decision-artifacts",
        "development-lifecycle",
        "spec-compatibility",
    ]
    .into_iter()
    .map(|change| {
        root.join("docs/openspec/changes")
            .join(change)
            .join("proposal.md")
    })
    .collect();

    let output = check_proposals(&paths.iter().map(PathBuf::as_path).collect::<Vec<_>>());

    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[ignore = "requires the installed mdschema CLI"]
fn rendered_proposal_passes_the_real_schema() {
    let temporary = tempfile::TempDir::new().unwrap();
    let path = temporary.path().join("proposal.md");
    std::fs::write(&path, rendered_proposal()).unwrap();

    let output = check_proposals(&[&path]);

    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[ignore = "requires the installed mdschema CLI"]
fn missing_status_still_fails_the_real_schema() {
    let temporary = tempfile::TempDir::new().unwrap();
    let path = temporary.path().join("proposal.md");
    std::fs::write(&path, rendered_proposal().replace("status: proposed\n", "")).unwrap();

    let output = check_proposals(&[&path]);

    assert!(!output.status.success());
    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(diagnostics.contains("status"), "{diagnostics}");
}
