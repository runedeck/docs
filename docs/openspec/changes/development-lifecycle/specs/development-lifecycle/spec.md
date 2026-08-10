## Purpose

Defines the portable, review-driven development lifecycle that carries a Rune change from specification through decisions, implementation, testing, review, and deployment.

## ADDED Requirements

### Requirement: Changes advance through reviewed lifecycle phases

Rune Deck MUST use this lifecycle by default for substantive changes. Trivial mechanical changes that do not alter behavior, architecture, delivery, or shared process are outside this lifecycle. A participating change MUST advance through specification, tailored specification review, decision declaration, implementation planning, implementation and testing, pull-request review, and deployment. A later phase MUST NOT be accepted or authorize dependent work until required artifacts validate and the user explicitly approves the preceding consequential transition. Decision drafts MAY be explored during specification review but remain provisional until the specification is accepted.

#### Scenario: Decision drafting starts during specification review

- **WHEN** decision drafting starts before the tailored specification review is accepted
- **THEN** the drafts remain provisional and are reconciled with every specification revision before decision acceptance

#### Scenario: Required phase is incomplete

- **WHEN** a required artifact is missing, invalid, or awaiting user approval
- **THEN** the lifecycle identifies what is missing, proposes the native next action, asks before starting it, and does not advance to a dependent phase

#### Scenario: Change is trivial and mechanical

- **WHEN** a change alters no behavior, architecture, delivery, or shared process
- **THEN** DevelopmentLifecycle does not start for that change

### Requirement: Approval questions show the reviewed material

Before asking for approval, the lifecycle MUST present the exact artifact, diff, plan, pull-request text, or outward-facing action under review through a complete readable display. A preview is sufficient only when it shows the complete reviewed content. If the preview truncates or hides lines, the lifecycle MUST open the full artifact in the configured editor or render it completely before asking. A path or summary alone MUST NOT be treated as sufficient approval context.

#### Scenario: Artifact acceptance is requested

- **WHEN** the lifecycle asks the user to accept a specification, ADR, plan, staged diff, or pull-request text
- **THEN** the question displays the reviewed content and identifies its source path

#### Scenario: Preview hides reviewed content

- **WHEN** the harness preview truncates the artifact or hides lines
- **THEN** the lifecycle opens or renders the complete artifact before asking for acceptance

#### Scenario: Outward-facing action is requested

- **WHEN** the lifecycle asks to publish, merge, release, install, or deploy
- **THEN** the question displays the exact action, destination, and reviewed material that will become public or change external state

### Requirement: Skill execution is deterministic from live context

The canonical `DevelopmentLifecycle` skill body MUST contain ordered dynamic context injection (`!`) probes using read-only, non-interactive, secret-free commands. The commands MUST use standard project and platform tools and MUST NOT invoke the Rune CLI. A supporting harness executes the probes before skill execution. A harness that ignores inline injection MUST gather equivalent context during orientation before selecting a phase.

The skill MUST define a closed phase set with explicit entry, completion, transition, and invalidation predicates. The same artifacts and probe output MUST produce the same current phase and proposed next action. Every selection MUST report the matched predicate, supporting evidence, and proposed transition. Each probe MUST distinguish a valid absent state from execution failure through stable output. A failed required probe MUST stop orientation and identify the missing context instead of allowing inferred state.

The skill MUST remain usable when shared outside Rune or the Rune Deck directory layout.

#### Scenario: Two sessions observe the same state

- **WHEN** two supported harnesses receive equivalent artifacts and probe output
- **THEN** both select the same lifecycle phase and report the same matched predicate, evidence, and proposed transition

#### Scenario: Harness ignores inline injection

- **WHEN** a harness does not execute the skill body's `!` probes before loading the instructions
- **THEN** DevelopmentLifecycle gathers equivalent context during orientation before evaluating phase predicates

#### Scenario: Optional context is absent

- **WHEN** a probe succeeds with a stable absent-state value such as no active pull request
- **THEN** the skill evaluates phase predicates using that absence as valid context

#### Scenario: Required probe fails

- **WHEN** a required probe cannot execute or returns unusable output
- **THEN** the skill identifies that probe and does not guess the lifecycle phase

### Requirement: Specification review uses tailored questions

After strict specification validation, the lifecycle MUST review the stated goals, non-goals, and each requirement scenario with questions tailored to their authored content. It MUST use the harness's structured question function when available. A harness without that function MUST ask the same bounded question in plain text and wait for the answer.

The interview MUST run in a context that can communicate directly with the user. Custom answers are clarification until they resolve to acceptance or a concrete revision.

#### Scenario: Harness supports structured questions

- **WHEN** the review runs in a harness with a structured question function
- **THEN** goals, non-goals, and scenarios are presented through that function with content-specific choices and custom clarification

#### Scenario: Harness lacks structured questions

- **WHEN** the review runs in a harness without a structured question function
- **THEN** it presents the tailored question in plain text and pauses until the user resolves it

#### Scenario: Review reaches its normal boundary

- **WHEN** eight questions have been asked and unresolved scenarios remain
- **THEN** the lifecycle asks whether to continue individually, review the remainder as a group, or stop for manual editing

### Requirement: Review revisions are applied and revalidated

When an answer changes a goal, non-goal, or scenario, the lifecycle MUST edit the specification immediately, rerun strict validation, and re-present the affected material. It MUST NOT treat the earlier wording as accepted after revision. When validation can be restored without changing the user's intent, the lifecycle MUST apply the smallest structural repair before re-presenting the material.

#### Scenario: User adjusts a scenario

- **WHEN** the user changes a scenario trigger, expected result, or boundary
- **THEN** the lifecycle updates the specification, validates it, and asks for acceptance of the revised scenario

#### Scenario: Revision breaks validation

- **WHEN** an interview revision produces an invalid specification
- **THEN** the lifecycle reports the finding, applies the smallest repair that preserves the user's intent, reruns validation, and keeps review active until the repaired material is accepted

### Requirement: Decisions and implementation require approval

Decision artifacts MUST begin from the accepted specification. When a decision draft introduces newer intent, the lifecycle MUST reopen and revise the specification to match before either artifact is accepted. After decision acceptance, implementation planning MUST use the harness's planning mode or corresponding planning function when available. Implementation MUST begin only after the plan validates and the user approves its approach. Plan approval MUST NOT authorize later consequential decisions that were not resolved in the approved plan.

#### Scenario: Decision introduces newer intent

- **WHEN** a decision draft conflicts with an accepted requirement or scenario
- **THEN** the lifecycle reopens the specification, applies the decision intent, reruns strict validation and tailored review, and keeps the decision provisional until both artifacts agree

#### Scenario: Decisions are accepted

- **WHEN** decision artifacts agree with the accepted specification and receive user approval
- **THEN** the lifecycle enters the harness's planning mode or corresponding planning function to create the implementation plan

#### Scenario: Implementation plan is accepted

- **WHEN** the implementation plan validates and the user approves its approach
- **THEN** the lifecycle exits planning mode and permits implementation while continuing to ask about consequential decisions not resolved by the plan

### Requirement: Testing and review provide completion evidence

Implementation MUST produce test or verification evidence appropriate to the changed behavior. Changes MUST be staged for user review before commit. Staged-change approval MUST NOT authorize pull-request creation; publication through a pull request requires a separate user decision and MUST retain its required review and continuous-integration checks.

#### Scenario: Verification fails

- **WHEN** required tests, validation, or continuous-integration checks fail
- **THEN** the lifecycle diagnoses and fixes failures within the approved approach, reruns verification, and asks before any fix that changes scope or architecture

#### Scenario: Change is ready for pull-request review

- **WHEN** implementation and verification pass and the user approves the staged changes
- **THEN** the lifecycle prepares the pull-request title and body, then asks separately before creating the pull request

### Requirement: Deployment publishes the reviewed result

Deployment MUST place the reviewed change at its intended published destination and MUST require explicit approval when it performs an outward-facing or hard-to-reverse action. When merge is the deployment action, one explicit merge approval authorizes both. For documentation, deployment is complete when the reviewed files are merged and visible on GitHub.

#### Scenario: Documentation change is deployed

- **WHEN** review and checks pass and the user explicitly approves merging a documentation pull request
- **THEN** the lifecycle merges it, verifies the files are visible in the target GitHub repository, and records deployment as complete without a duplicate deployment question

#### Scenario: Deployment awaits approval

- **WHEN** publication, merge, release, or installation requires user approval
- **THEN** the lifecycle pauses before the action and does not infer approval from an earlier phase

### Requirement: Lifecycle work resumes from durable state

The lifecycle MUST derive progress from validated artifacts, recorded approvals, version-control state, pull-request state, and deployment evidence. It MUST trust a completed checkpoint only when its artifacts remain valid and version control shows no later change. When an accepted artifact changed after its checkpoint, the lifecycle MUST clear that checkpoint and every dependent checkpoint. Resuming MUST continue from the earliest incomplete or invalid phase rather than recreating accepted work.

#### Scenario: Lifecycle resumes after interruption

- **WHEN** an interrupted session restarts with accepted specification and decision artifacts present
- **THEN** the lifecycle verifies those artifacts and continues from the next incomplete phase

#### Scenario: Accepted input changed

- **WHEN** version control shows that a previously accepted artifact changed after its completed checkpoint
- **THEN** the lifecycle clears that checkpoint and every dependent checkpoint, then returns to review of the changed phase
