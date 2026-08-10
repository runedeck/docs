## ADDED Requirements

### Requirement: Canonical Store Layout

A participating repository SHALL keep its only writable specification lifecycle tree at `docs/openspec/`, with `docs/.openspec-store/store.yaml` carrying a stable kebab-case store id and repository configuration carrying `spec.root: docs/openspec`.

#### Scenario: Both interfaces resolve the same tree

- **WHEN** `rune spec list` runs at the repository root and `openspec list` runs from `docs/`
- **THEN** both operate on `docs/openspec/` and report the same changes

#### Scenario: No generated mirror

- **WHEN** either CLI completes any lifecycle operation
- **THEN** no second canonical tree or generated mirror exists in the repository

### Requirement: Compatibility Baseline

Shared artifacts SHALL remain valid for the pinned OpenSpec compatibility baseline, OpenSpec v1.7.0, and compatibility SHALL be demonstrated by running both CLIs against the same files.

#### Scenario: Dual validation

- **WHEN** a repository adopts the layout or changes shared artifacts
- **THEN** `rune spec validate` and the pinned `openspec validate --all --no-interactive` both pass on the same tree

#### Scenario: Baseline advance

- **WHEN** a newer OpenSpec release is considered
- **THEN** the baseline advances only after the shared fixtures pass both implementations

### Requirement: Rune-Namespaced Extensions

A Rune-specific artifact or runtime file SHALL enter the shared tree only when it uses an OpenSpec-defined field, is additive content OpenSpec preserves unchanged, or lives in a Rune-namespaced path that OpenSpec ignores and Rune validation excludes from OpenSpec compatibility claims; Rune runtime state (the transaction journal, the archive lock, and the `.interop/` recovery mirror) SHALL remain gitignored.

#### Scenario: Runtime state stays untracked

- **WHEN** a Rune transaction is interrupted and leaves `.rune-transaction/journal.yaml` or `.rune-archive.lock` under `docs/openspec/`
- **THEN** the files remain untracked, and compatibility is claimed only after the interrupted-operation fixture proves the pinned OpenSpec validation and interactive view ignore them

#### Scenario: Incompatible extension

- **WHEN** a proposed extension fails all three conditions
- **THEN** it stays out of the shared tree until a compatibility fixture proves the pinned OpenSpec version preserves or safely ignores it
