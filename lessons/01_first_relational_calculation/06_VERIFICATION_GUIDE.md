# 6 — Verification Guide

Stage F: compare calculated results with observation, and keep the evidence. Content classes as in [Document 1](01_INTRODUCTION.md).

---

## 6.1 Four statuses that are not interchangeable  [CANONICAL basis]

| Status | Meaning in this lesson | What it does **not** mean |
|---|---|---|
| **DECLARED** | You stated it: a reading, a relation, a parameter | that it is correct |
| **CALCULATED** | A canonical operator returned it from admitted inputs | that it corresponds to anything observed |
| **VERIFIED** | A calculated quantity was compared with an independent **measured** observation, every comparison field was recorded, and you judged that they agree | that IRRS or a Verifier has confirmed it (see §6.4) |
| **OPEN** | Not evaluated, not admitted, or not compared | 0.0, false or failure |

"Code execution alone is not observational verification." Kernel V9 classifies a result in one of five ways and forbids collapsing them into PASS/FAIL (`kernel_reference.md` §16.3):

- conformant and correspondent;
- conformant and divergent;
- nonconformant;
- not evaluable, because a required declaration is missing;
- unresolved, because the observable comparison is insufficient.

**Illustrative sets can never be VERIFIED.** Their readings are not observations, so agreement with them shows nothing. The program prints "OPEN — DEMONSTRATION ONLY" for every comparison on an illustrative set.

## 6.2 What you can compare, and how  [EXPLANATORY]

Compare only a calculated quantity that has an **observable counterpart you measure independently**: by a different act of measurement, not by doing arithmetic on your own declared readings.

| Calculated quantity | Independent observation you can make | Method |
|---|---|---|
| A[P1 → P2], A[P2 → P3], A[P3 → P4] | the vertical drop from one mark to the next, measured directly | Hold a set-square or spirit level horizontal at the higher mark, and measure the vertical distance down to the lower mark with the ruler. Do not reuse the height readings |
| B[P1 → P2] | the vertical drop from P1 to P3, measured directly in the same way | [EXPLANATORY] Under this declaration, B[P1 → P2] combines A on P1 → P2 with A on its one declared successor, P2 → P3, so its counterpart is the drop across both. Declaring this correspondence is your decision as Origin, and you should state it in `method` |
| R, ρ, 𝟙 | **none declared in this domain** | These depend on ρ_base and χ₀, which have no observational basis here. Leave them OPEN; do not invent a counterpart |

Write each comparison into the `comparisons` block of your declaration:

```json
{ "quantity": "A[P1->P2]",
  "direct_observation": "61 mm (set-square at P1, ruler down to P2)",
  "uncertainty": "±1 mm",
  "method": "direct vertical drop between the marks; independent of the height readings",
  "outcome": "agrees" }
```

`outcome` is `agrees`, `disagrees` or `not_compared`. The decision is yours. The program never computes agreement from numbers. It prints your judgement beside the calculated value. A status of **VERIFIED** appears only when `observation_status` is `measured`, all four fields are filled in, and the outcome is `agrees`. `disagrees` gives **DIVERGENT**. Anything else stays **OPEN**.

## 6.3 When they disagree  [CANONICAL]

"Divergence is data" (§16.4). Keep it. Do not change readings, topology or parameters to make it go away, and do not add relations. Record where it arises: declared input → operator → calculated result → comparison target → divergence. It may come from the declaration, from M, from the topology, from your comparison measurement, or from an OPEN kernel condition. "Do not determine which explanation is correct without evidence." You, as Origin, decide whether to revise the declaration. If you do, revise it as a new set, and keep the old report.

## 6.4 Preserving evidence  [EXPLANATORY]

For each set you run, keep:

- your declaration file (its SHA-256 is printed at the top of the report);
- `records/<name>.report.txt`, the full run report;
- `records/<name>.irrs_eval.json`, the complete IRRS evaluation record;
- your notebook pages, named by the evidence identifiers in your declaration;
- `Cargo.lock`, which fixes the canonical revisions the run used (the report also prints them).

Never edit a report by hand. To correct something, change the declaration and run again.

[OPEN] Whether a student's recorded comparison may be called VERIFIED is returned to Origin (Q-5, carried from revision 1). IRRS reserves observational correspondence evaluation for its planned stage M4 (`abr-irrs` README: "Candidate correspondence evaluation is **not operational**"). Until Origin decides, treat a VERIFIED line as "the student's recorded comparison agrees". It still needs independent Verifier review.

## 6.5 Checklist before you call a result verified

- [ ] `observation_status` is `measured`, and every reading is in your notebook.
- [ ] Every node and relation you marked `observed` has an evidence entry.
- [ ] The comparison observation was made independently of the declared readings.
- [ ] `direct_observation`, `uncertainty`, `method` and `outcome` are all filled in.
- [ ] The report and IRRS record are saved and unedited.
- [ ] Every quantity you did not compare is still listed as OPEN.

*Bounded over D. No claim beyond D.*
