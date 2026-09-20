## ADDED Requirements

### Requirement: The lifecycle tree lives at the native root

This repository MUST keep its specification lifecycle tree at `docs/changes/` and `docs/specs/`, MUST carry no `openspec/` or `docs/openspec/` directory, and MUST set no `spec.root` override. `rune spec doctor` and `rune docs check` MUST pass on that tree.

#### Scenario: Doctor resolves the native root

- **WHEN** `rune spec doctor` runs at the repository root with no `config.yaml`
- **THEN** it reports the tree under `docs/` as healthy and names no `docs/openspec/` path

#### Scenario: Stray tree appears

- **WHEN** a change adds an `openspec/` directory beside `docs/`
- **THEN** the reviewer moves it with `rune spec import --openspec` before the change lands

### Requirement: Capability and change ids carry three words

Every capability directory under `docs/specs/` and every change id under `docs/changes/` MUST carry at least three hyphen-separated words, and every requirement statement MUST stay at or under 100 words.

#### Scenario: Two-word capability

- **WHEN** a delta targets `specs/review-integration/`
- **THEN** `rune spec doctor` fails with `capability-name-short`
