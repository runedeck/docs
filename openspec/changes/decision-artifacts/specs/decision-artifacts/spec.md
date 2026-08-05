## Purpose

Defines decision artifacts within the OpenSpec canon and their publication as durable architecture decision records.

## ADDED Requirements

### Requirement: Changes declare their decisions

Every active change MUST contain either one or more canonical ADR drafts under `decisions/` or exactly one `decisions/no-decision.md` marker with a non-empty reason. Drafts MUST use `status: proposed`, carry an allocated `id` of the form `SERIES-NNNN`, contain the canonical ADR sections, and contain no unresolved placeholders. Drafts and the marker MUST NOT coexist.

Compatible interfaces MUST preserve authored decision content and discover several drafts in stable repository-relative path order.

#### Scenario: Change declares decisions

- **WHEN** a change contains complete ADR drafts under `decisions/`
- **THEN** validation accepts each draft without rewriting its rationale

#### Scenario: Change declares no decision

- **WHEN** `decisions/no-decision.md` contains a reason and no drafts exist
- **THEN** validation accepts the declaration and archive creates no canonical ADR

#### Scenario: Decision declaration is invalid

- **WHEN** the declaration is absent, mixed, malformed, incomplete, or contains unresolved placeholders
- **THEN** validation identifies the affected paths and archive makes no canonical changes

### Requirement: Decision artifacts remain visible

Compatible interfaces MUST retain decision paths, parsed frontmatter, and authored bodies in change context. Interfaces with display, validation, or health surfaces MUST include decision artifacts and actionable contract findings.

#### Scenario: Context carries decision rationale

- **WHEN** context is requested for a change with an ADR draft
- **THEN** the output includes its path, parsed frontmatter, and authored body

#### Scenario: Health check finds unpublished decisions

- **WHEN** an archived change contains unpublished ADR drafts
- **THEN** health output identifies the drafts and the reconciliation action

### Requirement: Decision-aware archive publishes atomically

Before merging a change, a decision-aware archive MUST validate its declaration and plan every canonical write. A successful merge MUST publish accepted ADRs, provenance, and the decision index in the same recoverable operation as specification updates and the change move. Abandoning a change MUST publish no ADR.

Each published ADR MUST retain the `id` allocated to its draft, use `status: accepted`, and record its archived source path. Provenance MUST record that path and the source digest.

Publication MUST place each record at `<decisions-root>/<category>/<kebab-title>.md`. Filenames MUST NOT encode the identifier, and publication MUST NOT renumber an existing record.

#### Scenario: Merge publishes accepted decisions

- **WHEN** a completed change with valid drafts is merged through a decision-aware archive
- **THEN** each draft becomes a separate accepted ADR with matching provenance and an index entry

#### Scenario: Publication cannot complete

- **WHEN** a destination is invalid, conflicts, or publication is interrupted
- **THEN** the operation fails or recovers without a partial specification update or duplicate ADR

#### Scenario: Change is abandoned

- **WHEN** a change is archived as abandoned
- **THEN** its decision artifacts remain historical context and no canonical ADR is created

### Requirement: Preserving archives reconcile idempotently

An archive that does not publish decisions MUST retain them inside the archived change. Reconciliation MUST validate and publish pending drafts in the same accepted record shape as decision-aware archive.

The archived source path and digest MUST identify publication. A matching path and digest MUST be a no-op; a matching path with a different digest MUST fail closed.

#### Scenario: Reconcile pending decisions

- **WHEN** reconciliation finds valid unpublished drafts
- **THEN** it publishes accepted ADRs with provenance for their archived paths and digests

#### Scenario: Reconcile an existing publication

- **WHEN** an archived path and digest already identify a canonical ADR
- **THEN** reconciliation reports it as published without allocating or rewriting an ADR

#### Scenario: Published source changed

- **WHEN** an archived path matches provenance but its digest differs
- **THEN** reconciliation fails and identifies the modified source

### Requirement: ADR identifiers are allocated safely

Draft creation, import, archive publication, and reconciliation MUST coordinate `id` allocation so concurrent operations cannot assign one identifier to different decisions. An allocated identifier MUST NOT be reused, and gaps in a series are valid. Multi-draft publication MUST use stable source-path order.

#### Scenario: Concurrent drafts share a series

- **WHEN** concurrent operations allocate identifiers in the same series
- **THEN** each decision receives a distinct identifier and no existing file is overwritten

#### Scenario: Decision is superseded

- **WHEN** a later decision replaces an accepted record
- **THEN** the existing identifier and filename are unchanged and the relationship is recorded in frontmatter
