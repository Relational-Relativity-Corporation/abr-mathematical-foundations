# Lesson 1 — First Relational Calculation

> **Revision 2:** This was the Lesson 1 README in revision 1 (the two-locus Δ calculation). In revision 2 it is the **supporting exercise** for [`05_TOWARD_THE_PRIMARY_REGION.md`](05_TOWARD_THE_PRIMARY_REGION.md); the principal lesson starts at [`README.md`](README.md). The program for this exercise is now run with `cargo run --bin lesson01 -- <file>` instead of `cargo run -- <file>`. Its source (`src/main.rs`) is unchanged, and its four revision-1 run reports regenerate byte for byte.

**ABR Mathematical Foundations.** Draft, awaiting Origin review and independent Verifier review.

Mathematical authority: Kernel V9 (`abr-kernel` `1c1ef70`) and IRRS (`abr-irrs` `c7410c5`). This lesson implements no mathematics of its own. Every calculation and admission decision below comes from those two repositories, at those commits.

---

## What you will do (30–45 minutes)

You will measure two marks with a ruler. You will declare the relation between them and submit the declaration through the IRRS admission gate and the Kernel V9 admission interface. You will run the canonical directed-difference operator and then compare its result with a second, independent measurement of your own.

```
Observation → Declaration → IRRS admission → Canonical Kernel V9 execution
            → Calculated result → Comparison with observation
```

| Part | What you do | Time |
|---|---|---|
| 1 | Run the supplied **illustrative** example and read the report | 10 min |
| 2 | Measure two marks yourself and write your declaration ([`DECLARATION_GUIDE.md`](DECLARATION_GUIDE.md)) | 15 min |
| 3 | Run your declaration, then compare the result with an independent measurement ([`EXPERIMENT.md`](EXPERIMENT.md)) | 10 min |
| 4 | Move one mark, repeat, and see the result change | 10 min |

## Prerequisites

- A Rust toolchain (`cargo`) and `git`. The first build downloads the two pinned canonical crates from GitHub.
- A ruler with a millimetre scale, a strip of paper or tape, and a pencil.
- No prior knowledge of ABR is needed. Module 1 ([`../../modules/01_observation_and_abstraction.md`](../../modules/01_observation_and_abstraction.md)) gives the background, but it is optional.

## The observable relational primitive, in one paragraph

The starting point of Kernel V9 is *a directed relation declared from observable information* (`kernel_reference.md` §1). Here the observable information is two marks read on a ruler. The relation runs from one mark to the other, and **you** declare it from what you observed. The mathematics does not decide which marks are related, or in which direction: "the computation does not declare the relation" (`kernel_reference.md` §1). Once the relation is declared, the operator computes the directed difference across it:

> Δ(x)[e] = x[s] − x[t] for the declared relation e = (s, t), with s the source and t the target (`operators_notation_and_constraint.md`; `src/operators.rs`).

For a declared relation L1 → L2, the result is x[L1] − x[L2]. The sign follows the direction **you declared**, not the order in which the numbers were written down.

## Why declarations are necessary

The canonical kernel acts only on declared information. If a required declaration is missing, the operator "is not evaluated. No substitute value or classification is introduced" (`kernel_reference.md` §1). In this lesson you declare:

- where each reading came from and under what conditions,
- the instrument's printed scale (the declared domain),
- the measurement mapping M (here, Identity: the reading enters unchanged, in millimetres),
- the relation, its direction, and what you observed that supports them,
- which kernel applies.

Leave any of these out and the program does not guess. It reports the item as **OPEN** and withholds the result. Demonstration 4 shows this.

## Operator A and operator Δ: read this before you run

The lesson brief asks for operator A. Under the canonical rules, **A cannot be evaluated for two loci joined by one relation.**

- A belongs to the ABR kernel A → B → R. That kernel is admissible only when M establishes an *interior locus*, a locus that is the target of one declared relation and the source of another (s → t → u) (`kernel_reference.md` §10). Two marks with one relation have no interior locus.
- Where no interior locus is established, the canonical kernel is the Primary kernel, and its first operator is **Δ**. The canonical definition is: "Δ is A under a declared renaming … Formula identical to A" (`operators_notation_and_constraint.md`).
- The V9 admission interface admits the Primary kernel only. For the ABR kernel it reports "OPEN: ABR kernel A → B → R" (`src/admission.rs`).

So the lesson runs **canonical Δ** through the admitted canonical pathway, and Demonstration 3 shows the canonical refusal when A → B → R is requested. Operator A itself is not called and nothing stands in for it. Whether this is how Lesson 1 should treat operator A is an open question for Origin (see [`VERIFICATION_REPORT.md`](VERIFICATION_REPORT.md)).

## How to run

From this directory:

```
cargo run -- declarations/experiment_1_illustrative.txt
```

| Declaration file | Purpose |
|---|---|
| `declarations/experiment_1_illustrative.txt` | Demonstration 1: supplied example, L1 = 20 mm, L2 = 90 mm, L1 → L2 |
| `declarations/experiment_2_illustrative.txt` | Demonstration 2: L2 moved to 140 mm |
| `declarations/experiment_3_abr_request_illustrative.txt` | Demonstration 3: requests the ABR kernel (operator A); the canonical code declines |
| `declarations/experiment_4_incomplete_illustrative.txt` | Demonstration 4: measurement conditions left out; canonical admission returns OPEN |
| `declarations/my_measurement_TEMPLATE.txt` | Copy this file to record **your** measurement |

Each run prints a report and saves it under `records/`. Reference outputs are in [`EXPECTED_RESULTS.md`](EXPECTED_RESULTS.md).

## How to interpret the report

The report ends with a status summary. Read each line separately, because they answer different questions:

| Line | Question it answers |
|---|---|
| Code execution | Did the program run? |
| Declaration | Did you declare every Lesson 1 item? (**DECLARED** or a list of what is missing) |
| IRRS observation admission | Did the gate execute your M and find each reading inside your declared scale? |
| V9 provenance admission | Does every input trace to a declared observation with a named record and conditions, and does the independent reconstruction reproduce the calculation? |
| Canonical calculation | **CALCULATED** when the canonical Δ ran and was admitted; otherwise **OPEN** |
| Operator A | Why A was or was not evaluated |
| Observational verification | Did *you* compare the result with an independent observation? **VERIFIED** only for measured readings with your recorded agreement; **OPEN** otherwise; **DIVERGENT** if you recorded disagreement |

> **The illustrative values are not an observation.** With them, the result is a demonstration. The program reports observational verification as OPEN, and the V9 admission labels the result's observational status `Synthetic`. A passing build, a passing test, or agreement with the supplied numbers is not observational verification.

---

*Bounded over D. No claim beyond D.*
