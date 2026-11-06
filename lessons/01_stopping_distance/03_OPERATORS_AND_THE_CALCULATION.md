# 3 — Canonical Operators and the Calculation (Exercise B, execution)

Content classes are as in [Document 1](01_BASIC_CALCULATION.md).

## 3.1 The pathway  [CANONICAL calls]

For each field the program (`src/pipeline.rs`) calls only canonical code:

1. `irrs_eval::Evaluator::evaluate`: IRRS declaration evaluation.
2. `relational_gate::RegionBuilder::establish` and `AdmissionGate::submit`: IRRS observation admission.
3. Kernel selection under §10. Each field declares interior loci, so A → B → R applies, and "R is not optional".
4. Completeness of the declaration (§5 Step 2).
5. `irrs_session::execute`: `operator_a`, `compute_rho`, `operator_b`, `operator_r` and `detection_indicator`, then an independent RPPI re-execution.

## 3.2 What IRRS returned for the event

| Field | IRRS evaluation | Admission | Execution |
|---|---|---|---|
| F1 at hazard onset | OPEN, 14 conditions, 0 violations | all 4 readings admitted | A, ρ, B, R, 𝟙 calculated; RPPI agreed |
| F1 at full braking | OPEN, 14, 0 | 4 admitted | calculated |
| F2a elevation | OPEN, 40, 0 | 17 admitted | calculated |
| F2b friction | OPEN, 40, 0 | 17 admitted | calculated |
| F3 response chain | OPEN, 14, 0 | 4 admitted | calculated |

**Why every field is OPEN:**
- the four Phase 2 declarations are absent;
- every node, edge and interior locus is marked "proposed", because it is simulated, not observationally established (GR-5);
- ρ_base and χ₀ are illustrative.

IRRS raised no violation. The lesson executes on OPEN, as Lesson 1 revision 2 did, and stops only on REJECTED.

## 3.3 Selected outputs (full tables in `records/LESSON1_REPORT.txt`)

| Relation | A | Reading |
|---|---|---|
| F1 onset, Air → Body | −31.521 m/s | w − v: the air moves at 31.5 m/s relative to the car, toward it |
| F1 onset, Body → Tread | 0.0 m/s | the tyre is rolling freely before braking |
| F1 full braking, Body → Tread | +3.929 m/s | slip velocity under braking |
| F2a, Sk → Sk+1 | +0.75 m on every segment | elevation drop per 25 m (−3 % grade) |
| F2b, S00 → S01 | +0.0105 | change in friction between stations (a patchy wet surface) |
| F3, Obstacle → Driver; Driver → Brake; Brake → Tread | −2.0 s; −0.45 s; −0.25 s | the negated perception, response and build-up latencies |

B, R and ρ were also calculated for every relation. 𝟙[e] = 1.0 on 27 of the 41 relations. Under §11.4 that is a **review signal**, preserved as returned. It is not a finding.

## 3.4 Exactly how the outputs reach the stopping distance  [CANONICAL limit, then EXPLANATORY]

**No Kernel V9 operator calculates a stopping distance, and no canonical mapping exists from A, B, R, ρ or 𝟙 to one.** A projection C is "Not a kernel operator" (§10).

The calculation uses only admitted node values and A outputs, through declared projections (D-SD-3):

| Calculation input | Source | Projection |
|---|---|---|
| initial speed | admitted Body reading, F1 onset | none |
| along-road wind | admitted Air reading, F1 onset | M: speed × cos direction |
| perception, response and build-up times | A on the F3 relations | sign reversal |
| grade of segment k | A[Sk → Sk+1] in F2a | atan(−A / spacing) |
| friction of segment k | admitted reading at Sk in F2b | none |
| mass, drag area, air density, g | declared calculation inputs | none (not kernel inputs) |

**Not used:** B, R, ρ, 𝟙 and the slip velocity. They are reported, but none has a canonical meaning as a distance (OPEN).

**The expanded calculation** (`src/calc.rs`, Generator-proposed and pending Origin authorization D-SD-4) is Newtonian:
1. constant speed through the reaction;
2. a linear build-up of deceleration;
3. then a constant deceleration in each segment, g·(μ·cosθ + sinθ) + drag/m, with drag taken at the speed entering the segment.

It is not attributed to the kernel. For this event it gives **180.109 m** (reaction 65.715 m + build-up 6.669 m + braking 107.725 m).

*Bounded over D. No claim beyond D.*
