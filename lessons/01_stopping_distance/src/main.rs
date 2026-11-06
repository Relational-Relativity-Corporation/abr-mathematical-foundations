//! ABR Mathematical Foundations — Lesson 1: Newtonian stopping distance and
//! expanded relational calculation.
//!
//!   Exercise A  basic Newtonian calculation                 calc::basic       [EXPLANATORY]
//!   Exercise C  reference simulated braking event           sim::simulate     [EXPLANATORY, SIMULATED]
//!   Exercise B  declared fields → IRRS → canonical A→B→R    pipeline::run     [CANONICAL calls]
//!               → declared projections C → expanded calc    calc::expanded    [EXPLANATORY, pending D-SD-4]
//!   §7          interdependent conditions (four diamonds)
//!
//! Usage:  cargo run              (writes records/ and prints the summary)

mod calc;
mod fields;
mod pipeline;
mod sim;
mod v9;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_json::Value;

use calc::{ExpandedInputs, Segment};
use fields::FieldSpec;
use pipeline::{fmt, Outcome};
use sim::{Observation, Station, Trial, Vehicle};

const SIM_DECL: &str = include_str!("../declarations/simulator_illustrative.json");
const CARGO_LOCK: &str = include_str!("../Cargo.lock");
const MATERIAL_M: f64 = 1.0;

fn f(v: &Value, k: &str) -> f64 {
    v.get(k).and_then(Value::as_f64).unwrap_or_else(|| panic!("simulator declaration: '{k}' not declared"))
}

fn records() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("records")
}

fn save(rel: &str, body: &str) {
    let p = records().join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).expect("records dir");
    std::fs::write(&p, body).expect("write record");
}

fn locked_sources() -> Vec<String> {
    let wanted = ["relational-gate", "irrs-eval", "irrs-session", "irrs-kernel"];
    let mut out = Vec::new();
    let mut name = String::new();
    for line in CARGO_LOCK.lines() {
        if let Some(n) = line.strip_prefix("name = ") { name = n.trim_matches('"').to_string(); }
        if let Some(src) = line.strip_prefix("source = ") {
            if wanted.contains(&name.as_str()) { out.push(format!("{name}: {}", src.trim_matches('"'))); }
        }
    }
    out
}

/// Build a trial from the base trial with declared overrides.
fn trial(cfg: &Value, id: &str, over: &Value) -> Trial {
    let base = &cfg["base_trial"];
    let g = |k: &str| over.get(k).and_then(Value::as_f64).unwrap_or_else(|| f(base, k));
    let spacing = f(cfg, "station_spacing_m");
    let count = f(cfg, "station_count") as usize;
    let profile: Vec<f64> = over.get("friction_profile").or_else(|| base.get("friction_profile"))
        .and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_f64).collect()).unwrap_or_else(|| vec![1.0]);
    let (grade, mu) = (g("grade_percent"), g("friction"));
    let stations = (0..count).map(|k| {
        let x = k as f64 * spacing;
        Station {
            position_m: x,
            elevation_m: round9(grade / 100.0 * x),
            friction: round9(mu * profile[k.min(profile.len() - 1)]),
        }
    }).collect();
    Trial {
        id: id.into(),
        speed: g("speed_m_s"),
        perception: g("perception_s"),
        response: g("response_s"),
        buildup: g("brake_buildup_s"),
        wind_speed: g("wind_speed_m_s"),
        wind_direction_deg: g("wind_direction_deg"),
        peak_slip: g("peak_slip"),
        stations,
    }
}

fn round9(v: f64) -> f64 { (v * 1e9).round() / 1e9 }

/// The five Exercise B fields of one event.
fn event_fields(obs: &Observation, spacing: f64) -> Vec<FieldSpec> {
    vec![
        fields::f1_velocity(obs, "hazard_onset", obs.onset_velocity_m_s),
        fields::f1_velocity(obs, "full_braking", obs.full_braking_velocity_m_s),
        fields::f2_elevation(obs, spacing),
        fields::f2_friction(obs, spacing),
        fields::f3_response(obs),
        fields::f5_states(obs, "position"),
        fields::f5_states(obs, "speed"),
    ]
}

/// The stopping distance and its parts, as calculated by the canonical
/// operators on field F5 (vehicle states, position). Sign reversal only:
/// A[s->t] = x(s) − x(t), and positions increase along the direction of travel.
#[derive(Clone, Debug)]
struct Relational {
    stopping: f64,        // −A[Onset->Standstill]
    reaction: f64,        // −B[Onset->ResponseStart]   (perception + response)
    braking: f64,         // −B[BrakeOnset->FullBraking] (build-up + full braking)
    phases: [f64; 4],     // −A on the four chain relations
    speed_lost: [f64; 4], // A on the four chain relations of F5 speed
}

fn relational(pos: &Outcome, spd: &Outcome) -> Option<Relational> {
    if !pos.executed { return None; }
    let chain = ["Onset->ResponseStart", "ResponseStart->BrakeOnset", "BrakeOnset->FullBraking", "FullBraking->Standstill"];
    Some(Relational {
        stopping: -pos.a["Onset->Standstill"],
        reaction: -pos.b["Onset->ResponseStart"],
        braking: -pos.b["BrakeOnset->FullBraking"],
        phases: chain.map(|l| -pos.a[l]),
        speed_lost: chain.map(|l| spd.a.get(l).copied().unwrap_or(f64::NAN)),
    })
}

struct EventRun {
    outcomes: BTreeMap<String, Outcome>,
    inputs: Option<ExpandedInputs>,
    missing: Vec<String>,
    relational: Option<Relational>,
}

/// Run every field of one event through IRRS and the kernel, then build the
/// expanded-calculation inputs from admitted values and A outputs only.
fn run_event(obs: &Observation, spacing: f64, veh: &Vehicle, save_dir: Option<&str>) -> EventRun {
    let mut outcomes = BTreeMap::new();
    for spec in event_fields(obs, spacing) {
        let json = spec.to_json();
        let out = pipeline::run(&format!("{}.declaration.json", spec.stem), &json);
        if let Some(dir) = save_dir {
            save(&format!("{dir}/{}.declaration.json", spec.stem), &json);
            save(&format!("{dir}/{}.report.txt", spec.stem), &out.report);
            if let Some(j) = &out.irrs_json { save(&format!("{dir}/{}.irrs_eval.json", spec.stem), &(j.clone() + "\n")); }
        }
        outcomes.insert(spec.stem.clone(), out);
    }
    let mut missing = Vec::new();
    let mut need = |stem: &str, ok: bool| if !ok { missing.push(stem.to_string()); };
    let f1 = &outcomes["F1_velocity_hazard_onset"];
    let el = &outcomes["F2_road_elevation"];
    let fr = &outcomes["F2_road_friction"];
    let f3 = &outcomes["F3_response_chain"];
    need("F1_velocity_hazard_onset", f1.executed);
    need("F2_road_elevation", el.executed);
    need("F2_road_friction", fr.admission.starts_with("ADMITTED"));
    need("F3_response_chain", f3.executed);
    if !missing.is_empty() {
        let rel = relational(&outcomes["F5_vehicle_states_position"], &outcomes["F5_vehicle_states_speed"]);
        return EventRun { outcomes, inputs: None, missing, relational: rel };
    }
    // Declared projections C (DESIGN §5.1, D-SD-3). Each line states its source.
    let speed = f1.admitted["Body"];                         // admitted node value
    let wind_along = f1.admitted["Air"];                     // admitted node value (M-projected)
    let perception = -f3.a["Obstacle->Driver"];              // −A: sign reversal
    let response = -f3.a["Driver->Brake"];                   // −A
    let buildup = -f3.a["Brake->Tread"];                     // −A
    let n = obs.stations.len();
    let segments = (0..n - 1).map(|k| {
        let (a, b) = (format!("S{k:02}"), format!("S{:02}", k + 1));
        let drop = el.a[&format!("{a}->{b}")];               // A = z_k − z_{k+1}
        Segment {
            start_m: k as f64 * spacing,
            length_m: spacing,
            angle: (-drop / spacing).atan(),                 // C: uphill angle from drop ÷ spacing
            friction: fr.admitted[&a],                       // admitted node value
        }
    }).collect();
    let rel = relational(&outcomes["F5_vehicle_states_position"], &outcomes["F5_vehicle_states_speed"]);
    EventRun {
        relational: rel,
        outcomes,
        inputs: Some(ExpandedInputs {
            speed, perception, response, buildup, wind_along, segments,
            mass: veh.mass, drag_area: veh.drag_area, air_density: veh.air_density, g: veh.g,
        }),
        missing,
    }
}

fn main() {
    // `cargo run -- <field declaration .json>` runs one declaration (for example a
    // copy of records/expanded_event/*.declaration.json that you have edited)
    // through IRRS and the kernel, and prints its report. Nothing is saved.
    if let Some(path) = std::env::args().nth(1) {
        let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| { eprintln!("could not read '{path}': {e}"); std::process::exit(2) });
        let name = Path::new(&path).file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let out = pipeline::run(&name, &raw);
        print!("{}", out.report);
        println!("\nIRRS evaluation: {}\nGate admission: {}\nAuthorized execution: {}\nCalculation: {}", out.irrs_status, out.admission,
            if out.authorized { "AUTHORIZED" } else { "NOT PERFORMED (IRRS not authorized)" }, out.calculation);
        return;
    }
    let cfg: Value = serde_json::from_str(SIM_DECL).expect("simulator declaration is valid JSON");
    let dt = f(&cfg, "integration_step_s");
    let spacing = f(&cfg, "station_spacing_m");
    let v = &cfg["vehicle"];
    let veh = Vehicle {
        mass: f(v, "mass_kg"), drag_area: f(v, "drag_area_m2"), air_density: f(v, "air_density_kg_m3"),
        crr: f(v, "rolling_resistance_coefficient"), g: f(v, "gravity_m_s2"),
    };
    let mut r = String::new();
    let _ = writeln!(r, "ABR Mathematical Foundations — Lesson 1: Newtonian stopping distance and expanded relational calculation");
    let _ = writeln!(r, "ALL INPUT VALUES ARE ILLUSTRATIVE (Generator-chosen, declarations/simulator_illustrative.json).");
    let _ = writeln!(r, "THE REFERENCE OBSERVATION IS SIMULATED. NOTHING HERE IS EMPIRICAL EVIDENCE OR OBSERVATIONALLY VERIFIED.");
    let _ = writeln!(r, "\nCanonical authorities (resolved by Cargo.lock):");
    for s in locked_sources() { let _ = writeln!(r, "  {s}"); }

    // ── Exercise A ────────────────────────────────────────────────────────
    let bn = &cfg["baseline_newtonian"];
    let (v0, tr0, a0) = (f(bn, "speed_m_s"), f(bn, "reaction_time_s"), f(bn, "deceleration_m_s2"));
    let basic = calc::basic(v0, tr0, a0);
    let _ = writeln!(r, "\n================ Exercise A — Basic Newtonian calculation [EXPLANATORY, not kernel] ================");
    let _ = writeln!(r, "  d = v·t + v²/(2a),  v = {v0} m/s (60 mph), t = {tr0} s, a = {a0} m/s², level road");
    let _ = writeln!(r, "  reaction distance v·t      = {:.4} m", basic.reaction_m);
    let _ = writeln!(r, "  braking distance  v²/(2a)  = {:.4} m", basic.braking_m);
    let _ = writeln!(r, "  stopping distance          = {:.4} m", basic.total_m);
    let sim_basic = sim::simulate_basic(v0, tr0, a0, dt);
    let _ = writeln!(r, "  Simulator conformance check (Exercise A assumptions through the same integrator, dt = {dt} s):");
    let _ = writeln!(r, "    simulated {:.4} m; difference from hand calculation {:+.4} m (integration only; not physical accuracy)",
        sim_basic, sim_basic - basic.total_m);

    // ── Exercise C reference + Exercise B expanded, for the one event ─────
    let ev = &cfg["expanded_event"];
    let tr = trial(&cfg, "expanded_event", ev);
    let obs = sim::simulate(&tr, &veh, dt);
    save("expanded_event/reference_observation.json", &(serde_json::to_string_pretty(&obs).unwrap() + "\n"));
    let run = run_event(&obs, spacing, &veh, Some("expanded_event"));
    let _ = writeln!(r, "\n================ Exercise B — Expanded relational field (event '{}') ================", tr.id);
    let _ = writeln!(r, "  Conditions: worn tread, wet road (friction {}..{}), grade {} %, wind {} m/s toward {} deg (along-road {:+.3} m/s),",
        obs.stations.iter().map(|s| s.friction).fold(f64::INFINITY, f64::min),
        obs.stations.iter().map(|s| s.friction).fold(0.0, f64::max),
        f(ev, "grade_percent"), tr.wind_speed, tr.wind_direction_deg, tr.wind_along());
    let _ = writeln!(r, "              dusk + distracted (perception {} s), response {} s, brake build-up {} s, mass {} kg", tr.perception, tr.response, tr.buildup, veh.mass);
    let _ = writeln!(r, "\n  Each field: IRRS declaration evaluation → gate admission → canonical execution (full reports: records/expanded_event/)");
    let _ = writeln!(r, "  Four statuses, kept separate: IRRS declaration evaluation | gate admission | authorized execution | calculation.");
    let _ = writeln!(r, "  {:28} {:44} {:24} {:14} {}", "field", "IRRS evaluation", "gate admission", "authorized?", "calculation");
    for (stem, o) in &run.outcomes {
        let _ = writeln!(r, "  {:28} {:44} {:24} {:14} {}", stem, o.irrs_status, o.admission,
            if o.authorized { "AUTHORIZED" } else { "NO (held)" },
            if o.executed && !o.authorized { "DIAGNOSTIC only (non-authoritative)" } else if o.executed { "CALCULATED" } else { &o.calculation });
    }
    let any_unauth = run.outcomes.values().any(|o| !o.authorized);
    if any_unauth {
        let _ = writeln!(r, "\n  AUTHORIZED EXECUTION: NOT PERFORMED for any field. IRRS returned OPEN, and an OPEN record is 'not evaluated;");
        let _ = writeln!(r, "  build held' (abr-irrs eval/README.md); it 'cannot be authorized' (irrs-eval authorization.rs rule 1; record.rs).");
        let _ = writeln!(r, "  EVERY OPERATOR OUTPUT BELOW IS A NON-AUTHORITATIVE DIAGNOSTIC of the declared fields. The unresolved IRRS");
        let _ = writeln!(r, "  conditions are listed verbatim in each field report (records/expanded_event/*.report.txt). Distinct conditions:");
        let mut distinct: Vec<String> = Vec::new();
        for o in run.outcomes.values() {
            for c in &o.open_list {
                // Blank the quoted names so one line stands for each kind of condition.
                let mut key = String::new();
                let mut quoted = false;
                for ch in c.chars() {
                    if ch == '\'' { quoted = !quoted; key.push_str(if quoted { "'<name" } else { ">'" }); }
                    else if !quoted { key.push(ch); }
                }
                if !distinct.contains(&key) { distinct.push(key); }
            }
        }
        for c in &distinct { let _ = writeln!(r, "    OPEN: {c}"); }
    }
    let show = |r: &mut String, stem: &str, o: &Outcome| {
        let _ = writeln!(r, "\n  {stem}: canonical outputs");
        let _ = writeln!(r, "    {:14} {:>14} {:>14} {:>14} {:>4}", "relation", "A", "B", "R", "𝟙");
        for (l, a) in &o.a {
            let _ = writeln!(r, "    {:14} {:>14} {:>14} {:>14} {:>4}", l, fmt(*a), fmt(o.b[l]), fmt(o.r[l]), fmt(o.indicator[l]));
        }
        let rho: Vec<String> = o.rho.iter().map(|(k, v)| format!("{k} {}", fmt(*v))).collect();
        let _ = writeln!(r, "    ρ: {}", rho.join(", "));
    };
    for stem in ["F1_velocity_hazard_onset", "F1_velocity_full_braking", "F3_response_chain"] {
        show(&mut r, stem, &run.outcomes[stem]);
    }
    let el = &run.outcomes["F2_road_elevation"];
    let fr = &run.outcomes["F2_road_friction"];
    let _ = writeln!(r, "\n  F2_road_elevation and F2_road_friction: 16 relations each (full tables in records/expanded_event/). First four:");
    let _ = writeln!(r, "    {:10} {:>12} {:>12} {:>12} | {:>12} {:>12} {:>12}", "relation", "A elev (m)", "B", "R", "A μ", "B", "R");
    for l in el.a.keys().take(4) {
        let _ = writeln!(r, "    {:10} {:>12} {:>12} {:>12} | {:>12} {:>12} {:>12}", l, fmt(el.a[l]), fmt(el.b[l]), fmt(el.r[l]), fmt(fr.a[l]), fmt(fr.b[l]), fmt(fr.r[l]));
    }
    let signals: usize = run.outcomes.values().map(|o| o.indicator.values().filter(|&&v| v != 0.0).count()).sum();
    let rels: usize = run.outcomes.values().map(|o| o.indicator.len()).sum();
    let _ = writeln!(r, "\n  𝟙[e] = 1.0 (review signal, §11.4) on {signals} of {rels} relations across the seven event fields.");

    // Relationship between kernel outputs and the calculation.
    for stem in ["F5_vehicle_states_position", "F5_vehicle_states_speed"] {
        show(&mut r, stem, &run.outcomes[stem]);
    }
    let _ = writeln!(r, "\n  ---- RECONSTRUCTION: the stopping distance from the vehicle's (simulated) observed states, F5 [DIAGNOSTIC; IRRS OPEN] ----");
    match &run.relational {
        Some(q) => {
            let _ = writeln!(r, "    Canonical A (irrs-kernel operators.rs): A[e] = x[source(e)] − x[target(e)].");
            let _ = writeln!(r, "    Canonical B (operators.rs operator_b): B[e] = A[e] + Σ A[f] over f in succ(e), succ(e) = relations leaving target(e).");
            let _ = writeln!(r, "    stopping distance                       = −A[Onset->Standstill]       = {:.3} m", q.stopping);
            let _ = writeln!(r, "    distance, onset to brake onset          = −B[Onset->ResponseStart]    = {:.3} m", q.reaction);
            let _ = writeln!(r, "        succ(Onset->ResponseStart) = {{ResponseStart->BrakeOnset}}: perception {:.3} + response {:.3}", q.phases[0], q.phases[1]);
            let _ = writeln!(r, "    distance, brake onset to standstill     = −B[BrakeOnset->FullBraking] = {:.3} m", q.braking);
            let _ = writeln!(r, "        succ(BrakeOnset->FullBraking) = {{FullBraking->Standstill}}: build-up {:.3} + full braking {:.3}", q.phases[2], q.phases[3]);
            let _ = writeln!(r, "    Phase accounting: ResponseStart = end of perception (foot leaves the accelerator); BrakeOnset = brake pressure");
            let _ = writeln!(r, "    begins; FullBraking = full deceleration reached (simulator states, sim.rs). Each state position is the first");
            let _ = writeln!(r, "    integration step at or after the declared time (step 0.1 ms; ≤ about 3 mm at 27 m/s).");
            let _ = writeln!(r, "    Check: (onset→brake onset) + (brake onset→standstill) − stopping = {:+.3e} m", q.reaction + q.braking - q.stopping);
            let _ = writeln!(r, "    Interpretation limit: Exercise A's t = 1.5 s covers perception + response + build-up (the base trial splits it");
            let _ = writeln!(r, "    0.8 + 0.45 + 0.25 s). Its 'reaction distance' v·t therefore spans Onset->FullBraking, which no single B output");
            let _ = writeln!(r, "    gives under this topology. The two B outputs are NOT Exercise A's reaction and braking distances.");
            let _ = writeln!(r, "    speed lost per phase (A on F5 speed): perception {:.3}, response {:.3}, build-up {:.3}, full braking {:.3} m/s",
                q.speed_lost[0], q.speed_lost[1], q.speed_lost[2], q.speed_lost[3]);
            let pos = &run.outcomes["F5_vehicle_states_position"];
            let l = "Onset->Standstill";
            let _ = writeln!(r, "    On Onset->Standstill: A = B = R = {} m, 𝟙 = {}. succ and pred of this relation are empty, so B and R", fmt(pos.a[l]), fmt(pos.indicator[l]));
            let _ = writeln!(r, "    there are A plus sums over empty sets. The register text of A-3 (§18) names Σ, R and R_persistence; whether it");
            let _ = writeln!(r, "    also covers B's empty successor sum is OPEN (returned to Origin). A itself has no sum.");
            let _ = writeln!(r, "    This is a RECONSTRUCTION from the event's own states. It is not a prediction, and agreement with the");
            let _ = writeln!(r, "    simulator's figure is a reconstruction check (conformance), not a correspondence (GOVERNING_DECLARATIONS.md).");
        }
        None => { let _ = writeln!(r, "    OPEN — F5 was not executed"); }
    }

    // ── Kernel V9 provenance admission of the stopping relation ───────────
    let v9r = v9::admit_stopping_relation(&tr.id, obs.state_positions_m[0], obs.state_positions_m[4]);
    save("expanded_event/V9_stopping_relation.report.txt", &v9r.report);
    let _ = writeln!(r, "\n  ---- Kernel V9 verification of the reconstruction (abr-kernel 1c1ef70, admission::admit) ----");
    let _ = writeln!(r, "    Onset -> Standstill declared alone: two loci, no interior locus → Primary kernel, Δ(x)[e] = x[s] − x[t]");
    let _ = writeln!(r, "    (formula identical to A). V9 reconstructs it from the declaration records and traces it to its terminus.");
    let _ = writeln!(r, "    V9 admission: {}", v9r.status);
    if let (Some(d), Some(q)) = (v9r.delta, &run.relational) {
        let _ = writeln!(r, "    V9-admitted Δ[Onset->Standstill] = {} m;  F5 A[Onset->Standstill] = {} m;  difference {:+.3e} m",
            fmt(d), fmt(-q.stopping), d + q.stopping);
    }
    let _ = writeln!(r, "    Full record: records/expanded_event/V9_stopping_relation.report.txt. A Simulated terminus is not empirical evidence.");

    // ── Provenance of every prediction input (V-03) ───────────────────────
    let f1 = &run.outcomes["F1_velocity_hazard_onset"];
    let f3 = &run.outcomes["F3_response_chain"];
    let el = &run.outcomes["F2_road_elevation"];
    let fr = &run.outcomes["F2_road_friction"];
    let st = |o: &Outcome| format!("IRRS {} / gate {}", o.irrs_status.split(" — ").next().unwrap_or("?"), o.admission.split(" — ").next().unwrap_or("?"));
    let _ = writeln!(r, "\n  ---- Provenance of every input to the expanded prediction (application mathematics, src/calc.rs) ----");
    let _ = writeln!(r, "    {:22} | {:34} | {:22} | {:22} | {:22} | {}", "input", "declared observable / parameter", "IRRS / gate", "canonical output", "projection C", "destination");
    let rows: Vec<[String; 6]> = vec![
        ["initial speed".into(), "obs.Body, F1 onset (simulated)".into(), st(f1), "none (node value)".into(), "none".into(), "reaction and braking phases".into()],
        ["along-road wind".into(), "obs.Air, F1 onset (simulated)".into(), st(f1), "none (node value)".into(), "speed×cos(dir) IN sim.rs, BEFORE the gate (V-03a)".into(), "drag term".into()],
        ["perception time".into(), "obs.Obstacle, obs.Driver, F3".into(), st(f3), "A[Obstacle->Driver]".into(), "sign reversal".into(), "reaction phase".into()],
        ["response time".into(), "obs.Driver, obs.Brake, F3".into(), st(f3), "A[Driver->Brake]".into(), "sign reversal".into(), "reaction phase".into()],
        ["build-up time".into(), "obs.Brake, obs.Tread, F3".into(), st(f3), "A[Brake->Tread]".into(), "sign reversal".into(), "build-up phase".into()],
        ["grade per segment".into(), "obs.S00..S16 elevation, F2a".into(), st(el), "A[Sk->Sk+1]".into(), "atan(−A / spacing)".into(), "segment deceleration".into()],
        ["friction per segment".into(), "obs.S00..S15 friction, F2b".into(), st(fr), "none (node value)".into(), "none".into(), "segment deceleration".into()],
        ["station spacing 25 m".into(), "simulator_illustrative.json".into(), "NOT submitted to IRRS (V-03b)".into(), "none".into(), "none".into(), "segment length; grade".into()],
        ["mass, drag area, air density, g".into(), "simulator_illustrative.json".into(), "NOT submitted to IRRS (V-03b)".into(), "none".into(), "none".into(), "drag/m; g·(μcosθ+sinθ)".into()],
    ];
    for row in &rows {
        let _ = writeln!(r, "    {:22} | {:34} | {:22} | {:22} | {:22} | {}", row[0], row[1], row[2], row[3], row[4], row[5]);
    }
    let _ = writeln!(r, "    V-03a: the wind projection is executed by the simulator before submission; the gate's AuthorizedMapping is");
    let _ = writeln!(r, "           identity, so the gate does not itself execute this M. OPEN, returned to Origin.");
    let _ = writeln!(r, "    V-03b: mass, drag area, air density, g and spacing are DECLARED but never pass IRRS evaluation or the gate.");
    let _ = writeln!(r, "           They are also the simulator's own values (shared provenance, recorded not excluded). OPEN.");
    let _ = writeln!(r, "    V-03c: perception, response and build-up times are observed DURING the event (F3), so they are not 'conditions");
    let _ = writeln!(r, "           available before the stopping event'. Whether the expanded calculation counts as a prediction is OPEN.");
    let _ = writeln!(r, "    Every canonical output used is a NON-AUTHORITATIVE DIAGNOSTIC (IRRS OPEN). The Newtonian equations are");
    let _ = writeln!(r, "    explanatory application mathematics, not ABR; the formula awaits Origin authorization (D-SD-4, OPEN).");
    let _ = writeln!(r, "    R and ρ depend on the illustrative ρ_base and χ₀; 𝟙 is a review signal (§11.4). None is used.");

    // ── Side-by-side ──────────────────────────────────────────────────────
    let _ = writeln!(r, "\n================ Exercise C — PREDICTION vs RECONSTRUCTION: COMPARISON [all values diagnostic; IRRS OPEN] ================");
    let reference = run.relational.as_ref().map(|q| q.stopping).unwrap_or(f64::NAN);
    let expanded = run.inputs.as_ref().map(calc::expanded);
    let _ = writeln!(r, "  {:44} {:>14} {:>16}", "", "distance (m)", "comparison: minus reconstruction");
    let _ = writeln!(r, "  {:44} {:>14.3} {:>+16.3}", "PREDICTION, basic (Exercise A inputs)", basic.total_m, basic.total_m - reference);
    match &expanded {
        Some(e) => {
            let _ = writeln!(r, "  {:44} {:>14.3} {:>+16.3}", "PREDICTION, expanded (D-SD-4 OPEN; V-03c)", e.total_m, e.total_m - reference);
            let _ = writeln!(r, "      reaction {:.3} m + build-up {:.3} m + braking {:.3} m", e.reaction_m, e.buildup_m, e.braking_m);
        }
        None => { let _ = writeln!(r, "  Expanded: OPEN — required field(s) not executed: {}", run.missing.join(", ")); }
    }
    let _ = writeln!(r, "  {:44} {:>14.3} {:>16}", "RECONSTRUCTION (−A[Onset->Standstill], F5)", reference, "—");
    let _ = writeln!(r, "      calculated from the observed (simulated) vehicle states. The simulator's own figure is {:.3} m (difference {:+.3e} m):", obs.stopping_distance_m, reference - obs.stopping_distance_m);
    let _ = writeln!(r, "      the same observation, so they agree: a reconstruction check, not an independent prediction.");
    let _ = writeln!(r, "      stopping time {:.3} s; trajectory in records/expanded_event/reference_observation.json", obs.stopping_time_s);
    if let Some(e) = &expanded {
        let (db, de) = ((basic.total_m - reference).abs(), (e.total_m - reference).abs());
        let _ = writeln!(r, "  COMPARISON: |basic − reconstruction| = {db:.3} m, |expanded − reconstruction| = {de:.3} m — the {} prediction is closer here.",
            if de < db { "expanded" } else if db < de { "basic" } else { "two" });
        let _ = writeln!(r, "  The remaining expanded difference includes effects the simulator models and the calculation does not (D-SD-9).");
    }
    let _ = writeln!(r, "  Agreement with a simulator is agreement with a declared model, not with the world.");

    // Perturbation: which relationships materially affect the expanded calculation.
    if let Some(inp) = &run.inputs {
        let base = calc::expanded(inp).total_m;
        let bt = &cfg["base_trial"];
        let _ = writeln!(r, "\n  Which relationships materially changed the expanded result (each reverted alone to the base trial; threshold {MATERIAL_M} m, D-SD-10):");
        let mut cases: Vec<(&str, ExpandedInputs)> = Vec::new();
        let mut c = inp.clone(); c.perception = f(bt, "perception_s"); cases.push(("perception delay (visibility × attention, F3)", c));
        let mut c = inp.clone(); c.wind_along = 0.0; cases.push(("wind (F1 Air)", c));
        let mut c = inp.clone(); for s in &mut c.segments { s.angle = 0.0; } cases.push(("road grade (F2a)", c));
        let mut c = inp.clone(); for s in &mut c.segments { s.friction = f(bt, "friction"); } cases.push(("friction: tread × moisture, patchiness (F2b)", c));
        let mut c = inp.clone(); c.buildup = 0.0; cases.push(("brake build-up (F3)", c));
        let mut c = inp.clone(); c.drag_area = 0.0; cases.push(("aerodynamic drag at all (declared input)", c));
        let _ = writeln!(r, "    {:48} {:>14} {:>14} {}", "reverted", "expanded (m)", "change (m)", "");
        for (name, c) in cases {
            let d = calc::expanded(&c).total_m;
            let _ = writeln!(r, "    {:48} {:>14.3} {:>+14.3} {}", name, d, base - d,
                if (base - d).abs() >= MATERIAL_M { "material" } else { "below threshold" });
        }
        let _ = writeln!(r, "    (mass enters only through drag/mass; with friction-limited braking it otherwise cancels)");
    }
    let _ = writeln!(r, "\n  Unmeasured or unresolved:");
    let _ = writeln!(r, "    - every value: illustrative, simulated; no measurement exists (observational verification OPEN)");
    let _ = writeln!(r, "    - Phase 2 (changing deceleration over the event, brake activation history): not entered; the simulator is not declared");
    let _ = writeln!(r, "      a transducer (§8, D-SD-6), and irrs-session c7410c5 has no persistence execution path");
    let _ = writeln!(r, "    - ρ_base, χ₀: illustrative (R and ρ depend on them); the Onset->Standstill relation awaits Origin review (D-SD-12)");
    let _ = writeln!(r, "    - direction bases of F1/F3 (D-SD-2); projections C (D-SD-3); expanded formula (D-SD-4)");
    let _ = writeln!(r, "    - IRRS evaluation of every field is OPEN (nodes/edges 'proposed', parameters illustrative, Phase 2)");

    // ── §7 Interdependent conditions ──────────────────────────────────────
    let _ = writeln!(r, "\n================ Interdependent observable conditions (four controlled-trial diamonds) ================");
    let _ = writeln!(r, "  Each trial: reference simulation; its seven fields through IRRS and the kernel (stopping distance = −A on F5);");
    let _ = writeln!(r, "  basic and expanded predictions. The four relational stopping distances then form field F4, a diamond of");
    let _ = writeln!(r, "  controlled-condition relations (D-SD-8 OPEN), whose A outputs are the differences between trials.");
    let _ = writeln!(r, "  Every IRRS record here is OPEN: all values are NON-AUTHORITATIVE DIAGNOSTICS.");
    let mut trial_lines = String::new();
    for d in cfg["diamonds"].as_array().unwrap() {
        let id = d["id"].as_str().unwrap();
        let fa = d["factor_a"]["name"].as_str().unwrap();
        let fb = d["factor_b"]["name"].as_str().unwrap();
        let la: Vec<&str> = d["factor_a"]["levels"].as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
        let lb: Vec<&str> = d["factor_b"]["levels"].as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
        let order = [(0, 0), (1, 0), (0, 1), (1, 1)];
        let mut dist = [0.0; 4];
        let _ = writeln!(r, "\n  --- {id}: {fa} × {fb} ---");
        let _ = writeln!(r, "    Source of any interaction: {}", d["_basis"].as_str().unwrap_or("<not declared>"));
        let _ = writeln!(r, "    {:22} {:>12} {:>12} {:>12}  {}", "trial", "basic (m)", "expanded (m)", "reconstr. (m)", "IRRS (all seven fields)");
        for (k, (i, j)) in order.iter().enumerate() {
            let tid = format!("{}_{}", la[*i], lb[*j]);
            let t = trial(&cfg, &tid, &d["trials"][&tid]);
            let o = sim::simulate(&t, &veh, dt);
            let er = run_event(&o, spacing, &veh, None);
            let e = er.inputs.as_ref().map(|x| calc::expanded(x).total_m);
            let rel = er.relational.as_ref().map(|q| q.stopping).unwrap_or(f64::NAN);
            dist[k] = round9(rel);
            let all_open = er.outcomes.values().all(|x| x.irrs_status.starts_with("OPEN") && x.violations == 0);
            let all_exec = er.outcomes.values().all(|x| x.executed);
            let _ = writeln!(r, "    {:22} {:>12.3} {:>12} {:>12.3}  {}", tid, basic.total_m,
                e.map(|x| format!("{x:.3}")).unwrap_or_else(|| "OPEN".into()), rel,
                format!("{}; {}", if all_open { "OPEN, 0 violations" } else { "see record" }, if all_exec { "A→B→R executed" } else { "not all executed" }));
            let _ = writeln!(trial_lines, "{id}/{tid}: relational −A[Onset->Standstill] {:.6} m (simulator {:.6} m); expanded {}; field digests {}", rel, o.stopping_distance_m,
                e.map(|x| format!("{x:.6} m")).unwrap_or_else(|| "OPEN".into()),
                er.outcomes.iter().map(|(s, x)| format!("{s}={}", &x.digest[..16])).collect::<Vec<_>>().join(" "));
        }
        let spec = fields::diamond(id, fa, [la[0], la[1]], fb, [lb[0], lb[1]], dist);
        let json = spec.to_json();
        let out = pipeline::run(&format!("{}.declaration.json", spec.stem), &json);
        save(&format!("diamonds/{}.declaration.json", spec.stem), &json);
        save(&format!("diamonds/{}.report.txt", spec.stem), &out.report);
        if let Some(j) = &out.irrs_json { save(&format!("diamonds/{}.irrs_eval.json", spec.stem), &(j.clone() + "\n")); }
        let _ = writeln!(r, "    F4 diamond: IRRS {}; {}; {}", out.irrs_status, out.admission, if out.executed { "A→B→R executed" } else { &out.calculation });
        if out.executed {
            let n = |i: usize, j: usize| format!("{}_{}", la[i], lb[j]);
            let ea0 = out.a[&format!("{}->{}", n(0, 0), n(1, 0))];
            let ea1 = out.a[&format!("{}->{}", n(0, 1), n(1, 1))];
            let eb0 = out.a[&format!("{}->{}", n(0, 0), n(0, 1))];
            let eb1 = out.a[&format!("{}->{}", n(1, 0), n(1, 1))];
            let _ = writeln!(r, "    Effect of {fa} ({} → {}), as −A (increase in stopping distance):", la[0], la[1]);
            let _ = writeln!(r, "      with {fb} = {:10} −A[{}->{}] = {:+.3} m", lb[0], n(0, 0), n(1, 0), -ea0);
            let _ = writeln!(r, "      with {fb} = {:10} −A[{}->{}] = {:+.3} m", lb[1], n(0, 1), n(1, 1), -ea1);
            let _ = writeln!(r, "    Effect of {fb} ({} → {}):", lb[0], lb[1]);
            let _ = writeln!(r, "      with {fa} = {:10} −A[{}->{}] = {:+.3} m", la[0], n(0, 0), n(0, 1), -eb0);
            let _ = writeln!(r, "      with {fa} = {:10} −A[{}->{}] = {:+.3} m", la[1], n(1, 0), n(1, 1), -eb1);
            let same = (ea0 - ea1).abs() < 1e-6;
            let _ = writeln!(r, "    The two A values for the same {fa} change {} — {}", if same { "are equal" } else { "differ" },
                if same { "here the significance of one condition does not depend on the other" }
                else { "the significance of one condition depends on the other" });
            let _ = writeln!(r, "    (Two admitted A outputs side by side; no interaction formula was applied. B, R on the diamond: in the report.)");
        }
    }
    save("diamonds/trials_summary.txt", &trial_lines);

    // ── Negative-path fixtures (preserved evidence; V-06) ─────────────────
    let _ = writeln!(r, "\n================ Negative-path fixtures (declarations/negative_tests/; reports in records/negative_tests/) ================");
    let neg_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("declarations").join("negative_tests");
    let mut negs: Vec<PathBuf> = std::fs::read_dir(&neg_dir).map(|d| d.filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false)).collect()).unwrap_or_default();
    negs.sort();
    for p in &negs {
        let stem = p.file_stem().unwrap().to_string_lossy().to_string();
        let raw = std::fs::read_to_string(p).expect("read fixture");
        let out = pipeline::run(&format!("{stem}.json"), &raw);
        save(&format!("negative_tests/{stem}.report.txt", ), &format!("{}\nIRRS evaluation: {}\nGate admission: {}\nAuthorized execution: {}\nCalculation: {}\n",
            out.report, out.irrs_status, out.admission, if out.authorized { "AUTHORIZED" } else { "NOT PERFORMED" }, out.calculation));
        let _ = writeln!(r, "  {:32} IRRS {:54} | {}", stem, out.irrs_status, out.calculation);
    }
    let _ = writeln!(r, "  neg4 (directed ring): IRRS raises no violation, and the canonical calls execute A → B → R on a topology Kernel V9");
    let _ = writeln!(r, "  §8 declares inadmissible. Canonical defect, reported to the IRRS/Kernel authority: DEFECT_REPORT_V06_RING_TOPOLOGY.md.");
    let _ = writeln!(r, "  Lesson 1 adds no ring-rejection rule of its own.");

    let _ = writeln!(r, "\n================ Status (DECLARED · CALCULATED · SIMULATED · VERIFIED · OPEN) ================");
    let _ = writeln!(r, "  DECLARED   : fields F1, F2a, F2b, F3, F5 and diamonds F4 (D, M, loci, relations, bases); simulator and calculation");
    let _ = writeln!(r, "               values (declarations/simulator_illustrative.json); ρ_base and χ₀ (illustrative). Declared ≠ correct.");
    let _ = writeln!(r, "  SIMULATED  : every reading and the reference trajectory (src/sim.rs). Not empirical evidence.");
    let _ = writeln!(r, "  CALCULATED : Exercise A (Newtonian, by hand); the expanded prediction (Newtonian application mathematics, not ABR;");
    let _ = writeln!(r, "               D-SD-4 OPEN); canonical A, ρ, B, R, 𝟙 per field as a NON-AUTHORITATIVE DIAGNOSTIC (IRRS OPEN);");
    let _ = writeln!(r, "               Kernel V9 admission::admit of Δ[Onset->Standstill]: ADMITTED, status Simulated (provenance conformance).");
    let _ = writeln!(r, "  VERIFIED   : NOTHING. No outcome has been compared with an actual observation. Compilation, passing tests,");
    let _ = writeln!(r, "               deterministic reproduction, RPPI agreement and V9 admission are conformance, not verification.");
    let _ = writeln!(r, "  OPEN       : IRRS evaluation of every field (authorized execution not performed); Phase 2 persistence (V-04);");
    let _ = writeln!(r, "               V-03a/b/c; ring defect (V-06); D-SD-2, -3, -4, -6, -7, -8, -10, -12; A-3 for B's empty sum;");
    let _ = writeln!(r, "               observational verification.");
    print!("{r}");
    save("LESSON1_REPORT.txt", &r);
    eprintln!("Records written to records/ (LESSON1_REPORT.txt, expanded_event/, diamonds/).");
}
