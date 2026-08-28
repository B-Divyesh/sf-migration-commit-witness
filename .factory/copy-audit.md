# Copy audit — polish round 1

Audited 2026-08-28. Counts use whitespace-separated words. Code samples, navigation labels, and table cells are listed separately because they are not sentences.

## Landing-page sentences

| Words | Sentence |
| ---: | --- |
| 18 | For backend teams reviewing migrations, run one on a test database and save signed before, after, and rollback evidence. |
| 10 | See a partial commit and verified rollback; nothing is saved. |
| 7 | The demo reloads after one visit. |
| 6 | Only demo session state is stored. |
| 6 | The CLI uses the MIT License. |
| 8 | A successful migration command does not prove database state. |
| 6 | Compare checks before and after commit. |
| 12 | The sample command returns 0 after creating one of two required tables. |
| 10 | The CLI records the failed check and then tests rollback. |
| 11 | Add one-value queries for the schema and data your release requires. |
| 13 | The CLI runs your configured commands only after you confirm a test target. |
| 10 | Keep matching JSON and Markdown files with the release review. |
| 11 | The browser view reads a recorded artifact from `mcw demo`. |
| 8 | Run the same sample locally in one command. |
| 11 | A test environment label, confirmation flag, and URL-name checks are required. |
| 12 | Rollback runs only when the config and command flag both request it. |
| 10 | Database URLs and signing keys are omitted from witness files. |
| 10 | The config names the exact command your team already reviews. |
| 6 | Build the Rust binary from source. |
| 9 | The demo needs no config, account, or database setup. |
| 13 | The CLI accepts `test`, `ci`, `development`, or `ephemeral`. |
| 5 | It also requires `--confirm-test-database`. |
| 9 | Save migration and rollback results with each release. |
| 5 | Built by Param Factory. |

No landing sentence exceeds 22 words. No sentence contains a banned marketing word.

## Headings, labels, and actions

All headings make sense without their section body. The primary action is “Try it with sample data.” Other actions name their result: “Install the CLI,” “Open the sample result,” “Copy install command,” and “Read the config reference.”

The former metaphor, jargon, ambiguous buttons, paid language, and unavailable checkout copy are absent.

## README

Every prose sentence in the rewritten README is 22 words or fewer. The configuration key `[[invariants]]` remains only inside executable TOML because it is the public file format. Surrounding prose consistently says “check.”

No README sentence contains a banned marketing word.

## Terminology

| Concept | Required word |
| --- | --- |
| Command-line product | CLI |
| One execution | run |
| Saved JSON or Markdown result | witness |
| Query plus expected result | check |
| `mcw.toml` | config file |
| Shipped try-out | sample or demo |

