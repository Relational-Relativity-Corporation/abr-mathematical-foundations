//! ABR Mathematical Foundations — Lesson 1 (revision 2): a complete bounded
//! observable set.
//!
//! Observation → Declaration → IRRS declaration evaluation → IRRS observation
//! admission → canonical A → B → R (IRRS execution) → observable outcome →
//! comparison.
//!
//! This program implements NO operator, NO admission rule and NO comparison
//! rule. Every mathematical and admission step is a call into a pinned
//! canonical crate (abr-irrs c7410c5):
//!
//!   IRRS declaration evaluation  irrs_eval::Evaluator::evaluate
//!                                (irrs-eval/src/evaluator.rs; Stage 2 ABR topology admission)
//!   IRRS observation admission   relational_gate::RegionBuilder::establish,
//!                                relational_gate::AdmissionGate::submit
//!                                (relational-gate/src/region.rs, gate.rs)
//!   Canonical execution          irrs_session::execute::execute
//!                                (irrs-session/src/execute.rs, abr_exec.rs), which calls
//!                                operator_a, compute_rho, operator_b, operator_r and
//!                                detection_indicator of the IRRS copy of Kernel V9
//!                                (irrs-kernel, byte for byte with abr-kernel 1c1ef70),
//!                                then re-executes them independently (RPPI integrity)
//!   Structural check             relational_isolation (irrs-kernel src/operators.rs)
//!
//! What this program does itself: read the student's JSON declaration (the
//! irrs-eval declaration schema, plus a "lesson" block and a "class" field on
//! each edge), look up declared names, pass gate-admitted values and declared
//! parameters to the canonical calls, print what they return, and record the
//! student's own comparison statements. It never computes a difference, an
//! accumulation, a ρ value, an agreement or a status from numbers.
//!
//! Usage:  cargo run -- <declaration .json>

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use serde_json::Value;
use sha2::{Digest, Sha256};

use irrs_eval::{Declaration as IrrsDeclarationDoc, EvaluationStatus, Evaluator};
use irrs_kernel::operators::{relational_isolation, DeclaredRelations, EdgeProvenance, NodeField};
use irrs_session::execute::{execute, IrrsDeclaration, IrrsError};
use relational_gate::{
    AdmissionGate, AdmissionOutcome as GateOutcome, AuthorizedMapping, CrossFamilyOrientation,
    DeclarationPackage, DeclaredEdge, Observable, ObservableExtent, ProvenanceChain, RegionBuilder,
    RegionalAdmissionRule, TopologySpec,
};

/// The lock file Cargo resolved for this build: the pinned canonical revisions
/// are read from it, not restated by hand.
const CARGO_LOCK: &str = include_str!("../../Cargo.lock");

const CANONICAL_PACKAGES: &[&str] =
    &["abr-kernel", "relational-gate", "irrs-eval", "irrs-session", "irrs-kernel"];

// ── Small helpers ─────────────────────────────────────────────────────────

/// relational-gate takes `&'static str` labels. Declared text is leaked once
/// for the life of this short program.
fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

fn text<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty())
}

/// "P1 -> P2" → ("P1", "P2").
fn parse_edge(s: &str) -> Option<(String, String)> {
    s.split_once("->").map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
}

fn locked_sources() -> Vec<String> {
    let mut out = Vec::new();
    let mut name = String::new();
    for line in CARGO_LOCK.lines() {
        if let Some(n) = line.strip_prefix("name = ") {
            name = n.trim_matches('"').to_string();
        }
        if let Some(src) = line.strip_prefix("source = ") {
            if CANONICAL_PACKAGES.contains(&name.as_str()) {
                out.push(format!("{name}: {}", src.trim_matches('"')));
            }
        }
    }
    out
}

/// One raw reading, as the student recorded it.
struct Reading {
    id: String,
    value: f64,
}

impl Observable for Reading {
    fn source_id(&self) -> &str { &self.id }
    fn primary_value(&self) -> f64 { self.value }
}

struct Status {
    declaration: String,
    irrs_evaluation: String,
    irrs_admission: String,
    kernel_selection: String,
    calculation: String,
    v9_admission: String,
    observation: String,
    primary_region: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: cargo run -- <declaration .json>\n\
                   e.g.   cargo run -- declarations/set_1_ramp_illustrative.json");
        std::process::exit(2);
    }
    let path = Path::new(&args[1]);
    let raw = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("INPUT ERROR (nothing evaluated): could not read '{}': {e}", path.display());
            std::process::exit(2);
        }
    };

    let mut r = String::new();
    let mut irrs_record_json: Option<String> = None;
    let status = run(path, &raw, &mut r, &mut irrs_record_json);

    let _ = writeln!(r, "\n== Status summary ==");
    let _ = writeln!(r, "  Code execution             : completed (this report was produced)");
    let _ = writeln!(r, "  Declaration                : {}", status.declaration);
    let _ = writeln!(r, "  IRRS declaration evaluation: {}", status.irrs_evaluation);
    let _ = writeln!(r, "  IRRS observation admission : {}", status.irrs_admission);
    let _ = writeln!(r, "  Kernel selection           : {}", status.kernel_selection);
    let _ = writeln!(r, "  Canonical calculation      : {}", status.calculation);
    let _ = writeln!(r, "  V9 provenance admission    : {}", status.v9_admission);
    let _ = writeln!(r, "  Observational verification : {}", status.observation);
    let _ = writeln!(r, "  Primary Region             : {}", status.primary_region);
    let _ = writeln!(r, "\nThese statuses are not interchangeable. Execution, admission and calculation are");
    let _ = writeln!(r, "conformance statements; only the verification line concerns agreement with");
    let _ = writeln!(r, "observation (kernel_reference.md §16.3, §17).");

    print!("{r}");

    let stem = path.file_stem().map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "declaration".into());
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("records");
    let out = dir.join(format!("{stem}.report.txt"));
    match std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&out, &r)) {
        Ok(()) => eprintln!("Report preserved: {}", out.display()),
        Err(e) => eprintln!("WARNING: report not preserved ({e})"),
    }
    if let Some(j) = irrs_record_json {
        let out = dir.join(format!("{stem}.irrs_eval.json"));
        match std::fs::write(&out, j) {
            Ok(()) => eprintln!("IRRS evaluation record preserved: {}", out.display()),
            Err(e) => eprintln!("WARNING: IRRS record not preserved ({e})"),
        }
    }
}

fn run(path: &Path, raw: &str, r: &mut String, irrs_json: &mut Option<String>) -> Status {
    let mut st = Status {
        declaration: "OPEN".into(),
        irrs_evaluation: "not reached".into(),
        irrs_admission: "not reached".into(),
        kernel_selection: "not reached".into(),
        calculation: "OPEN — not evaluated".into(),
        v9_admission: "not reached".into(),
        observation: "OPEN".into(),
        primary_region: "OPEN — not observed; not reconstructed (05_TOWARD_THE_PRIMARY_REGION.md)".into(),
    };

    let _ = writeln!(r, "ABR Mathematical Foundations — Lesson 1: a complete bounded observable set");
    let _ = writeln!(r, "Declaration file: {}", path.display());
    let digest = hex::encode(Sha256::digest(raw.as_bytes()));
    let _ = writeln!(r, "Declaration SHA-256 (raw text, as irrs-eval computes it): {digest}");
    let _ = writeln!(r, "\n== Canonical authorities (resolved by Cargo.lock) ==");
    for s in locked_sources() {
        let _ = writeln!(r, "  {s}");
    }

    // ── Parse: the same text is read by IRRS (its schema) and by the lesson ──
    let doc: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => {
            st.declaration = format!("OPEN — the file is not valid JSON: {e}");
            let _ = writeln!(r, "\n  {}", st.declaration);
            return st;
        }
    };
    let irrs_doc: IrrsDeclarationDoc = match serde_json::from_str(raw) {
        Ok(d) => d,
        Err(e) => {
            st.declaration = format!("OPEN — the IRRS declaration schema (irrs-eval Declaration) could not read the file: {e}");
            let _ = writeln!(r, "\n  {}", st.declaration);
            return st;
        }
    };
    let Some(lesson) = doc.get("lesson") else {
        st.declaration = "OPEN — no \"lesson\" block declared (readings, kernel selection, tolerance)".into();
        let _ = writeln!(r, "\n  {}", st.declaration);
        return st;
    };

    // ── Step 1: Declaration (Origin: the student) ─────────────────────────
    let _ = writeln!(r, "\n== Step 1 — Declaration (Origin: the student) ==");
    let observation_status = text(lesson, "observation_status").unwrap_or("<not declared>");
    let _ = writeln!(r, "  Observation status : {}", match observation_status {
        "illustrative" => "ILLUSTRATIVE — supplied demonstration values, NOT an observation",
        "measured" => "MEASURED — the student declares these readings were independently observed",
        _ => "<not declared: must be 'illustrative' or 'measured'>",
    });
    let rp = &irrs_doc.regional_prerequisite;
    let unit = rp.extent_unit.trim().to_string();
    let bound = |b: Option<f64>| b.map(|v| format!("{v:?}")).unwrap_or_else(|| "<not declared>".into());
    let _ = writeln!(r, "  D (observable extent): [{}, {}] {unit} inclusive — region '{}'",
        bound(rp.extent_floor), bound(rp.extent_ceiling), rp.region_name);
    let m_kind = text(lesson, "m_kind").unwrap_or("<not declared>");
    let mapping = irrs_doc.mappings.first();
    let _ = writeln!(r, "  M: kind {m_kind}; mapping '{}'", mapping.map(|m| m.name.as_str()).unwrap_or("<none declared>"));
    let _ = writeln!(r, "     preserves: {}", rp.orientation_preserved);
    let _ = writeln!(r, "     discards : {}", rp.orientation_discarded);

    let Some(topo) = irrs_doc.abr_topology.as_ref() else {
        st.declaration = "OPEN — no abr_topology declared (loci and relations)".into();
        let _ = writeln!(r, "  {}", st.declaration);
        return st;
    };
    let readings = lesson.get("readings").and_then(Value::as_object);
    let _ = writeln!(r, "  Loci (node index = list position: a label only, NOT topology):");
    for (i, n) in topo.nodes.iter().enumerate() {
        let v = readings.and_then(|m| m.get(&n.observable)).and_then(Value::as_f64);
        let _ = writeln!(r, "    [{i}] {:4} ← {:8} reading {} {unit}", n.id, n.observable,
            v.map(|x| format!("{x:?}")).unwrap_or_else(|| "<not declared>".into()));
    }
    let edge_values: Vec<&Value> = doc.pointer("/abr_topology/edges").and_then(Value::as_array)
        .map(|a| a.iter().collect()).unwrap_or_default();
    let _ = writeln!(r, "  Declared relations (each with its own observational basis):");
    for (i, e) in topo.edges.iter().enumerate() {
        let class = edge_values.get(i).and_then(|v| text(v, "class")).unwrap_or("<not declared>");
        let _ = writeln!(r, "    e{i}: {} → {}  class {class}", e.from_node, e.to_node);
        let _ = writeln!(r, "        basis: {}", e.description);
    }
    let interior: Vec<&Value> = lesson.get("interior_loci").and_then(Value::as_array)
        .map(|a| a.iter().collect()).unwrap_or_default();
    let _ = writeln!(r, "  Interior loci declared by M (kernel_reference.md §10):");
    if interior.is_empty() {
        let _ = writeln!(r, "    none declared");
    }
    for il in &interior {
        let _ = writeln!(r, "    {} — incoming {}, continuing {}",
            text(il, "node").unwrap_or("<?>"), text(il, "incoming").unwrap_or("<?>"),
            text(il, "continuing").unwrap_or("<?>"));
        let _ = writeln!(r, "        basis: {}", text(il, "basis").unwrap_or("<not declared>"));
    }
    let params = topo.operator_parameters.as_ref();
    let rho_base = params.and_then(|p| p.rho_base.as_ref());
    let chi_0 = params.and_then(|p| p.chi_0.as_ref());
    let tolerance = lesson.get("detection_tolerance").and_then(Value::as_f64);
    let _ = writeln!(r, "  Operator parameters declared by M (kernel_reference.md §9; no kernel defaults):");
    let _ = writeln!(r, "    ρ_base = {}", rho_base.map(|p| format!("{:?} ({:?})", p.value, p.provenance)).unwrap_or_else(|| "<not declared>".into()));
    let _ = writeln!(r, "    χ₀     = {}", chi_0.map(|p| format!("{:?} {unit} ({:?})", p.value, p.provenance)).unwrap_or_else(|| "<not declared>".into()));
    let _ = writeln!(r, "    component pairs / cc: none declared (one component per locus)");
    let _ = writeln!(r, "    detection tolerance for 𝟙[e] (implementation-only, §11.5) = {}",
        tolerance.map(|t| format!("{t:?} {unit}")).unwrap_or_else(|| "<not declared>".into()));
    st.declaration = "DECLARED (completeness is checked below; IRRS decides admissibility)".into();

    // ── Step 2: IRRS declaration evaluation (irrs-eval) ───────────────────
    let _ = writeln!(r, "\n== Step 2 — IRRS declaration evaluation ==");
    let _ = writeln!(r, "  Called: irrs_eval::Evaluator::evaluate(declaration, sha256) (abr-irrs c7410c5, irrs-eval/src/evaluator.rs)");
    let record = Evaluator::evaluate(irrs_doc.clone(), digest.clone());
    *irrs_json = serde_json::to_string_pretty(&record).ok();
    let _ = writeln!(r, "  Record {} — status {:?}", record.record_id, record.status);
    if let Some(t) = &record.abr_topology {
        let s = &t.summary;
        let _ = writeln!(r, "  ABR topology: nodes {} (observed {}, proposed {}, missing {}, contradicted {});",
            s.total_nodes, s.observed_nodes, s.proposed_nodes, s.missing_nodes, s.contradicted_nodes);
        let _ = writeln!(r, "                edges {} (observed {}, proposed {}, missing {}, contradicted {});",
            s.total_edges, s.observed_edges, s.proposed_edges, s.missing_edges, s.contradicted_edges);
        let _ = writeln!(r, "                interior locus {}; parameters {} (observational {}, illustrative {})",
            s.interior_locus_status, s.total_parameters, s.observational_parameters, s.illustrative_parameters);
    }
    let _ = writeln!(r, "  Open conditions returned by IRRS ({}):", record.open_conditions.len());
    for c in &record.open_conditions {
        let _ = writeln!(r, "    OPEN: {c}");
    }
    for v in &record.violations {
        let _ = writeln!(r, "    VIOLATION: {v}");
    }
    match record.status {
        EvaluationStatus::Rejected => {
            st.irrs_evaluation = format!("REJECTED — {}", record.failure_reason.clone().unwrap_or_default());
            let _ = writeln!(r, "  A contradiction is established. Nothing is executed.");
            return st;
        }
        EvaluationStatus::Open => {
            st.irrs_evaluation = format!("OPEN — {} condition(s), 0 violations; not authorized (records/<file>.irrs_eval.json)",
                record.open_conditions.len());
        }
        ref other => {
            st.irrs_evaluation = format!("{other:?}");
        }
    }

    // ── Step 3: IRRS observation admission (relational-gate) ──────────────
    let _ = writeln!(r, "\n== Step 3 — IRRS observation admission ==");
    let _ = writeln!(r, "  Called: relational_gate::RegionBuilder::establish, relational_gate::AdmissionGate::submit");
    let _ = writeln!(r, "          (abr-irrs c7410c5: relational-gate/src/region.rs, relational-gate/src/gate.rs)");
    let (Some(floor), Some(ceiling)) = (rp.extent_floor, rp.extent_ceiling) else {
        st.irrs_admission = "OPEN — observable extent not declared".into();
        let _ = writeln!(r, "  {}", st.irrs_admission);
        return st;
    };
    let Some(mapping) = mapping else {
        st.irrs_admission = "OPEN — no authorized mapping declared".into();
        let _ = writeln!(r, "  {}", st.irrs_admission);
        return st;
    };
    if m_kind != "identity" {
        st.irrs_admission = format!("OPEN — this lesson supplies the gate only an Identity M; declared kind '{m_kind}' is not supported here");
        let _ = writeln!(r, "  {}", st.irrs_admission);
        return st;
    }
    let extent = match ObservableExtent::new(floor, ceiling, leak(&unit)) {
        Ok(e) => e,
        Err(e) => {
            st.irrs_admission = format!("{e}");
            let _ = writeln!(r, "  {e}");
            return st;
        }
    };
    let mut builder = RegionBuilder::new()
        .name(leak(&rp.region_name))
        .extent(extent)
        .admission_rule(RegionalAdmissionRule {
            name: leak(&rp.admission_rule_name),
            description: leak(&rp.admission_rule_description),
        });
    if !rp.orientation_preserved.trim().is_empty() && !rp.orientation_discarded.trim().is_empty() {
        builder = builder.orientation(CrossFamilyOrientation {
            preserved: leak(&rp.orientation_preserved),
            discarded: leak(&rp.orientation_discarded),
        });
    }
    let Some(observed_locus) = rp.observed_locus else {
        st.irrs_admission = "OPEN — no observed locus declared as evidence for the region".into();
        let _ = writeln!(r, "  {}", st.irrs_admission);
        return st;
    };
    let region = match builder.establish(observed_locus) {
        Ok(reg) => reg,
        Err(e) => {
            st.irrs_admission = format!("{e}");
            let _ = writeln!(r, "  Region not established — {e}");
            return st;
        }
    };
    let _ = writeln!(r, "  Region '{}' established; extent [{:?}, {:?}] {}; evidence reading {:?} {unit}",
        region.name, region.extent.floor, region.extent.ceiling, region.extent.unit, region.observed_locus);
    let m_name = leak(&mapping.name);
    let gate_edges: Vec<DeclaredEdge> = topo.edges.iter().map(|e| DeclaredEdge {
        from_family: leak(&e.from_node),
        to_family: leak(&e.to_node),
        provenance: leak(&e.description),
        mapping_name: m_name,
    }).collect();
    let topology_id = leak(&irrs_doc.topology_id);
    let gate: AdmissionGate<Reading, f64> = AdmissionGate::new(
        TopologySpec::new(topology_id, region, gate_edges),
        vec![AuthorizedMapping::new(
            m_name,
            "Declared M, kind Identity: the reading enters D unchanged in the declared unit",
            |primary, _reference| Ok(primary),
            |v: &f64| *v,
        )],
    );
    let mut admitted: BTreeMap<String, f64> = BTreeMap::new();
    for n in &topo.nodes {
        let Some(value) = readings.and_then(|m| m.get(&n.observable)).and_then(Value::as_f64) else {
            let _ = writeln!(r, "  {}: OPEN — no reading declared for {}", n.id, n.observable);
            continue;
        };
        let package = DeclarationPackage {
            source: Reading { id: n.observable.clone(), value },
            mapping_name: m_name,
            topology_id,
            provenance: ProvenanceChain::new()
                .link("read_height", "The student reads the height of the mark above the table on the ruler scale")
                .link("record", "The reading is written into the declaration file")
                .link("apply_declared_M", "The gate executes the declared identity M"),
        };
        match gate.submit(package) {
            GateOutcome::Admitted { observation, trace } => {
                let p = &trace.provenance;
                let _ = writeln!(r, "  {}: ADMITTED — source {}, mapping {}, raw input {:?}, gate-computed {:?} {unit}",
                    n.id, p.source_id, p.mapping_applied, p.primary_input, p.gate_computed_result);
                admitted.insert(n.id.clone(), observation.gate_result());
            }
            GateOutcome::Divergent { divergence, .. } => {
                let _ = writeln!(r, "  {}: DIVERGENT — {} (preserved; not entered)", n.id, divergence.description);
            }
            GateOutcome::Open { condition, .. } => { let _ = writeln!(r, "  {}: {condition}", n.id); }
            GateOutcome::Failed { error, .. } => { let _ = writeln!(r, "  {}: FAILED — {error}", n.id); }
            GateOutcome::Rejected { violation, .. } => { let _ = writeln!(r, "  {}: {violation}", n.id); }
        }
    }
    if admitted.len() != topo.nodes.len() || topo.nodes.is_empty() {
        st.irrs_admission = "OPEN — not every reading was admitted (see Step 3)".into();
        let _ = writeln!(r, "  Only gate-admitted observations may enter the kernel. Stopping here.");
        return st;
    }
    st.irrs_admission = format!("ADMITTED — all {} readings (AdmittedObservation<f64>, identity M, inside declared extent)",
        admitted.len());

    // ── Step 4: Kernel selection (declared by M, §10) ─────────────────────
    let _ = writeln!(r, "\n== Step 4 — Kernel selection (kernel_reference.md §10) ==");
    let declared_edges: Vec<(String, String)> =
        topo.edges.iter().map(|e| (e.from_node.clone(), e.to_node.clone())).collect();
    let mut established: Vec<String> = Vec::new();
    for il in &interior {
        let node = text(il, "node").unwrap_or("");
        let inc = text(il, "incoming").and_then(parse_edge);
        let cont = text(il, "continuing").and_then(parse_edge);
        // A lookup of the declared statement in the declared edge list: the
        // incoming edge must be declared and end at the node; the continuing
        // edge must be declared and start at it. Nothing is inferred.
        let ok = match (&inc, &cont) {
            (Some(i), Some(c)) => declared_edges.contains(i) && declared_edges.contains(c)
                && i.1 == node && c.0 == node,
            _ => false,
        };
        if ok {
            let _ = writeln!(r, "  {node}: declared incoming and continuing relations are both in the declared edge list");
            established.push(node.to_string());
        } else {
            let _ = writeln!(r, "  {node}: the declared incoming/continuing relations are not both declared edges at this node — not established");
        }
    }
    let selection = text(lesson, "kernel_selection").unwrap_or("<not declared>");
    let _ = writeln!(r, "  Declared kernel selection: {selection} — basis: {}",
        text(lesson, "kernel_selection_basis").unwrap_or("<not declared>"));
    if established.is_empty() {
        st.kernel_selection = "Full-operator admissibility condition not established by this declaration (no interior \
            locus declared, or none consistent with the declared edges); under §10 the Primary kernel Δ → Σ applies. \
            This program does not execute the Primary kernel; see the supporting exercise (cargo run --bin lesson01)".into();
        let _ = writeln!(r, "  {}", st.kernel_selection);
        st.calculation = "OPEN — not executed here (Primary condition)".into();
        st.primary_region = "Primary condition declared (no interior locus); not an observation of the Primary Region".into();
        return st;
    }
    if selection != "abr" {
        st.kernel_selection = format!("REJECTED — interior loci {established:?} are established, so A → B → R applies; \
            declared selection '{selection}' contradicts the declared structural condition");
        let _ = writeln!(r, "  {}", st.kernel_selection);
        return st;
    }
    st.kernel_selection = format!("ABR kernel A → B → R — interior loci established by M: {}", established.join(", "));
    let _ = writeln!(r, "  {}", st.kernel_selection);
    let _ = writeln!(r, "  R is not optional once the condition holds (§10).");

    // ── Step 5: Declaration completeness for A → B → R (§5 Step 2) ────────
    let _ = writeln!(r, "\n== Step 5 — Completeness of the declaration for A → B → R ==");
    let mut missing = Vec::new();
    if rho_base.is_none() { missing.push("ρ_base (abr_topology.operator_parameters.rho_base)"); }
    if chi_0.is_none() { missing.push("χ₀ (abr_topology.operator_parameters.chi_0)"); }
    if tolerance.is_none() { missing.push("detection tolerance for 𝟙[e] (lesson.detection_tolerance)"); }
    let mut classes = Vec::new();
    for (i, e) in topo.edges.iter().enumerate() {
        match edge_values.get(i).and_then(|v| text(v, "class")) {
            Some("continuation") => classes.push(EdgeProvenance::Continuation),
            Some("coupling") => classes.push(EdgeProvenance::Coupling),
            Some(other) => {
                let _ = writeln!(r, "  e{i} ({} → {}): class '{other}' is not a spatial edge provenance \
                    (Continuation or Coupling; Persistence is not declared in the spatial edge list)", e.from_node, e.to_node);
                missing.push("a spatial provenance class for every edge");
            }
            None => missing.push("a provenance class for every edge (a relation with no declared provenance is fabricated within D)"),
        }
    }
    if !missing.is_empty() {
        missing.dedup();
        st.calculation = format!("OPEN — not executed. Missing declaration(s): {}", missing.join("; "));
        let _ = writeln!(r, "  STOP at the missing declaration(s), returned to Origin. Nothing is substituted:");
        for m in &missing {
            let _ = writeln!(r, "    not declared: {m}");
        }
        let _ = writeln!(r, "  A → B → R is not executed in part: R is not optional (§10), and without ρ_base and χ₀");
        let _ = writeln!(r, "  'the operator is not evaluated and there is no operator result' (§9).");
        return st;
    }
    let _ = writeln!(r, "  Every input the A → B → R path uses is declared.");

    // ── Step 6: Canonical execution (irrs-session → Kernel V9) ────────────
    let _ = writeln!(r, "\n== Step 6 — Canonical execution ==");
    let _ = writeln!(r, "  Called: irrs_session::execute::execute(&IrrsDeclaration) (abr-irrs c7410c5, irrs-session/src/execute.rs)");
    let _ = writeln!(r, "          → operator_a, compute_rho, operator_b, operator_r, detection_indicator");
    let _ = writeln!(r, "          (irrs-kernel src/operators.rs, src/derived_invariants.rs), then an independent re-execution");
    let index: BTreeMap<&str, usize> = topo.nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();
    let edges_idx: Vec<(usize, usize)> = topo.edges.iter()
        .map(|e| (index[e.from_node.as_str()], index[e.to_node.as_str()])).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(topo.nodes.len(), edges_idx.clone(), classes.clone());
    let field = NodeField::new(vec![topo.nodes.iter().map(|n| admitted[&n.id]).collect()]);
    let labels: Vec<String> = topo.edges.iter().map(|e| format!("{}->{}", e.from_node, e.to_node)).collect();
    let _ = writeln!(r, "  Inputs: NodeField x[0] = {:?} {unit} (gate-admitted values, node order above)", field.data[0]);
    let _ = writeln!(r, "          DeclaredRelations edges {:?}, provenance {:?}", edges_idx, classes);
    let _ = writeln!(r, "          ρ_base = {:?}, χ₀ = {:?} {unit}, pairs = [], cc = [], tolerance = {:?} {unit}",
        rho_base.unwrap().value, chi_0.unwrap().value, tolerance.unwrap());
    let isolated = relational_isolation(&rel);
    let _ = writeln!(r, "  Structural check relational_isolation(rel) = {isolated}");

    let decl = IrrsDeclaration {
        topology: rel.clone(),
        field,
        rho_base: rho_base.unwrap().value,
        chi_0: chi_0.unwrap().value,
        cc: vec![],
        pairs: vec![],
        tolerance: tolerance.unwrap(),
    };
    let out = match execute(&decl) {
        Ok(o) => o,
        Err(e) => {
            let msg = match &e {
                IrrsError::VerificationFailed(v) => format!("REJECTED — RPPI re-execution diverged: {v:?}"),
                other => format!("OPEN — structural invalidity returned by IRRS: {other:?}"),
            };
            st.calculation = msg.clone();
            let _ = writeln!(r, "  NOT EXECUTED — {msg}");
            return st;
        }
    };
    let _ = writeln!(r, "  Executed. Independent RPPI re-execution (verify_abr_irrs): agreed on every R and 𝟙 output.");
    let _ = writeln!(r, "\n  Edge outputs (component 0, unit {unit}; 𝟙 is 0.0 or 1.0):");
    let _ = writeln!(r, "    {:10} {:>10} {:>10} {:>12} {:>5}", "relation", "A", "B", "R", "𝟙");
    let mut calculated: BTreeMap<String, String> = BTreeMap::new();
    for (e, l) in labels.iter().enumerate() {
        let (a, b, rr, ind) = (out.a.spatial[0][e], out.b.spatial[0][e], out.r.spatial[0][e], out.detection_indicators[0][e]);
        let _ = writeln!(r, "    {:10} {:>10} {:>10} {:>12} {:>5}", l, format!("{a:?}"), format!("{b:?}"), format!("{rr:?}"), format!("{ind:?}"));
        calculated.insert(format!("A[{l}]"), format!("{a:?} {unit}"));
        calculated.insert(format!("B[{l}]"), format!("{b:?} {unit}"));
        calculated.insert(format!("R[{l}]"), format!("{rr:?} {unit}"));
        calculated.insert(format!("1[{l}]"), format!("{ind:?}"));
    }
    let _ = writeln!(r, "  Node outputs (ρ is dimensionless):");
    for (i, n) in topo.nodes.iter().enumerate() {
        let succ: Vec<String> = rel.out[i].iter().map(|&f| labels[f].clone()).collect();
        let pred: Vec<String> = rel.inc[i].iter().map(|&f| labels[f].clone()).collect();
        let _ = writeln!(r, "    ρ[{}] = {:?}   (declared incoming {:?}, outgoing {:?})", n.id, out.rho[i], pred, succ);
        calculated.insert(format!("rho[{}]", n.id), format!("{:?}", out.rho[i]));
    }
    let _ = writeln!(r, "  Kernel V9 open conditions surfaced by IRRS with every R evaluation:");
    for k in &out.kernel_open {
        let _ = writeln!(r, "    {}", k.describe());
    }
    let any_signal = out.detection_indicators[0].iter().any(|&v| v != 0.0);
    if any_signal {
        let _ = writeln!(r, "  𝟙[e] = 1.0 on at least one relation: antisymmetric expression in R is nonzero there.");
        let _ = writeln!(r, "  That is a REVIEW SIGNAL (§11.4), preserved exactly; not a failure, discovery or confirmation.");
    }
    let illustrative_params = matches!(rho_base.unwrap().provenance, irrs_eval::ParameterProvenanceKind::Illustrative)
        || matches!(chi_0.unwrap().provenance, irrs_eval::ParameterProvenanceKind::Illustrative);
    st.calculation = format!("CALCULATED — A, ρ, B, R and 𝟙 over {} relations and {} loci (irrs_session::execute; \
        RPPI re-execution agreed). {}{}",
        labels.len(), topo.nodes.len(),
        if observation_status == "measured" { "Readings declared measured. " } else { "Readings illustrative. " },
        if illustrative_params { "ρ_base and χ₀ illustrative: ρ and R are calculated results of illustrative parameters, not properties of the ramp" }
        else { "ρ_base and χ₀ declared; IRRS reports their observational support in Step 2" });

    // ── Step 7: V9 provenance admission ───────────────────────────────────
    let _ = writeln!(r, "\n== Step 7 — V9 provenance admission ==");
    st.v9_admission = "OPEN — not available for A → B → R. abr-kernel src/admission.rs, supported scope: \
        'OPEN: ABR kernel A → B → R'; reconstruct_primary returns UnsupportedScope for KernelSelection::Abr. \
        The IRRS RPPI re-execution in Step 6 is integrity, not V9 admission".into();
    let _ = writeln!(r, "  {}", st.v9_admission);

    // ── Step 8: Comparison with observation ───────────────────────────────
    let _ = writeln!(r, "\n== Step 8 — Comparison with observation (the student's record) ==");
    let comparisons: Vec<&Value> = lesson.get("comparisons").and_then(Value::as_array)
        .map(|a| a.iter().collect()).unwrap_or_default();
    let keys = ["direct_observation", "uncertainty", "method", "outcome"];
    let mut verified = 0;
    let mut divergent = 0;
    for c in &comparisons {
        let q = text(c, "quantity").unwrap_or("<no quantity>");
        let calc = calculated.get(q).cloned();
        let _ = writeln!(r, "  {q}: calculated {}", calc.clone().unwrap_or_else(|| "<no calculated quantity of this name>".into()));
        for k in keys {
            let _ = writeln!(r, "    {k:18} = {}", text(c, k).unwrap_or("<not declared>"));
        }
        let complete = keys.iter().all(|k| text(c, k).is_some());
        let line = match (observation_status, text(c, "outcome"), complete, calc.is_some()) {
            (_, _, _, false) => "OPEN — no calculated quantity of that name".to_string(),
            ("measured", Some("agrees"), true, true) => { verified += 1;
                "VERIFIED — by the student's recorded comparison with an independent measured observation (see Q-5)".into() }
            ("measured", Some("disagrees"), true, true) => { divergent += 1;
                "DIVERGENT — conformant calculation, divergent from observation (§16.3); preserved for Origin".into() }
            ("measured", _, _, true) => "OPEN — measured readings, comparison not fully declared".into(),
            _ => "OPEN — DEMONSTRATION ONLY (illustrative readings cannot be verified)".into(),
        };
        let _ = writeln!(r, "    status: {line}");
    }
    if comparisons.is_empty() {
        let _ = writeln!(r, "  No comparison declared.");
    }
    let _ = writeln!(r, "  The program computes no agreement. Each outcome is the student's own recorded judgement.");
    let _ = writeln!(r, "  Calculated quantities with no comparison declared remain OPEN.");
    st.observation = if observation_status != "measured" {
        "OPEN — DEMONSTRATION ONLY (illustrative readings); nothing is observationally verified".into()
    } else {
        format!("{verified} VERIFIED, {divergent} DIVERGENT by the student's recorded comparisons; every other \
            calculated quantity OPEN ({} calculated)", calculated.len())
    };
    st.primary_region = "OPEN — not observed and not reconstructed. Every locus here is resolved by M, so this set \
        is not at the Primary condition (05_TOWARD_THE_PRIMARY_REGION.md)".into();
    st
}
