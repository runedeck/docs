use super::*;
use crate::spec::model::Requirement;
use crate::spec::parse::{parse_canonical, parse_delta};

fn requirement_text(name: &str, action: &str) -> String {
    format!(
        "### Requirement: {name}\n\nThe tool SHALL {action}.\n\n#### Scenario: Review\n\n- WHEN a review completes\n- THEN the tool records the result\n"
    )
}

fn requirement(name: &str, scenarios: &[&str]) -> Requirement {
    Requirement {
        name: name.to_string(),
        content: format!("### Requirement: {name}"),
        line: 1,
        scenarios: scenarios.iter().map(ToString::to_string).collect(),
    }
}

#[test]
fn added_requirement_with_case_variant_name_is_rejected() {
    let original = format!(
        "## Requirements\n\n{}",
        requirement_text("Temporary review state", "record the result")
    );
    let mut canonical = parse_canonical(&original).unwrap();
    let operations = parse_delta(&format!(
        "## ADDED Requirements\n\n{}",
        requirement_text("Temporary Review State", "discard the result")
    ))
    .unwrap();

    let issue = apply_delta(&mut canonical, &operations, "Cap")
        .err()
        .unwrap();

    assert!(issue.message.contains("cannot add existing requirement"));
    assert_eq!(canonical.requirement_count(), 1);
    assert_eq!(canonical.render(), original);
}

#[test]
fn added_requirement_identical_up_to_case_is_skipped() {
    for line_ending in ["\n", "\r\n"] {
        let original = format!(
            "## Requirements\n\n{}",
            requirement_text("Temporary review state", "record the result")
        )
        .replace('\n', line_ending);
        let mut canonical = parse_canonical(&original).unwrap();
        let operations = parse_delta(&format!(
            "## ADDED Requirements\n\n{}",
            requirement_text("Temporary  Review State", "record the result")
        ))
        .unwrap();

        let result = apply_delta(&mut canonical, &operations, "Cap").unwrap();

        assert_eq!(result.summary.added, 0);
        assert_eq!(canonical.requirement_count(), 1);
        assert_eq!(canonical.render(), original);
        assert!(!canonical.changed());
    }
}

#[test]
fn ambiguous_legacy_names_reject_each_operation_without_mutation() {
    let original = format!(
        "## Requirements\n\n{}\n{}",
        requirement_text("Temporary review state", "record the first result"),
        requirement_text("Temporary Review State", "record the second result")
    );
    let deltas = [
        "## REMOVED Requirements\n\n### Requirement: Temporary Review State\n".to_string(),
        format!(
            "## MODIFIED Requirements\n\n{}",
            requirement_text("Temporary Review State", "record a changed result")
        ),
        "## RENAMED Requirements\n\n- FROM: `### Requirement: Temporary Review State`\n- TO: `### Requirement: New name`\n".to_string(),
        format!(
            "## ADDED Requirements\n\n{}",
            requirement_text("Temporary Review State", "record the second result")
        ),
    ];
    for delta in deltas {
        let mut canonical = parse_canonical(&original).unwrap();
        let operations = parse_delta(&delta).unwrap();

        let issue = apply_delta(&mut canonical, &operations, "Cap")
            .err()
            .unwrap();

        assert!(issue.message.contains("ambiguous requirement identity"));
        assert_eq!(canonical.render(), original);
        assert!(!canonical.changed());
    }
}

#[test]
fn later_ambiguous_addition_preserves_canonical_after_removal() {
    for line_ending in ["\n", "\r\n"] {
        let original = format!(
            "## Requirements\n\n{}\n{}\n{}",
            requirement_text("Removable", "record the old result"),
            requirement_text("Temporary review state", "record the first result"),
            requirement_text("Temporary Review State", "record the second result"),
        )
        .replace('\n', line_ending);
        let mut canonical = parse_canonical(&original).unwrap();
        let before = canonical.clone();
        let operations = parse_delta(&format!(
            "## REMOVED Requirements\n\n### Requirement: Removable\n\n## ADDED Requirements\n\n{}",
            requirement_text("Temporary Review State", "record another result"),
        ))
        .unwrap();

        let issue = apply_delta(&mut canonical, &operations, "Cap")
            .err()
            .unwrap();

        assert!(issue.message.contains("ambiguous requirement identity"));
        assert_eq!(canonical, before);
        assert_eq!(canonical.render(), original);
    }
}

#[test]
fn later_errors_preserve_canonical_after_each_operation_kind() {
    let original = format!(
        "## Requirements\n\n{}\n{}\n{}",
        requirement_text("Stable", "record the old result"),
        requirement_text("Temporary review state", "record the first result"),
        requirement_text("Temporary Review State", "record the second result"),
    );
    let deltas = [
        "## RENAMED Requirements\n\n- FROM: `### Requirement: Stable`\n- TO: `### Requirement: Renamed`\n- FROM: `### Requirement: Missing`\n- TO: `### Requirement: Other`\n".to_string(),
        "## REMOVED Requirements\n\n### Requirement: Stable\n\n### Requirement: Missing\n".to_string(),
        format!(
            "## MODIFIED Requirements\n\n{}\n{}",
            requirement_text("Stable", "record a changed result"),
            requirement_text("Missing", "record another result"),
        ),
        format!(
            "## ADDED Requirements\n\n{}\n{}",
            requirement_text("New requirement", "record a new result"),
            requirement_text("Temporary Review State", "record another result"),
        ),
    ];
    for delta in deltas {
        let mut canonical = parse_canonical(&original).unwrap();
        let before = canonical.clone();
        let operations = parse_delta(&delta).unwrap();

        assert!(apply_delta(&mut canonical, &operations, "Cap").is_err());
        assert_eq!(canonical, before);
    }
}

#[test]
fn failed_delta_preserves_changes_from_an_earlier_successful_delta() {
    let original = format!(
        "## Requirements\n\n{}",
        requirement_text("Stable", "record the old result"),
    );
    let mut canonical = parse_canonical(&original).unwrap();
    let initial = parse_delta(&format!(
        "## ADDED Requirements\n\n{}",
        requirement_text("Previously added", "record the earlier result"),
    ))
    .unwrap();
    apply_delta(&mut canonical, &initial, "Cap").unwrap();
    let before = canonical.clone();
    assert!(before.changed());
    let failing = parse_delta(
        "## REMOVED Requirements\n\n### Requirement: Stable\n\n### Requirement: Missing\n",
    )
    .unwrap();

    assert!(apply_delta(&mut canonical, &failing, "Cap").is_err());
    assert_eq!(canonical, before);
}

#[test]
fn case_only_rename_updates_the_parsed_heading() {
    let original = format!(
        "## Requirements\n\n{}",
        requirement_text("Temporary review state", "record the result")
    );
    let mut canonical = parse_canonical(&original).unwrap();
    let operations = parse_delta(
        "## RENAMED Requirements\n\n- FROM: `### Requirement: Temporary review state`\n- TO: `### Requirement: Temporary Review State`\n",
    )
    .unwrap();

    let result = apply_delta(&mut canonical, &operations, "Cap").unwrap();

    assert_eq!(result.summary.renamed, 1);
    assert_eq!(
        canonical.render(),
        original.replace("Temporary review state", "Temporary Review State")
    );
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
