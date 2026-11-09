# 3 — Canonical Operators and the Calculation (Exercise B, execution)

Content classes are as in [Document 1](01_BASIC_CALCULATION.md).

## 3.1 The pathway  [CANONICAL calls]

For each field the program (`src/pipeline.rs`) calls canonical kernel code:

1. Kernel selection under §10. Each field declares interior loci, so A → B → R applies, and "R is not optional".
2. Completeness of the declaration (§5 Step 2).
3. `operator_a`, `compute_rho`, `operator_b`, `operator_r` and `detection_indicator`, then an independent RPPI re-execution.

## 3.2 Operator outputs for the event

| Field | Execution |
|---|---|
| F1 at hazard onset | A, ρ, B, R, 𝟙 calculated; RPPI agreed |
| F1 at full braking | calculated |
| F2a elevation | calculated |
| F2b friction | calculated |
| F3 response chain | calculated |
| F5 vehicle states, position | calculated |
| F5 vehicle states, speed | calculated |

Every output in this document is a **NON-AUTHORITATIVE DIAGNOSTIC** of the declared fields, labelled so in the runtime report. All values are illustrative (ρ_base and χ₀ are illustrative; every node and edge is simulated, not observationally established).

## 3.3 Selected outputs (full tables in `records/LESSON1_REPORT.txt`)

| Relation | A | Reading |
|---|---|---|
| F1 onset, Air → Body | −31.521 m/s | w − v: the air moves at 31.5 m/s relative to the car, toward it |
| F1 onset, Body → Tread | 0.0 m/s | the tyre is rolling freely before braking |
| F1 full braking, Body → Tread | +3.929 m/s | slip velocity under braking |
| F2a, Sk → Sk+1 | +0.75 m on every segment | elevation drop per 25 m (−3 % grade) |
| F2b, S00 → S01 | +0.0105 | change in friction between stations (a patchy wet surface) |
| F3, Obstacle → Driver; Driver → Brake; Brake → Tread | −2.0 s; −0.45 s; −0.25 s | the negated perception, response and build-up latencies |

B, R and ρ were also calculated for every relation. 𝟙[e] = 1.0 on most relations. Under §11.4 that is a **review signal**, preserved as returned. It is not a finding.

## 3.4 Reconstruction: the stopping distance from the vehicle's states  [canonical operators on declared F5; diagnostic]

The operators calculate the directed difference between elements of the system on declared relations. The stopping distance is exactly such a difference: between the vehicle's position at hazard onset and its position at standstill. On field F5 (position), A[s → t] = x(s) − x(t):

| Canonical output on F5 | Value | Meaning |
|---|---|---|
| −A[Onset → Standstill] | **174.953 m** | **stopping distance** |
| −A[Onset → ResponseStart] | 53.483 m | distance during perception |
| −A[ResponseStart → BrakeOnset] | 11.990 m | distance during response |
| −A[BrakeOnset → FullBraking] | 6.620 m | distance during brake build-up |
| −A[FullBraking → Standstill] | 102.861 m | distance under full braking |
| −B[Onset → ResponseStart] = −(A[e] + A[ResponseStart → BrakeOnset]) | 65.472 m | distance from onset to brake onset |
| −B[BrakeOnset → FullBraking] = −(A[e] + A[FullBraking → Standstill]) | 109.481 m | distance from brake onset to standstill |

**Canonical B** (`operator_b`): B[e] = A[e] + Σ A[f] over f ∈ succ(e), where succ(e) is the set of relations leaving target(e). The two sums above contain exactly one relation each.

The sum checks: 65.472 + 109.481 − 174.953 = 0.

**These B outputs are not Exercise A's two terms.** Exercise A's t = 1.5 s covers perception, response *and* build-up (0.8 + 0.45 + 0.25 s in the base trial). Its v·t therefore spans Onset → FullBraking, which no single B output gives under this topology.

**State definitions** (from the simulator):
- ResponseStart = end of perception;
- BrakeOnset = brake pressure begins;
- FullBraking = full deceleration reached.

**This is a reconstruction.** It uses the event's own states, so it is not a prediction, and its agreement with the simulator's figure is a conformance check.

**Kernel V9 verification.** Onset → Standstill was declared alone (two loci, so the Primary kernel; Δ has the same formula as A) and run through `abr_kernel::admission::admit` at `1c1ef70`.
- V9 **ADMITTED** it, with observational status Simulated: Δ = −174.9531225782069 m, bit-identical to F5's A.
- The reverse traces end at the simulated position records (`records/expanded_event/V9_stopping_relation.report.txt`).
- This is provenance conformance, not empirical verification.

**On the stopping relation:**
- The full path gives A = B = R and 𝟙 = 0.
- No relation precedes Onset or follows Standstill, so both sums in R are over empty sets. R there therefore does not depend on the illustrative ρ_base and χ₀.
- B and R there are A plus sums over empty sets. The A-3 register (§18) names Σ, R and R_persistence. Whether it also covers B's empty successor sum is OPEN.

**F5 speed** gives the speed lost in each phase as A: 0.161, 0.036, 0.430 and 26.196 m/s.

[OPEN] The Onset → Standstill relation is declared so that the stopping distance is the difference on a single relation. Origin should confirm it (D-SD-12).

## 3.5 The expanded prediction  [EXPLANATORY, pending D-SD-4]

The relational result above is a **calculation from the observed states**. A *prediction* has to be made before the event, from its conditions.

The expanded Newtonian calculation (`src/calc.rs`) makes that prediction from the condition fields. The full provenance chain of every input is in the runtime report and in `VERIFICATION_REPORT.md` §4. Two findings are OPEN:
- **V-03a:** the wind projection is computed before the kernel pipeline.
- **V-03c:** the latencies are observed during the event, so they are not pre-event conditions.

Its inputs:
- the admitted Body and Air readings (F1);
- −A on F3 (latencies);
- atan(−A / spacing) on F2a (grade);
- the friction readings in F2b;
- declared mass, drag area, air density and g.

How it runs: constant speed through the reaction; a linear build-up of deceleration; then g·(μ·cosθ + sinθ) + drag/m in each segment. It gives **180.109 m**. It is not attributed to the kernel.

R and ρ on the condition fields depend on the illustrative ρ_base and χ₀, and are reported only.

*Bounded over D. No claim beyond D.*
