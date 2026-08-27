use super::*;
use crate::spec::model::{CanonicalSpec, Requirement};

fn requirement(name: &str, scenarios: &[&str]) -> Requirement {
    Requirement {
        name: name.to_string(),
        content: format!("### Requirement: {name}"),
        line: 1,
        scenarios: scenarios.iter().map(ToString::to_string).collect(),
    }
}

fn empty_result() -> ApplyResult {
    ApplyResult {
        summary: MergeSummary::default(),
        warnings: Vec::new(),
    }
}

#[test]
fn added_requirement_with_case_variant_name_is_rejected() {
    let mut canonical = CanonicalSpec::new("Cap", "chg");
    canonical.add_requirement(requirement("Temporary review state", &[]));
    let incoming = requirement("Temporary Review State", &[]);
    let mut result = empty_result();

    let issue = apply_added(
        &mut canonical,
        &[DeltaOperation::Added(incoming)],
        "Cap",
        &mut result,
    )
    .unwrap_err();

    assert!(issue.message.contains("cannot add existing requirement"));
    assert_eq!(canonical.requirement_count(), 1);
}

#[test]
fn added_requirement_identical_up_to_case_is_skipped() {
    let mut canonical = CanonicalSpec::new("Cap", "chg");
    let mut existing = requirement("Temporary review state", &[]);
    existing.content = "shared content".to_string();
    canonical.add_requirement(existing);
    let mut incoming = requirement("Temporary Review State", &[]);
    incoming.content = "shared content".to_string();
    let mut result = empty_result();

    apply_added(
        &mut canonical,
        &[DeltaOperation::Added(incoming)],
        "Cap",
        &mut result,
    )
    .unwrap();

    assert_eq!(result.summary.added, 0);
    assert_eq!(canonical.requirement_count(), 1);
}

#[test]
fn scenario_preservation_compares_occurrence_multiplicity() {
    let current = requirement("Repeated", &["Retry", "Retry"]);
    let incoming = requirement("Repeated", &["Retry"]);

    assert_eq!(
        missing_scenario_occurrences(&current, &incoming),
        vec!["Retry"]
    );
}
