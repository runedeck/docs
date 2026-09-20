# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Change every changelog entry to one verb-first line, as `rune docs check` requires.
- Change the spec tree root from `docs/openspec/` to the native `docs/`, rename five two-word ids, and adopt the spec-driven lifecycle decision as `DOCS-0001`.
- Remove the `spec.root` override and OpenSpec's `config.yaml`, and archive `openspec-store-compatibility` as abandoned.

## [0.1.0] - 2026-07-25

### Added

- Add `links`: broken internal links and orphan pages across a repo's `docs/` tree.
- Add `adr`: `<PREFIX>-<NNNN>` decision records under `docs/decisions/`, with list, supersede with cross-links, and index regeneration.
- Add `spec` (feature `lifecycle`): propose, list, show, context, doctor, archive with delta merges, tree validation, and a crash-safe transaction engine with journal recovery.
- Add `interop` (feature `lifecycle`): ownership-preserving converters between the native spec root and OpenSpec's `openspec/` layout.
- Add `support`, `sheet`, and `error`: atomic writes, frontmatter splitting, path confinement, terminal styling, and the crate error type.
- Add spec root autodetection that counts an interrupted transaction journal as a live tree, so a half-moved root still resolves to the unfinished work.
- Add crash-recovery awareness to retried conversions, which report a `recovered` marker instead of failing on an empty tree.
- Add archive journal validation that rejects overlapping canonical destinations.
- Fix export from a root whose manifest recorded no opaque files, which no longer fails while planning removal of the absent mirror directory.
