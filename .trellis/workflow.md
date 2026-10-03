# Codex Subnode Channel Workflow

---

## Core Contract

1. **Plan before delivery** — define the task, acceptance criteria, and necessary evidence before changing code.
2. **The main session delivers by default** — it owns implementation, task facts, final verification, and Git.
3. **A `subnode` is explicit independent evidence** — use one only when the user expressly needs independently reviewable analysis, design, audit, review, counterargument, or verification. It is never the automatic implementation or checking path.
4. **Artifacts are durable** — the coordinator creates an immutable `brief.json`; the subnode appends its `worklog.md` and writes one pending-review `report.json` under the active task.
5. **Completion is not acceptance** — the coordinator validates the report, rechecks material sources and protected targets, then records `accepted`, `rejected`, or `deferred` with its reason.
6. **Channel is the lifecycle surface** — use its native create, spawn, send, and wait protocol. Wait for events rather than high-frequency polling; do not create a second waiter, use terminal JSON as the report, or add automatic retry/scheduling.
7. **Evidence is unit-sized** — split read-heavy main-session or subnode work into independently useful evidence units when one bounded session cannot persist a conclusion. Each unit records its scope, minimal evidence, destination, stop condition, conclusion or blocker, unknowns, and recovery point before the next unit.

### Subnode Evidence Profiles

The Trellis project template provides an editable
`.trellis/agents/subnode-profiles.json`. It is the single project-local source
for non-implementation evidence profiles. Keep the same `subnode` identity,
brief `lens`, report v2, and coordinator disposition for every profile; a
profile changes dispatch configuration, not authority or acceptance semantics.

The shipped `default_model` is `gpt-6.1-sol`. `docs_source` overrides it with
`gpt-5.6-luna`. These eight default profile IDs select bounded evidence work:

| Profile | Default effort | Typical evidence unit |
|---|---:|---|
| `docs_source` | `xhigh` | extraction from identified official sources |
| `fault_diagnosis` | `high` | root-cause and failure reconstruction |
| `correctness_test` | `high` | contract and regression review |
| `security_permission` | `high` | trust boundary and permission review |
| `architecture_compat` | `high` | cross-layer compatibility and migration review |
| `requirements_assumption` | `high` | requirements, ambiguity, and assumption audit |
| `ux_accessibility` | `high` | interaction and accessibility review |
| `evidence_synthesis` | `medium` | bounded cross-source synthesis |

Ordinary code-location and call-path lookup stays in the main session;
`code_path` is not a shipped preset. When independent call-chain evidence is
needed, select `correctness_test`, `fault_diagnosis`, or `architecture_compat`
with that specific brief lens.

Use `docs_source` for extraction from known sources. A version conflict,
interface-contract judgment, or disputed source requires an explicit
`gpt-6.1-sol` / `high` override. Keep judgment profiles on Sol; increase to
`xhigh` when conflicting evidence, cross-layer reasoning, or material risk
justifies it. Luna may organize already-verified facts through an explicit
override; model selection does not transfer coordinator acceptance authority.
Do not introduce automatic model fallback, effort escalation, or a default
`max` effort; `max` is outside the current Trellis effort contract.

Projects may add or remove profile IDs while preserving the required
`reasoning_effort` values (`medium`, `high`, or `xhigh`). A profile may provide
its own model; otherwise it uses `default_model`. A single dispatch may
explicitly override model and effort. Resolution is deterministic:
single-dispatch override, profile model, profile default model, then the
agent's legacy model when no profile was selected. Explicit effort overrides
profile effort. Unknown, missing, malformed, symlinked, or non-Codex profile
configuration is a dispatch error; never silently fall back.

`xhigh` requires a concrete non-empty `--reasoning-effort-reason` tied to the
evidence unit. This also applies to Luna: the reason may explain that `xhigh`
is its lowest selected effort for a bounded source-extraction unit, without
claiming that ordinary extraction is complex. The reason is recorded with
the resolved values. The worker
still uses the shared Codex configuration window; this workflow does not
promise a temporary main-session model or context override and does not claim
provider-side effective effort when the adapter cannot observe it.

Before a worker is spawned, the coordinator records the immutable brief,
selected profile, profile configuration relative path and SHA-256 digest,
resolved model and effort, source of each resolution, and any override reason.
The durable `spawned` event is the machine-readable receipt. A terminal event,
worker message, or report alone never proves that the selected profile was
accepted.

### Persistent FIFO Dispatch Queue

When one task has multiple independent evidence units, the coordinator may use
the task-owned queue helper after the reliability gate has passed:

```bash
python3 ./.trellis/scripts/subnode_artifact.py queue init \
  --task <task-dir> --work-id <work-id> \
  --channel-name <channel> --channel-scope project \
  --brief <brief-1> --brief <brief-2>
python3 ./.trellis/scripts/subnode_artifact.py queue validate \
  --task <task-dir> --work-id <work-id>
```

`queue.json` is written once and contains only the task/work/Channel identity,
creation time, and immutable FIFO brief references with digests. It is not a
worker-state mirror. Immediately before each native `channel spawn`/`send`, the
coordinator writes one `dispatch-claim.json` for that subnode; a claim followed
by a spawn failure is a failed attempt that must be investigated, not silently
retried. A new attempt requires a new brief and subnode ID.

The main session fills available guard capacity in queue order, then uses one
native Channel waiter after a durable barrier. Terminal release of a physical
slot does not itself authorize a replacement. Before every replacement, the
coordinator rechecks every dispatched item: only a native terminal event,
complete report with no validator concerns, matching source and protected
target recheck, and a single written `accepted` disposition authorize the next
FIFO item. `blocked`, `incomplete`, `rejected`, validator concern, capacity
rejection, missing event, conflicting receipt, or an unresolvable reservation
pauses the whole queue.

On interruption or resume, reconstruct state from the original Channel events,
claims, live PID/reservation, reports, and dispositions. An existing claim or
attempt is never re-dispatched merely because its process is gone. The
coordinator continues only with items that have no attempt facts and only when
all earlier items are accepted. The main session must remain present; this is
not a background scheduler. To stop permanently, write one abandonment marker
and partition every queued ID between persisted dispatch claims and the
remaining unclaimed items. A claim counts as an attempted dispatch even if the
native spawn subsequently fails; the helper rejects incomplete, overlapping,
or claim-mismatched ranges:

```bash
python3 ./.trellis/scripts/subnode_artifact.py queue abandon \
  --task <task-dir> --work-id <work-id> --reason <reason> \
  --dispatched <id> --pending <id>
```

An abandoned queue cannot claim further work. Do not delete, reorder, skip, or
edit queue items. A different order requires a new work ID. Waiting uses the
existing durable barrier/`afterSeq` path; high-frequency polling, a second
waiter, automatic retry, and a resident scheduler remain prohibited.

This workflow assumes Codex's default `codex.dispatch_mode: inline`. Do not
select `auto` or `sub-agent` for this workflow: those modes seed native-agent
context and belong to the Native Trellis Workflow instead.

## Trellis System

### Task And Spec Records

Keep requirements, research, plans, review conclusions, and acceptance evidence
in the current task. Read relevant package and layer guidance before editing:

```bash
python3 ./.trellis/scripts/get_context.py --mode packages
python3 ./.trellis/scripts/task.py current --source
```

For clearly bounded, single-surface work with an immediate verification path,
proceed directly in the main session without creating a task or subnode. Apply
normal safety rules. Create a task before continuing when scope expands or the
work needs design, ownership coordination, release, credentials, or a durable
record. A task may be PRD-only when lightweight; complex work also needs
`design.md` and `implement.md` before it starts.

When research, audit, or review establishes task-relevant verified facts,
candidates, uncertainty, or a decision basis that should persist, use
`trellis-research-record` to write the conclusion with its evidence in the
active task. Do not create a record for routine navigation or transient output.

For ordinary main-session research, keep each evidence unit small enough for one
normal context window without inventing an exact token or time limit. Create a
child task only when the unit has an independent owner, lifecycle, and acceptance
contract. A subnode is an explicit independent-evidence choice, not a way to
offload every long read.

### Semantic RecoveryBrief

Trellis task artifacts remain the semantic record. The low-level
`ctx_recovery_brief` provider is an optional task-local evidence projection for
an already-started task, a material recorded change, explicit inspection or
repair, or a user-requested pause/finish. Do not create one for ordinary edits,
tests, diffs, compaction, resume, or a subnode event/report. It is never a
formal-handoff gate, a source-session exit signal, or a replacement for the
Pennix session-handoff Semantic Handoff Capsule and its explicit Cognee
write and exact dataset/data/content retrieval-verification step (including the raw content read). For a formal
handoff, the `cognee_required` path's authenticated dataset/data UUID proof and
handoff receipt record source convergence; local task artifacts remain
authoritative. A taskless handoff seals its session boundary without creating
a task; a task-bearing handoff retains native ownership barriers. The explicit
`core_only` mode is the offline exception. A RecoveryBrief may be included as
corroborating task evidence only.

### Independent Evidence

For an explicit independent-evidence request, first load `trellis-channel` and
follow its `subnode-work` reference. It defines the brief schema, safe artifact
initialization, role prompt, native wait protocol, report validation, and
counterwork. The coordinator may create a bounded subnode while the task is
`planning` or `in_progress`.

The subnode may inspect the necessary current project, external sources, and
internet evidence. It may write only its assigned `worklog.md` and
`report.json`; it must not change protected targets, task facts, Git state, or
worker lifecycle. The final Channel message is a short status and report path,
not the report payload. Reports use schema version 2 with one scope assessment
per assigned scope, evidence-linked structured findings, typed notes, and
worklog checkpoints; validator concerns remain coordinator follow-up, never
acceptance.

Use `workspace_note.py` only for a durable cross-task observation, decision,
open question, or blocker. It is separate from task and subnode evidence.

---

## Phase Index

```text
Phase 1: Plan    -> classify, then create a task when requirements or evidence need one
Phase 2: Execute -> main-session delivery; optional explicit subnode evidence
Phase 3: Finish  -> verify, retain conclusions, update specs, commit, and wrap up
```

### Request Triage

- Direct small work: for a clearly bounded, single-surface operation with an immediate verification path, proceed in the main session without a Trellis task or subnode. Apply normal safety rules. If discovery expands the scope or reveals a design, ownership, release, or durable-record need, stop and create a task before continuing.
- `analysis_only` is eligible only when `task.json.meta.delivery_mode = "analysis_only"` exactly and the PRD defines a bounded evidence deliverable plus a no-change boundary for product source, runtime configuration, deployment, credentials, and external systems. Complex research, cross-owner coordination, design, release, credential, or material-decision work is not eligible for this route.
- An eligible `analysis_only` task stays in `planning`: complete and verify its evidence, commit task artifacts, and archive directly. Do not start implementation. A protected-target recommendation requires a separate change-bearing task.
- Complex work: create a task after approval, then complete planning before implementation.
- Task-creation approval is not implementation approval.
- Do not create a subnode merely because work is broad or inconvenient. State the independent question, reason, evidence method, scope, and stop condition first; use the main session when independent evidence adds no decision value.

### Planning Artifacts

- `prd.md` — requirements, constraints, and acceptance criteria.
- `design.md` — boundaries, data flow, contracts, and tradeoffs for complex work.
- `implement.md` — ordered execution, validation, and rollback plan for complex work.
- `research/` — attributable external or local findings.
- `subnodes/<work-id>/<subnode-id>/` — a coordinator brief, append-only node worklog, and pending-review report when independent evidence was expressly requested.

[workflow-state:no_task]
No active task. Classify the request. Direct small work with a clear single owner and immediate verification may proceed in the main session without a Trellis task or subnode under normal safety rules. If scope expands or requires design, ownership, release, credentials, or a durable record, create a task before continuing; do not promote an unrecorded conclusion as task fact.
[/workflow-state:no_task]

[workflow-state:task_error]
The active task record cannot be read. Do not create or activate another task. Repair the current task record while preserving existing artifacts, or ask the user when its status cannot be determined safely.
[/workflow-state:task_error]

[workflow-state:unbound_task]
An existing developer-owned task has no direct session binding. Inspect native `task.py current --json`: a non-null `session_source` means direct identity is already available; an unbound source still means no task binding. Select the existing task from the user's explicit intent; do not guess or create a duplicate. Resume its planning/status gates first. Run native `task.py start <task>` only when its activation contract permits it, after the Planning Seal and implementation authorization for a planning change-bearing task. An eligible `analysis_only` task stays in planning without start. If identity is absent, report that separately; never invent an identity or edit runtime pointers.
[/workflow-state:unbound_task]

[workflow-state:unbound_ambiguous]
Multiple developer-owned tasks have no direct session binding. Inspect native `task.py current --json`: a non-null `session_source` means direct identity is already available; an unbound source still means no task binding. Select the existing task from the user's explicit intent; do not guess or create a duplicate. Resume its planning/status gates first. Run native `task.py start <task>` only when its activation contract permits it, after the Planning Seal and implementation authorization for a planning change-bearing task. An eligible `analysis_only` task stays in planning without start. If identity is absent, report that separately; never invent an identity or edit runtime pointers.
[/workflow-state:unbound_ambiguous]

[workflow-state:unbound_ambiguous-inline]
Multiple developer-owned tasks have no direct session binding. Inspect native `task.py current --json`: a non-null `session_source` means direct identity is already available; an unbound source still means no task binding. Select the existing task from the user's explicit intent; do not guess or create a duplicate. Resume its planning/status gates first. Run native `task.py start <task>` only when its activation contract permits it, after the Planning Seal and implementation authorization for a planning change-bearing task. An eligible `analysis_only` task stays in planning without start. If identity is absent, report that separately; never invent an identity or edit runtime pointers.
[/workflow-state:unbound_ambiguous-inline]

[workflow-state:planning]
Only if `task.json.meta.delivery_mode = "analysis_only"` exactly and its PRD satisfies the bounded evidence-only eligibility rule, stay in planning: complete and verify the bounded evidence, preserve the protected-target no-change boundary, commit task artifacts, and archive without running `task.py start`.
For a change-bearing task, stay in planning until the required artifacts are complete, the Planning Seal is closed, and the user approves implementation. Then the main session runs the native `python3 ./.trellis/scripts/task.py start <task-dir>` to enter `in_progress`; do not implement while status is `planning`. The main session performs ordinary work directly. A subnode is allowed only for a user-requested independent-evidence question with a frozen brief and durable task artifact path; report status is never acceptance.
[/workflow-state:planning]

[workflow-state:planning-inline]
Only if `task.json.meta.delivery_mode = "analysis_only"` exactly and its PRD satisfies the bounded evidence-only eligibility rule, stay in planning: complete and verify the bounded evidence, preserve the protected-target no-change boundary, commit task artifacts, and archive without running `task.py start`.
For a change-bearing task, stay in planning until the required artifacts are complete, the Planning Seal is closed, and the user approves implementation. Then the main session runs the native `python3 ./.trellis/scripts/task.py start <task-dir>` to enter `in_progress`; do not implement while status is `planning`. The main session performs ordinary work directly. A subnode is allowed only for a user-requested independent-evidence question with a frozen brief and durable task artifact path; report status is never acceptance.
[/workflow-state:planning-inline]

[workflow-state:in_progress]
Deliver and verify in the main session. Before code changes, load `trellis-before-dev`; after changes, use `trellis-check` and the task acceptance criteria. If implementation exposes a material unresolved decision, record `decision-needed`, run `task.py replan`, and return to planning. Invoke a Channel subnode only for explicit independent evidence, then validate its report and recheck sources before recording a disposition. The main session alone commits and finishes.
[/workflow-state:in_progress]

[workflow-state:in_progress-inline]
Deliver and verify in the main session. Before code changes, load `trellis-before-dev`; after changes, use `trellis-check` and the task acceptance criteria. If implementation exposes a material unresolved decision, record `decision-needed`, run `task.py replan`, and return to planning. Invoke a Channel subnode only for explicit independent evidence, then validate its report and recheck sources before recording a disposition. The main session alone commits and finishes.
[/workflow-state:in_progress-inline]

[workflow-state:completed]
The task is complete only after required verification, decision records, and Git work are complete. Use `trellis-finish-work` to wrap up; return to the appropriate phase if the working tree or acceptance evidence is incomplete.
[/workflow-state:completed]

---

## Phase 1: Plan

#### 1.0 Create Task `[required · once]`

After user approval, create the task and record its scope, actual modification
target, constraints, and acceptance criteria. Do not treat task creation as
permission to implement.

#### 1.1 Requirements And Design `[required · repeatable]`

Write `prd.md`; add `design.md` and `implement.md` when the task is complex.
Resolve only unknowns that can change scope, risk, modification path, or
acceptance. Keep alternatives as alternatives until the user selects one or
evidence justifies a decision. After each answer, persist the decision,
recalculate the planning frontier, and continue the same planning loop. Do not
close the turn merely because a question was answered.

Before the final planning summary, run one Planning Seal closure pass. Reconcile
task metadata, PRD, design, implementation plan, research, decision records, and
manifests; lock actual targets and branches, dependencies, release, validation,
rollback, dynamic-fact dispositions, replan triggers, and every material decision
to an owner and outcome. No static `TBD`, `TODO`, `decision-needed`, unowned
option, unspecified branch, open implementation path, validation gap, or
conditional acceptance may remain. A material discovery returns the task to
planning and invalidates the seal.

#### 1.2 Research `[optional · repeatable]`

Perform ordinary local and external research in the main session and write
material findings to `research/`. When the user expressly needs independent
evidence, define one narrow `subnode` brief and use the installed
`trellis-channel` reference. Preserve both the report and the coordinator's
later decision; do not substitute a Channel message for either record.

#### 1.3 Prepare Execution `[required · once]`

Confirm the direct implementation and verification path. This workflow uses
inline Codex execution: do not curate native-agent JSONL or dispatch native
implementation/check workers. Plan any optional subnode as evidence work with
protected targets, an independence reason, evidence method, stop conditions,
and a deadline.

#### 1.4 Activate Task `[required · once]`

For change-bearing tasks only, review the planning artifacts. A lightweight
task needs a complete PRD; a complex task also needs its design and
implementation plan. Once the Planning Seal is closed and the user approves
implementation, run native `python3 ./.trellis/scripts/task.py start <task-dir>`;
implementation begins only after status becomes `in_progress`. An eligible
`analysis_only` task completes its bounded evidence and archives from planning
without start.

#### 1.5 Completion Criteria

Before Phase 2, ensure that the modification target, acceptance criteria,
validation commands, and known risks are explicit. For a planned subnode,
ensure the question is independent and bounded rather than a proxy for normal
implementation or review. The planning seal must close every static
implementation choice: no `TBD`, `TODO`, `decision-needed`, unowned option,
unspecified branch, open implementation path, validation gap, or conditional
acceptance point may remain.

---

## Phase 2: Execute

#### 2.1 Implement `[required · repeatable]`

The main session loads `trellis-before-dev`, reads the reviewed task artifacts,
implements the approved scope, and records relevant findings. Do not expand the
change for hypothetical future needs.

When a user explicitly requests independent evidence, the coordinator creates
the artifact brief before spawning `subnode` through Trellis Channel. The node
does not implement business changes. After its native terminal event, validate
the report and independently recheck enough evidence to decide its disposition.

#### 2.2 Quality Check `[required · repeatable]`

The main session runs the task's relevant checks and `trellis-check`, then
addresses verified issues. An optional counter-subnode is appropriate only when
the evidence is consequential and a distinct lens can materially challenge the
primary report; it is never automatic.

#### 2.3 Rollback `[on demand]`

If verification or evidence disproves the selected approach, record the reason,
restore the smallest correct state, and return to the relevant planning or
implementation step. For a material unresolved decision during implementation,
record `decision-needed`, run `python3 ./.trellis/scripts/task.py replan
<task> "<reason>"`, and return to planning. Do not open an ad-hoc popup or
silently choose. Do not hide a retracted conclusion by rewriting a subnode
worklog or report history.

---

## Phase 3: Finish

#### 3.2 Debug Retrospective `[on demand]`

For repeated failures, use `trellis-break-loop` to identify the root cause and
the missing prevention rule. Keep the retrospective proportional to the actual
failure mode.

#### 3.3 Update Spec `[required · once]`

Use `trellis-update-spec` only when the task established a reusable contract,
convention, or prevention rule. Keep task-specific evidence in the task rather
than inflating permanent specifications.

#### 3.4 Commit Changes `[required · once]`

After required checks pass, review the diff, retain task and decision records,
and create the scoped commit directly. The main session owns all Git
operations; this workflow does not insert a second `Proposed commits`/`ok`
confirmation step between the accepted task plan and the commit. Push, remote
release, and destructive operations still require their normal explicit
authorization and owner protocol.

#### 3.5 Wrap Up `[required · once]`

Use `trellis-finish-work` to report checks, remaining risks, and task state.
Do not claim complete until the evidence and verification appropriate to the
task are actually recorded.
