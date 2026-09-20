Move this repository's spec tree from `docs/openspec/` to the native `docs/` root and adopt the deck's CORE-0019 as `DOCS-0001`.

## Plan

The crate that implements `rune spec` kept its own lifecycle tree behind a `spec.root: docs/openspec` override, against the decision that the native root is `docs/` with no mirror. Move the tree, bring it under the rune 0.5.0 caps, abandon the change that specified the old layout, and adopt the decision through the review ceremony.

## Changes

- Move `docs/openspec/{changes,specs}` to `docs/{changes,specs}`, remove `docs/openspec/config.yaml` and the repository `config.yaml` that only held `spec.root`.
- Rename five two-word ids to three words, split three requirements over 100 words.
- Archive `openspec-store-compatibility` as abandoned.
- Add `module.yaml` and `defaults.yaml`, adopt CORE-0019 as `DOCS-0001` (12 blocks reviewed, 4 adapted).
- Add `docs/changes/native-spec-root-adoption/`.

## Testing

- [x] `rune spec doctor`, `rune spec validate`, `rune docs check`, `rune adopt doctor`, `rune validate`: no error.
- [ ] Both prek stages in an isolated clone.

## Release Notes

- Change the spec tree root from `docs/openspec/` to `docs/` and adopt the spec-driven lifecycle decision as `DOCS-0001`.
