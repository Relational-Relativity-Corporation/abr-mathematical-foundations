# Lesson 1 — First Relational Calculation

**ABR Mathematical Foundations.** Revision 2, draft. Awaiting Origin review and independent Verifier review.

Mathematical authority: Kernel V9 (`abr-kernel` `1c1ef70`) and IRRS (`abr-irrs` `c7410c5`). This lesson implements no mathematics of its own. Every calculation and every admission decision comes from those two repositories, at those commits.

---

## What you will do (about 90 minutes)

You start from a **complete bounded observable set**: four marks on a marble ramp, the height of each mark, and the relations the marble's observed passage establishes between them. You will:

1. declare the set (its domain D, its measurement M, its loci and its relations);
2. submit the declaration to IRRS for evaluation and admission;
3. run the canonical Kernel V9 operators that apply to it, A → B → R;
4. read what the results establish about the relations in the set;
5. look at a single relation alone, and see why that brings you to the condition the kernel calls *Primary*;
6. compare calculated results with your own direct observations.

```
Observation → Declaration → IRRS admission → Kernel calculation
            → Observable outcome → Comparison
```

| Document | What it covers | Time |
|---|---|---|
| [`01_INTRODUCTION.md`](01_INTRODUCTION.md) | The observable relational primitive and the purpose of this lesson | 10 min |
| [`02_OBSERVABLE_SET.md`](02_OBSERVABLE_SET.md) | Stage A–B: establishing D, M, loci and topology for the ramp | 20 min |
| [`03_CANONICAL_CALCULATION_GUIDE.md`](03_CANONICAL_CALCULATION_GUIDE.md) | Stage B–D: submitting, executing and reading the outputs | 20 min |
| [`04_EXPERIMENTAL_EXERCISES.md`](04_EXPERIMENTAL_EXERCISES.md) | Exercises: change the observation and repeat | 20 min |
| [`05_TOWARD_THE_PRIMARY_REGION.md`](05_TOWARD_THE_PRIMARY_REGION.md) | Stage E: from the whole set toward the Primary condition, and where this lesson stops | 15 min |
| [`06_VERIFICATION_GUIDE.md`](06_VERIFICATION_GUIDE.md) | Stage F: comparing results with observation and preserving evidence | 15 min |

For reviewers: [`VERIFICATION_REPORT.md`](VERIFICATION_REPORT.md) is the revision-2 evidence and self-audit.

## Quick start

```
git clone https://github.com/Relational-Relativity-Corporation/abr-mathematical-foundations.git
cd abr-mathematical-foundations/lessons/01_first_relational_calculation
cargo run -- declarations/set_1_ramp_illustrative.json
```

The first build downloads the pinned canonical crates from GitHub. The report is printed and saved in `records/`.

**The supplied sets are illustrative.** They show how the calculation runs. They are not observations, and nothing they produce is observationally verified. Exercise 4 in Document 4 has you measure your own ramp.

## Prerequisites

- A Rust toolchain (`cargo`) and `git`.
- For your own measurement: a grooved ramp (a length of curtain track, a folded strip of card, or a ruler with a central groove), a few books to prop it on, a marble, a pencil, and a ruler with a millimetre scale. A small set-square or spirit level helps with the comparison readings.
- No prior knowledge of ABR is needed. Module 1 ([`../../modules/01_observation_and_abstraction.md`](../../modules/01_observation_and_abstraction.md)) is optional background.

## Files

| Path | What it is |
|---|---|
| `src/bin/observable_set.rs` | Principal program (`cargo run -- <file>.json`): IRRS evaluation, IRRS admission, canonical A → B → R |
| `src/main.rs` | Supporting program (`cargo run --bin lesson01 -- <file>.txt`): two loci, canonical Δ, V9 admission. Unchanged from revision 1 |
| `declarations/set_*.json` | Supplied illustrative sets for the principal exercise |
| `declarations/my_set_TEMPLATE.json` | Template for your own measured set |
| `declarations/experiment_*.txt` | Supplied two-locus declarations for the supporting exercise (1–4 from revision 1; 5 is one relation of the ramp) |
| `records/` | Run reports (`*.report.txt`) and IRRS evaluation records (`*.irrs_eval.json`) |
| `Cargo.toml`, `Cargo.lock` | Pinned canonical dependencies and their resolved revisions |
| `SUPPORTING_PRIMARY_PAIR.md`, `DECLARATION_GUIDE.md`, `EXPERIMENT.md`, `EXPECTED_RESULTS.md` | Revision-1 documents, kept for the supporting exercise |
| `VERIFICATION_REPORT_R1.md` | Revision-1 verification report, kept unchanged |

## Status

| | Status |
|---|---|
| Declaration of the supplied sets | DECLARED (illustrative) |
| IRRS declaration evaluation (`irrs-eval`) | OPEN: 14 conditions, 0 violations (illustrative support, illustrative parameters, Phase 2 undeclared) |
| IRRS observation admission (`relational-gate`) | ADMITTED: every reading |
| Canonical calculation (A, ρ, B, R, 𝟙) | CALCULATED from illustrative inputs |
| V9 provenance admission of A → B → R | OPEN: not in canonical scope at `1c1ef70` |
| Observational verification | OPEN: no actual observation has been compared |
| Primary Region | OPEN: not observed and not reconstructed |

*Bounded over D. No claim beyond D.*
