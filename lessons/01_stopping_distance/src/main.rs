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
    ]
}

struct EventRun {
    outcomes: BTreeMap<String, Outcome>,
    inputs: Option<ExpandedInputs>,
    missing: Vec<String>,
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
        return EventRun { outcomes, inputs: None, missing };
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
    EventRun {
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
        println!("\nIRRS evaluation: {}\nAdmission: {}\nCalculation: {}", out.irrs_status, out.admission, out.calculation);
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
    let _ = writeln!(r, "  {:28} {:44} {:28} {}", "field", "IRRS evaluation", "admission", "calculation");
    for (stem, o) in &run.outcomes {
        let _ = writeln!(r, "  {:28} {:44} {:28} {}", stem, o.irrs_status, o.admission, if o.executed { "CALCULATED (A, ρ, B, R, 𝟙)" } else { &o.calculation });
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
    let _ = writeln!(r, "\n  𝟙[e] = 1.0 (review signal, §11.4) on {signals} of {rels} relations across the five fields.");

    // Relationship between kernel outputs and the calculation.
    let _ = writeln!(r, "\n  How admitted relational outputs enter the stopping-distance calculation (declared projections C, D-SD-3):");
    let _ = writeln!(r, "    initial speed           ← admitted node value Body (F1 onset)");
    let _ = writeln!(r, "    along-road wind         ← admitted node value Air (F1 onset; M projects speed × cos(direction))");
    let _ = writeln!(r, "    relative air speed      ← A[Air->Body] = w − v (reported; the calculation uses v − w at each segment entry)");
    let _ = writeln!(r, "    perception / response / build-up times ← −A on F3 relations (sign reversal)");
    let _ = writeln!(r, "    grade of segment k      ← atan(−A[Sk->Sk+1] / spacing) from F2a (drop ÷ declared spacing)");
    let _ = writeln!(r, "    friction of segment k   ← admitted node value at Sk (F2b)");
    let _ = writeln!(r, "    slip velocity           ← A[Body->Tread] at full braking — reported; no use in the calculation");
    let _ = writeln!(r, "    B, R, ρ, 𝟙              ← NO canonical mapping to a stopping distance: OPEN, not used");
    let _ = writeln!(r, "    mass, drag area, air density, g ← declared calculation inputs (single values, no relation; not kernel inputs)");

    // ── Side-by-side ──────────────────────────────────────────────────────
    let _ = writeln!(r, "\n================ Exercise C — Comparison with the reference (simulated) observation ================");
    let reference = obs.stopping_distance_m;
    let expanded = run.inputs.as_ref().map(calc::expanded);
    let _ = writeln!(r, "  {:44} {:>14} {:>16}", "", "distance (m)", "minus reference");
    let _ = writeln!(r, "  {:44} {:>14.3} {:>+16.3}", "Basic (Exercise A inputs)", basic.total_m, basic.total_m - reference);
    match &expanded {
        Some(e) => {
            let _ = writeln!(r, "  {:44} {:>14.3} {:>+16.3}", "Expanded (pending Origin authorization D-SD-4)", e.total_m, e.total_m - reference);
            let _ = writeln!(r, "      reaction {:.3} m + build-up {:.3} m + braking {:.3} m", e.reaction_m, e.buildup_m, e.braking_m);
        }
        None => { let _ = writeln!(r, "  Expanded: OPEN — required field(s) not executed: {}", run.missing.join(", ")); }
    }
    let _ = writeln!(r, "  {:44} {:>14.3} {:>16}", "Reference (simulated event)", reference, "—");
    let _ = writeln!(r, "      stopping time {:.3} s; trajectory in records/expanded_event/reference_observation.json", obs.stopping_time_s);
    if let Some(e) = &expanded {
        let (db, de) = ((basic.total_m - reference).abs(), (e.total_m - reference).abs());
        let _ = writeln!(r, "  Observed result: |basic − reference| = {db:.3} m, |expanded − reference| = {de:.3} m — the {} calculation is closer here.",
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
    let _ = writeln!(r, "    - ρ_base, χ₀: illustrative; B, R, ρ, 𝟙 have no canonical mapping to distance");
    let _ = writeln!(r, "    - direction bases of F1/F3 (D-SD-2); projections C (D-SD-3); expanded formula (D-SD-4)");
    let _ = writeln!(r, "    - IRRS evaluation of every field is OPEN (nodes/edges 'proposed', parameters illustrative, Phase 2)");

    // ── §7 Interdependent conditions ──────────────────────────────────────
    let _ = writeln!(r, "\n================ Interdependent observable conditions (four controlled-trial diamonds) ================");
    let _ = writeln!(r, "  Each trial: reference simulation; its five fields through IRRS and the kernel; basic and expanded calculations.");
    let _ = writeln!(r, "  Then the four reference distances form field F4 (a diamond of controlled-condition relations, D-SD-8 OPEN).");
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
        let _ = writeln!(r, "    {:22} {:>12} {:>12} {:>12}  {}", "trial", "basic (m)", "expanded (m)", "reference (m)", "IRRS (all five fields)");
        for (k, (i, j)) in order.iter().enumerate() {
            let tid = format!("{}_{}", la[*i], lb[*j]);
            let t = trial(&cfg, &tid, &d["trials"][&tid]);
            let o = sim::simulate(&t, &veh, dt);
            let er = run_event(&o, spacing, &veh, None);
            let e = er.inputs.as_ref().map(|x| calc::expanded(x).total_m);
            dist[k] = round9(o.stopping_distance_m);
            let all_open = er.outcomes.values().all(|x| x.irrs_status.starts_with("OPEN") && x.violations == 0);
            let all_exec = er.outcomes.values().all(|x| x.executed);
            let _ = writeln!(r, "    {:22} {:>12.3} {:>12} {:>12.3}  {}", tid, basic.total_m,
                e.map(|x| format!("{x:.3}")).unwrap_or_else(|| "OPEN".into()), o.stopping_distance_m,
                format!("{}; {}", if all_open { "OPEN, 0 violations" } else { "see record" }, if all_exec { "A→B→R executed" } else { "not all executed" }));
            let _ = writeln!(trial_lines, "{id}/{tid}: reference {:.6} m; expanded {}; field digests {}", o.stopping_distance_m,
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

    let _ = writeln!(r, "\n================ Status ================");
    let _ = writeln!(r, "  Exercise A                 : CALCULATED (Newtonian, by hand; not kernel)");
    let _ = writeln!(r, "  IRRS declaration evaluation: OPEN for every field (0 violations) — proposed support, illustrative parameters, Phase 2");
    let _ = writeln!(r, "  IRRS observation admission : see the table above (simulated readings, admitted only as simulated)");
    let _ = writeln!(r, "  Canonical calculation      : A, ρ, B, R, 𝟙 per field (irrs_session::execute; RPPI re-execution agreed)");
    let _ = writeln!(r, "  Expanded stopping distance : CALCULATED by a non-kernel Newtonian formula PENDING Origin authorization (D-SD-4)");
    let _ = writeln!(r, "  V9 provenance admission    : OPEN — A → B → R outside abr-kernel 1c1ef70 admission scope");
    let _ = writeln!(r, "  Observational verification : OPEN — the reference is simulated; no comparison with a measurement exists");
    print!("{r}");
    save("LESSON1_REPORT.txt", &r);
    eprintln!("Records written to records/ (LESSON1_REPORT.txt, expanded_event/, diamonds/).");
}
