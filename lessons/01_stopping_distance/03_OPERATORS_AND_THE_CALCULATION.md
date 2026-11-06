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
| F5 vehicle states, position | OPEN, 17, 0 | 5 admitted | calculated |
| F5 vehicle states, speed | OPEN, 17, 0 | 5 admitted | calculated |

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

B, R and ρ were also calculated for every relation. 𝟙[e] = 1.0 on most relations. Under §11.4 that is a **review signal**, preserved as returned. It is not a finding.

## 3.4 The stopping distance, calculated by the canonical operators  [CANONICAL operators on declared F5]

The operators calculate the directed difference between elements of the system on declared relations. The stopping distance is exactly such a difference: between the vehicle's position at hazard onset and its position at standstill. On field F5 (position), A[s → t] = x(s) − x(t):

| Canonical output on F5 | Value | Meaning |
|---|---|---|
| −A[Onset → Standstill] | **174.953 m** | **stopping distance** |
| −A[Onset → ResponseStart] | 53.483 m | distance during perception |
| −A[ResponseStart → BrakeOnset] | 11.990 m | distance during response |
| −A[BrakeOnset → FullBraking] | 6.620 m | distance during brake build-up |
| −A[FullBraking → Standstill] | 102.861 m | distance under full braking |
| −B[Onset → ResponseStart] = −(A[e] + A[ResponseStart → BrakeOnset]) | 65.472 m | **reaction distance**, the counterpart of v·t |
| −B[BrakeOnset → FullBraking] = −(A[e] + A[FullBraking → Standstill]) | 109.481 m | **braking distance**, the counterpart of v²/(2a) |

reaction + braking − stopping = 0.

The basic calculation *assumes* t and a. The relational calculation obtains each term as a difference between observed states.

**On the stopping relation:**
- The full path gives A = B = R and 𝟙 = 0.
- No relation precedes Onset or follows Standstill, so both sums in R are over empty sets. R there therefore does not depend on the illustrative ρ_base and χ₀.
- It does rest on empty sums evaluating to 0.0, which is kernel open condition A-3 (§18). A and B do not.

**F5 speed** gives the speed lost in each phase as A: 0.161, 0.036, 0.430 and 26.196 m/s.

[OPEN] The Onset → Standstill relation is declared so that the stopping distance is the difference on a single relation. Origin should confirm it (D-SD-12).

## 3.5 The expanded prediction  [EXPLANATORY, pending D-SD-4]

The relational result above is a **calculation from the observed states**. A *prediction* has to be made before the event, from its conditions.

The expanded Newtonian calculation (`src/calc.rs`) makes that prediction from the condition fields. Its inputs:
- the admitted Body and Air readings (F1);
- −A on F3 (latencies);
- atan(−A / spacing) on F2a (grade);
- the friction readings in F2b;
- declared mass, drag area, air density and g.

How it runs: constant speed through the reaction; a linear build-up of deceleration; then g·(μ·cosθ + sinθ) + drag/m in each segment. It gives **180.109 m**. It is not attributed to the kernel.

R and ρ on the condition fields depend on the illustrative ρ_base and χ₀, and are reported only.

*Bounded over D. No claim beyond D.*
