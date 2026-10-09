# Clarify FastCtx deployment and guidance sources

## Goal

Implement the README clarification included in the approved root task `consumer-update-agents-recheck` revision 4. Name the actual deployment Skill and the repository that maintains the user instruction template.

## Requirements

- Update the English and Chinese README installation/source paragraphs together.
- Identify `pennix-workflow-lifecycle` as the deployment entry and `PennixRv/pennix-skills` as the template source.
- Preserve the normal `--guidance none` behavior and existing release/distribution contracts.
- This is a direct documentation change; no runtime changes or binary release are required.

## Acceptance Criteria

- [ ] Both README versions identify the same concrete deployment and template sources.
- [ ] The diff changes documentation and this task record only; `git diff --check` passes.
- [ ] Commit and push the change to the existing `main` branch, then archive this non-PR task through the native exception.

## Notes

- The user explicitly approved the complete root/component revision package and its listed cross-repository documentation changes.
- Root evidence remains in the root task; this repository owns the README changes.
