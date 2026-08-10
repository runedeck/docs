# RuneSpec and OpenSpec Compatibility Design

## Problem Statement

RuneSpec and OpenSpec provide similar specification workflows through different command interfaces. RuneSpec has its own proposal, browsing, validation, doctor, archive, context, transaction, recovery, rendering, and library APIs. OpenSpec provides its own lifecycle commands, interactive view, status, instructions, templates, schemas, stores, worksets, and tool integrations.[OPENCLI]

Routing one command interface into the other would couple unrelated command grammars and make behavior depend on executable availability. Maintaining separate writable trees would create synchronization and review problems.

Both interfaces instead need one canonical artifact tree. OpenSpec is the compatibility baseline for shared artifacts. RuneSpec remains independent and can develop additional behavior without replacing or wrapping OpenSpec.

## Proposed Approach

### Independent interfaces

The two CLIs remain separate:

```text
rune spec ...
openspec ...
```

`rune spec` always executes RuneSpec. `openspec` always executes OpenSpec. Rune has no `spec.provider` setting, OpenSpec passthrough, executable detection, command fallback, or mixed routing table.

OpenSpec-only capabilities remain available through the OpenSpec CLI. RuneSpec can implement comparable or different capabilities over time without changing OpenSpec commands.

### Canonical repository layout

Each participating repository uses this layout:

```text
<repository>/
├── config.yaml
└── docs/
    ├── .openspec-store/
    │   └── store.yaml
    └── openspec/
        ├── config.yaml
        ├── specs/
        └── changes/
```

`docs/openspec/` is the only writable lifecycle tree. RuneSpec resolves it through repository configuration:

```yaml
spec:
    root: docs/openspec
```

`<repository>/docs` is an OpenSpec store path because it contains the required `openspec/` tree. The tracked `docs/.openspec-store/store.yaml` gives the store a stable, registry-unique kebab-case identifier.[OPENSTORE]

No generated mirror exists.

### RuneSpec capability preservation

All existing RuneSpec behavior remains supported:

- `propose`, including repeatable capabilities and optional design scaffolding
- `list`, including specification listing and progress sorting
- `show`, including automatic change or capability resolution and stable human and JSON output
- change-specific `context`
- Rune's semantic and mdschema-backed validation
- Rune's read-only tree and transaction doctor
- transactional archive, including `--abandon`
- root configuration, autodetection, journal-only roots, and transaction-journal tie-breaking
- import and export as recovery tools
- transaction journaling, locking, confinement, verification, recovery, and failure-injection seams
- typed result APIs, pure renderers, printing wrappers, JSON shapes, golden fixtures, clap integration, and pinned public signatures

The compatibility work does not delete, rename, hide, or make unreachable any current RuneSpec command, flag, output form, root mode, transaction path, test, or public library API.

The deferred printing-wrapper and clap-ownership cleanup does not proceed as part of this design. Any later removal requires a separate explicit compatibility decision.

### OpenSpec capability access

Users invoke OpenSpec directly for its complete interface:

```text
openspec view
openspec status --change add-search
openspec instructions apply --change add-search
openspec templates --schema spec-driven
openspec schemas --json
openspec new change add-search
```

OpenSpec v1.7.0 accepts `--store <id>` for the interactive view and other store-aware commands. Interactive use can select the registered repository directly or discover the nearest root from `docs/`:

```sh
openspec view --store <repository-store-id>
```

This preserves OpenSpec's terminal behavior, updates, completions, schema system, stores, and future command surface without a Rune adapter.[OPENCLI]

### Artifact compatibility contract

OpenSpec's supported artifact format is the common denominator. RuneSpec can add behavior when the resulting tree remains valid for the pinned OpenSpec compatibility baseline.

A Rune-specific extension must satisfy one of these conditions:

- It uses an OpenSpec-defined field or artifact.
- It is additive content that OpenSpec preserves unchanged.
- It is stored in a Rune-namespaced path that OpenSpec ignores and Rune's validation excludes from OpenSpec compatibility claims.

Rune transaction runtime state uses the Rune-namespaced condition. The `TRANSACTION_DIRECTORY`, `JOURNAL_FILE`, and `LOCK_FILE` constants in Rune's transaction module define the exact paths as `docs/openspec/.rune-transaction/journal.yaml` and `docs/openspec/.rune-archive.lock`. They are runtime state, not shared artifacts, and remain gitignored.

Repository rollout is blocked until the pinned OpenSpec validator and interactive view tolerate both paths, including a persistent interrupted-operation journal. If OpenSpec does not safely ignore them, a separate design moves Rune runtime state outside the shared tree while preserving locking and recovery semantics.

RuneSpec does not reinterpret an OpenSpec artifact into a different meaning. Shared requirement, scenario, task, archive, and configuration structures retain OpenSpec semantics.

Compatibility is demonstrated by running both CLIs against the same files, not by maintaining conversion parity between separate trees.

### Recovery import and export

RuneSpec import and export remain production recovery tools. They can inspect, restore, or compare external layouts, but the normal repository workflow does not invoke them and does not generate a second canonical tree.

No migration command is required for repository rollout. Existing compatible specifications and changes move into `docs/openspec/` without content transformation, then both validators check the moved files.

## API and Interface Design

### Rune configuration

```text
rune config set spec.root docs/openspec
```

No provider key is introduced.

### OpenSpec store identity

A repository tracks store metadata under `docs/.openspec-store/store.yaml`. OpenSpec v1.7.0 defines version `1`, a required kebab-case `id`, and an optional non-empty `remote` field.[OPENSTORESRC]

```yaml
version: 1
id: example-project
```

The real ID derives from the repository's public slug and remains stable across machines. It must not collide with another registered store path.

### OpenSpec registration

Repository setup registers the existing nested store:

```sh
openspec store register ./docs --id <repository-store-id> --yes
```

The setup path uses `store register`, not `store setup`. OpenSpec v1.7.0 exposes `--id` and `--yes`; `--yes` authorizes identity metadata creation for a healthy existing root.[OPENSTORECMD] Registration records the canonical path in OpenSpec's machine-local registry. Ordinary RuneSpec and OpenSpec commands do not modify another registration automatically.[OPENSTORE]

### Direct use

From the repository root:

```sh
rune spec list
rune spec show add-search
rune spec validate add-search
```

From the OpenSpec store root:

```sh
cd docs
openspec list
openspec view
openspec validate --all --no-interactive
```

Store-aware commands can run from elsewhere:

```sh
openspec list --store <repository-store-id>
openspec doctor --store <repository-store-id>
openspec view --store <repository-store-id>
```

## Data Flow

### RuneSpec

1. Rune resolves the repository source.
2. Rune reads `spec.root` from merged configuration.
3. Rune canonicalizes and confines `docs/openspec` to the repository.
4. Rune executes its native command against that tree.
5. Rune preserves its current rendering, JSON, exit status, transaction, and recovery behavior.

### OpenSpec

1. OpenSpec resolves the registered store, explicit `--store`, or nearest `openspec/` tree according to its own rules.[OPENSTORE]
2. OpenSpec reads or writes `docs/openspec/` directly.
3. OpenSpec returns its own output, exit status, interactive behavior, and diagnostics.

Neither CLI invokes the other.

### Repository rollout

1. Move existing lifecycle directories into `docs/openspec/` without rewriting artifact content.
2. Add `docs/openspec/config.yaml` and `docs/.openspec-store/store.yaml`.
3. Set `spec.root: docs/openspec` in repository configuration.
4. Run RuneSpec validation against the moved tree.
5. Run the pinned OpenSpec version's non-interactive full validation against the same tree.
6. Register `<repository>/docs` as the local OpenSpec store.
7. Exercise both command interfaces, including the real interactive OpenSpec view.

Repository rollout begins only after the git ceremonies design is complete. Each repository moves through its own staged review and PR process.

## Edge Cases

### OpenSpec is unavailable

RuneSpec remains fully available because it has no runtime dependency on the OpenSpec executable. OpenSpec-only capabilities are unavailable until OpenSpec is installed. Rune does not print fallback or provider messages because no provider relationship exists.

### Custom RuneSpec roots

`docs/openspec` is the runedeck convention, not a RuneSpec restriction. RuneSpec retains arbitrary repository-confined `spec.root` values, autodetection, journal-only root recognition, and transaction-journal tie-breaking.

A custom root that is not shaped as `<store>/openspec` does not claim direct OpenSpec-store compatibility. The user can continue using RuneSpec there without a mirror or automatic conversion.

### OpenSpec version drift

Compatibility tests pin a supported OpenSpec release. OpenSpec v1.7.0 is the first target for this design.[OPENREL]

A later OpenSpec release does not change RuneSpec behavior. Compatibility advances only after the shared fixtures pass both implementations and any format differences are reviewed.

### Store registration drift

A missing machine-local registration does not invalidate the repository. Running OpenSpec from `docs/` still discovers the local `openspec/` tree. Registration is required only for store-ID selection from other locations.

A store ID registered to another canonical path is an error. Setup reports the collision and does not rewrite or remove the existing registration.

### Path safety

RuneSpec continues to reject absolute roots, parent traversal, and symlink escapes through canonical path confinement. OpenSpec receives the real filesystem layout directly; Rune does not construct or interpolate OpenSpec commands.

### Artifact extensions

A proposed Rune-specific artifact extension cannot enter the common tree until a compatibility fixture proves that the pinned OpenSpec version preserves or safely ignores it. An incompatible extension requires a Rune-namespaced path and an explicit statement that the artifact is outside the OpenSpec common denominator.

## Test Strategy

### Shared artifact fixtures

- Run RuneSpec and the pinned OpenSpec CLI against the same canonical specifications, active changes, archived changes, tasks, metadata, and nested capabilities.
- Cover every delta operation, fenced examples, task progress, nested capability paths, and UTF-8 BOM input supported by the compatibility baseline.[OPENREL]
- Alternate mutations between the two CLIs and verify that the other can read and validate the result without conversion.
- Add an interrupted-operation fixture containing `.rune-transaction/journal.yaml` and `.rune-archive.lock`; verify OpenSpec validation and interactive view safely ignore the runtime state.

### RuneSpec regression

- Run every current RuneSpec command against `docs/openspec/`.
- Keep all golden human and JSON fixtures unchanged unless a separate behavior decision approves a change.
- Keep the compile-time public API test unchanged.
- Run transaction failure-injection, interrupted-operation recovery, locking, confinement, and read-only doctor tests unchanged.
- Verify flexible custom roots, autodetection, journal-only roots, and transaction-journal tie-breaking.

### OpenSpec acceptance

- Run OpenSpec list, show, validate, archive, status, instructions, templates, schemas, doctor, context, and interactive view against the same store.
- Verify store selection from `docs/`, by registered ID where supported, and through OpenSpec's own root-resolution rules.
- Exercise the real interactive dashboard manually before repository rollout.

### Repository safety

- Use an isolated OpenSpec data directory in automated tests so the user's real store registry is never modified.
- Verify that runtime state and interrupted-operation files remain untracked.
- Verify that repository moves preserve file bytes before validation.

## Out of Scope

- Adding a Rune `spec.provider` setting.
- Dispatching OpenSpec commands through Rune.
- Reimplementing OpenSpec's interactive view, stores, worksets, schemas, completion, or instructions as part of compatibility rollout.
- Deleting or reducing any existing RuneSpec capability or public API.
- Maintaining a generated mirror beside `docs/openspec/`.
- Requiring RuneSpec and OpenSpec command names, flags, output, or internal behavior to match.
- Automatically installing or upgrading OpenSpec.
- Opening repository PRs before the git ceremonies design is complete.

## Sources

[OPENCLI]: https://openspec.dev/docs/reference/cli "OpenSpec CLI reference"
[OPENSTORE]: https://openspec.dev/docs/stores "OpenSpec stores"
[OPENSTORECMD]: https://github.com/Fission-AI/OpenSpec/blob/v1.7.0/src/commands/store.ts "OpenSpec v1.7.0 store command"
[OPENSTORESRC]: https://github.com/Fission-AI/OpenSpec/blob/v1.7.0/src/core/store/foundation.ts "OpenSpec v1.7.0 store metadata"
[OPENREL]: https://github.com/Fission-AI/OpenSpec/releases/tag/v1.7.0 "OpenSpec v1.7.0"
