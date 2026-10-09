# Acceptance

Both README versions name `pennix-workflow-lifecycle` as the deployment entry
and the exact `PennixRv/pennix-skills` user-template path as the guidance source.
The documented default remains `--guidance none`; Apply/TUI still leave AGENTS
untouched. `git diff --check` passed; the diff includes only documentation and
this task. No runtime code, npm package or binary release changed.

The task is direct on main and has no PR. Native archive uses the explicit
`--skip-branch-validation` exception; the parent records the pushed source commit.
