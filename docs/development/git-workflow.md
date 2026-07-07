# Git Workflow for the Torus Blockchain Codebase

This document defines our Git workflow for the `torus-substrate` repository. It lays out the rationale behind how we commit, structure, and merge changes into the `main` /`dev` branch. In a blockchain context, where code governs real economic value, we must prioritize traceability, auditability, and long-term maintainability over short-term convenience.

We compare common merge strategies and explain our preference for using **merge commits with structured commit hygiene**, rather than squash merges or rebasing workflows. This isn't just a stylistic choice: it's a deliberate effort to ensure clarity around **who changed what, when, and why**, especially during incidents, audits, or upgrades.

The principles and workflow below serve as guidance for all contributors to ensure consistency, trustworthiness, and clarity in the project's evolution.

---

## Principles

We value a commit history that is:

- **Traceable**: Every line of code should be attributable to a specific intent, person, and context.
- **Auditable**: Changes to sensitive components (consensus, balances, permissions, etc.) must be easily inspectable.
- **Sequentially clear**: It should be obvious what was introduced, when, and in what order.
- **Low-friction**: The workflow should be predictable and intuitive for contributors, without requiring excessive Git gymnastics.

These principles are especially important in a blockchain setting where code correctness is not just a matter of engineering discipline, but of trust and economic safety.

---

## Merge Strategy Trade-offs

There are three common merge strategies:

### 1. Squash Merge

- **Pros**:
  - Keeps the `main` branch visually clean.
  - Easy to revert a PR.
  - Useful for one-off contributions or noisy, single-developer branches.
- **Cons**:
  - Destroys commit granularity and authorship detail.
  - Makes it hard to know what happened inside a PR.
  - Comments on intermediate commits are lost.
  - Local branches don't show up as merged—causing confusion and cleanup issues.

### 2. Merge Commit (Recommended)

- **Pros**:
  - Preserves full commit history and branch ancestry.
  - Enables clear visibility into the evolution of a feature or fix.
  - Works well with tools like `git bisect`, `git blame`, and code audits.
  - Facilitates safe deletion of merged branches.
- **Cons**:
  - History is visually noisier.
  - Requires discipline to maintain clean, structured commits.

### 3. Rebase and Merge

- **Pros**:
  - Produces a linear history.
  - Preserves commit intent when structured properly.
- **Cons**:
  - Rewrites history (new SHAs), which can create confusion.
  - Comments and context can get lost.
  - Not ideal for branches with multiple collaborators.

---

## Recommended Workflow

### For All PRs

- Use **Merge Commit** via GitHub's "Create a merge commit" option.
- Before opening a PR, clean up your commits using:

  ```sh
  git rebase -i HEAD~N
  ```

- Ensure each commit is atomic, with a clear, descriptive message.
- If your branch is messy or trivial, squash manually *before* merging.

### Branch Cleanup

You can safely delete local branches that have been merged with:

```sh
git branch --merged main | grep -v '\*' | xargs -n 1 git branch -d
```

### Special Cases

- For external contributors: **Squash merge** is acceptable.
- For collaborative PRs or those touching critical systems: preserve full history via merge commits.
- Avoid GitHub's "Rebase and merge" to maintain clarity and ancestry in history.

---

## Deployment and Release Traceability

- Every commit in `main` /`dev` represents a concrete step in system evolution.
- Releases and deploys should reference merge commits to clarify what was included.
- PRs should link to Linear issues or changelogs where applicable.

---

## TODO: Commit Message Guidelines

We should define a small set of conventions for commit messages to make history more searchable and consistent. Suggestions:

- Use a prefix for the type of change: `feat:`, `fix:`, `refactor:`, `chore:`, `test:`
- Start with a concise imperative verb.
- Follow up with a short summary of intent, not implementation.
- Optionally, include a body explaining context or rationale.
- Consider linking Linear task IDs when appropriate.

Example:

```txt
feat(runtime): add staking rewards distribution logic

Implements the first version of reward logic based on active stake and session intervals.
Related to LIN-42.
```

We may automate linting or enforce this with a pre-push hook in the future.

---

> Our Git history is our ledger of intent. Write it for future teammates, auditors, and users who trust us with code that governs money.
