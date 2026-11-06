# 2 — The Expanded Relational Field (Exercise B, declaration)

The same stopping event, described with twelve more kinds of observable information. Content classes are as in [Document 1](01_BASIC_CALCULATION.md).

## 2.1 The rules this declaration follows  [CANONICAL]

- D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ }, and M : O → D is declared before any operator acts. A value in D is the numerical projection of an observable through M, not the observable itself (`kernel_reference.md` §9).
- A NodeField holds a value for **every** declared component at **every** declared node (§12.2). "Not evaluated" is never 0.0.
- A acts on one component across the two ends of a declared relation: A[e] = x[s] − x[t] (§10).
- A relation needs an observational basis, a single declared direction and a provenance class. Rings are inadmissible (§8).
- ρ_base and χ₀ are supplied by M. The kernel has no default for either (§9).

Because of the NodeField rule, the twelve items cannot share one field: lux, seconds, metres and friction have no common component. They are split into separate declared fields, plus trial conditions and plain calculation inputs.

## 2.2 Where each item goes

| # | Item | Enters as |
|---|---|---|
| 1 | Tyre–road friction coefficient | Field F2b, a reading at each road station |
| 2 | Road surface and moisture | A trial condition, acting through item 1 |
| 3 | Tyre condition | A trial condition, acting through item 1 |
| 4 | Vehicle mass | A calculation input only: one value at one locus has no relation |
| 5 | Brake force and activation history | F3, brake-onset and full-braking times. The history over time is Phase 2: OPEN |
| 6 | Wind velocity and direction | F1, the Air locus. M projects the wind to its along-road component |
| 7 | Road grade | F2a, the elevation at each station |
| 8 | Ambient illumination | A trial condition, acting through F3 times |
| 9 | Visibility and obstacle detection | F3, obstacle and driver times |
| 10 | Driver attention and response history | F3, driver and brake times |
| 11 | Vehicle position and velocity | **F5, the vehicle's observed states**, and F1 |
| 12 | Changing deceleration | Phase 2: OPEN (§2.4) |

## 2.3 The fields  (declarations are written by `src/fields.rs` and saved in `records/expanded_event/*.declaration.json`)

| Field | Loci | Component (unit), extent | Relations and declared basis | Interior loci |
|---|---|---|---|---|
| **F1 velocity**, at hazard onset and at full braking | Air, Body, Tread, Road | along-road velocity (m/s), [−60, 60] | Air → Body *(coupling: the air acts on the body)*; Body → Tread *(continuation: the momentum-removal path)*; Tread → Road *(coupling: force at the contact patch)* | Body, Tread |
| **F2a elevation** | stations S00 … S16, every 25 m | elevation relative to S00 (m), [−50, 50] | Sk → Sk+1 *(continuation: the observed direction of travel)* | S01 … S15 |
| **F2b friction** | S00 … S16 | peak friction coefficient (1), [0, 1.5] | as F2a | S01 … S15 |
| **F3 response chain** | Obstacle, Driver, Brake, Tread | time from hazard onset (s), [0, 10] | Obstacle → Driver → Brake → Tread *(the observed signal path)* | Driver, Brake |
| **F5 vehicle states**, position and speed (two fields) | the vehicle at Onset, ResponseStart, BrakeOnset, FullBraking, Standstill | position from the onset position (m), [0, 600]; speed (m/s), [0, 60] | Onset → ResponseStart → BrakeOnset → FullBraking → Standstill *(continuation: the observed course of the event)*, plus Onset → Standstill *(the stopping event as a whole; D-SD-12)* | ResponseStart, BrakeOnset, FullBraking |

**F5 is the field the stopping distance belongs to.** The stopping distance is a relation between two observed states of the same vehicle: where it was at hazard onset and where it is at standstill. Declaring those states as loci, and the passage between them as relations, lets the canonical operators calculate it (Document 3).

**M for each field** is an identity on the declared reading, except at the Air locus. There M projects the measured wind speed and direction onto the direction of travel (speed × cos direction). The projection is declared with what it preserves (the along-road component) and what it discards (the crosswind) (D-SD-3).

**Direction bases** for F1 and F3 are Generator proposals. Whether they are physically justified is Origin's decision (D-SD-2).

**Parameters**, all illustrative and Generator-chosen (none is carried from another domain):

| Field | ρ_base | χ₀ |
|---|---|---|
| F1 | 0.5 | 1.0 m/s |
| F2a | 0.5 | 0.5 m |
| F2b | 0.5 | 0.05 |
| F3 | 0.5 | 0.25 s |
| F5 position, F5 speed | 0.5 | 5.0 m, 1.0 m/s |

## 2.4 What is not declared  [OPEN]

**Changing deceleration over the event and the brake activation history** (items 5 and 12) need Phase 2: a declared sequence of actual observations. The kernel states that "a model-generated trajectory is not an observable sequence through M unless the model is itself declared as a transducer from observable inputs" (§8). That declaration is Origin's (D-SD-6).

`irrs-session` at `c7410c5` also has no persistence execution path. The two F1 instants are therefore two separate spatial declarations, not a sequence.

*Bounded over D. No claim beyond D.*
