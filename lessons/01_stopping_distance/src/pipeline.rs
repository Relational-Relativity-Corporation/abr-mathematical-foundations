//! Declaration → IRRS declaration evaluation → IRRS observation admission →
//! kernel selection → completeness → canonical A → B → R (IRRS execution).
//!
//! This module implements NO operator, NO admission rule and NO comparison
//! rule. Every mathematical and admission step is a call into a pinned
//! canonical crate (abr-irrs c7410c5):
//!
//!   IRRS declaration evaluation  irrs_eval::Evaluator::evaluate
//!   IRRS observation admission   relational_gate::RegionBuilder::establish,
//!                                relational_gate::AdmissionGate::submit
//!   Canonical execution          irrs_session::execute::execute → operator_a,
//!                                compute_rho, operator_b, operator_r,
//!                                detection_indicator (irrs-kernel = Kernel V9
//!                                1c1ef70 byte for byte), then an independent
//!                                RPPI re-execution
//!   Structural check             relational_isolation
//!
//! It reads the declaration JSON (irrs-eval schema plus a "lesson" block),
//! passes gate-admitted values and declared parameters to the canonical calls,
//! and records what they return. It computes no difference, accumulation, ρ,
//! agreement or status from numbers. (Carried over from Lesson 1 revision 2,
//! `observable_set.rs`, with the report text generalised.)

use std::collections::BTreeMap;
use std::fmt::Write as _;

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

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

fn text<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty())
}

fn parse_edge(s: &str) -> Option<(String, String)> {
    s.split_once("->").map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
}

struct Reading {
    id: String,
    value: f64,
}

impl Observable for Reading {
    fn source_id(&self) -> &str { &self.id }
    fn primary_value(&self) -> f64 { self.value }
}

#[derive(Default, Clone, Debug)]
pub struct Outcome {
    pub report: String,
    pub irrs_json: Option<String>,
    pub digest: String,
    pub irrs_status: String,
    pub open_conditions: usize,
    pub violations: usize,
    pub admission: String,
    pub calculation: String,
    pub executed: bool,
    /// Gate-admitted node values, by node id.
    pub admitted: BTreeMap<String, f64>,
    /// Edge outputs by "from->to".
    pub a: BTreeMap<String, f64>,
    pub b: BTreeMap<String, f64>,
    pub r: BTreeMap<String, f64>,
    pub indicator: BTreeMap<String, f64>,
    pub rho: BTreeMap<String, f64>,
    pub kernel_open: Vec<String>,
}

pub fn run(name: &str, raw: &str) -> Outcome {
    let mut o = Outcome::default();
    o.irrs_status = "not reached".into();
    o.admission = "not reached".into();
    o.calculation = "OPEN — not evaluated".into();
    let mut r = String::new();
    run_inner(name, raw, &mut r, &mut o);
    o.report = r;
    o
}

fn run_inner(name: &str, raw: &str, r: &mut String, o: &mut Outcome) {
    let _ = writeln!(r, "Field declaration: {name}");
    o.digest = hex::encode(Sha256::digest(raw.as_bytes()));
    let _ = writeln!(r, "Declaration SHA-256 (raw text, as irrs-eval computes it): {}", o.digest);

    let doc: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => { o.calculation = format!("OPEN — not valid JSON: {e}"); let _ = writeln!(r, "  {}", o.calculation); return; }
    };
    let irrs_doc: IrrsDeclarationDoc = match serde_json::from_str(raw) {
        Ok(d) => d,
        Err(e) => { o.calculation = format!("OPEN — irrs-eval schema could not read the declaration: {e}"); let _ = writeln!(r, "  {}", o.calculation); return; }
    };
    let Some(lesson) = doc.get("lesson") else {
        o.calculation = "OPEN — no lesson block".into(); let _ = writeln!(r, "  {}", o.calculation); return;
    };

    // ── Step 1: Declaration ───────────────────────────────────────────────
    let _ = writeln!(r, "\n== Step 1 — Declaration ==");
    let _ = writeln!(r, "  Observation status : {}", text(lesson, "observation_status").unwrap_or("<not declared>"));
    let rp = &irrs_doc.regional_prerequisite;
    let unit = rp.extent_unit.trim().to_string();
    let bound = |b: Option<f64>| b.map(|v| format!("{v:?}")).unwrap_or_else(|| "<not declared>".into());
    let _ = writeln!(r, "  D (observable extent): [{}, {}] {unit} — region '{}'",
        bound(rp.extent_floor), bound(rp.extent_ceiling), rp.region_name);
    let mapping = irrs_doc.mappings.first();
    let _ = writeln!(r, "  M: mapping '{}'", mapping.map(|m| m.name.as_str()).unwrap_or("<none declared>"));
    if let Some(m) = mapping {
        let _ = writeln!(r, "     input   : {}", m.input_description);
        let _ = writeln!(r, "     preserves: {}", m.preserves);
        let _ = writeln!(r, "     discards : {}", m.discards);
    }
    let Some(topo) = irrs_doc.abr_topology.as_ref() else {
        o.calculation = "OPEN — no abr_topology declared".into(); let _ = writeln!(r, "  {}", o.calculation); return;
    };
    let readings = lesson.get("readings").and_then(Value::as_object);
    let _ = writeln!(r, "  Loci (list position is a label only, NOT topology):");
    for (i, n) in topo.nodes.iter().enumerate() {
        let v = readings.and_then(|m| m.get(&n.observable)).and_then(Value::as_f64);
        let _ = writeln!(r, "    [{i}] {:16} reading {} {unit}", n.id,
            v.map(|x| format!("{x:?}")).unwrap_or_else(|| "<not declared>".into()));
    }
    let edge_values: Vec<&Value> = doc.pointer("/abr_topology/edges").and_then(Value::as_array)
        .map(|a| a.iter().collect()).unwrap_or_default();
    let _ = writeln!(r, "  Declared relations:");
    for (i, e) in topo.edges.iter().enumerate() {
        let class = edge_values.get(i).and_then(|v| text(v, "class")).unwrap_or("<not declared>");
        let _ = writeln!(r, "    e{i}: {} → {}  class {class}", e.from_node, e.to_node);
        let _ = writeln!(r, "        basis: {}", e.description);
    }
    let interior: Vec<&Value> = lesson.get("interior_loci").and_then(Value::as_array)
        .map(|a| a.iter().collect()).unwrap_or_default();
    let _ = writeln!(r, "  Interior loci declared by M (§10): {}",
        interior.iter().filter_map(|il| text(il, "node")).collect::<Vec<_>>().join(", "));
    let params = topo.operator_parameters.as_ref();
    let rho_base = params.and_then(|p| p.rho_base.as_ref());
    let chi_0 = params.and_then(|p| p.chi_0.as_ref());
    let tolerance = lesson.get("detection_tolerance").and_then(Value::as_f64);
    let _ = writeln!(r, "  ρ_base = {}; χ₀ = {}; component pairs / cc: none; 𝟙 tolerance = {}",
        rho_base.map(|p| format!("{:?} ({:?})", p.value, p.provenance)).unwrap_or_else(|| "<not declared>".into()),
        chi_0.map(|p| format!("{:?} {unit} ({:?})", p.value, p.provenance)).unwrap_or_else(|| "<not declared>".into()),
        tolerance.map(|t| format!("{t:?}")).unwrap_or_else(|| "<not declared>".into()));

    // ── Step 2: IRRS declaration evaluation ───────────────────────────────
    let _ = writeln!(r, "\n== Step 2 — IRRS declaration evaluation (irrs_eval::Evaluator::evaluate) ==");
    let record = Evaluator::evaluate(irrs_doc.clone(), o.digest.clone());
    o.irrs_json = serde_json::to_string_pretty(&record).ok();
    o.open_conditions = record.open_conditions.len();
    o.violations = record.violations.len();
    let _ = writeln!(r, "  Record {} — status {:?}", record.record_id, record.status);
    if let Some(t) = &record.abr_topology {
        let s = &t.summary;
        let _ = writeln!(r, "  ABR topology: nodes {} (observed {}, proposed {}); edges {} (observed {}, proposed {}); interior locus {}; parameters {} (illustrative {})",
            s.total_nodes, s.observed_nodes, s.proposed_nodes, s.total_edges, s.observed_edges, s.proposed_edges,
            s.interior_locus_status, s.total_parameters, s.illustrative_parameters);
    }
    let _ = writeln!(r, "  Open conditions returned by IRRS ({}):", record.open_conditions.len());
    for c in &record.open_conditions { let _ = writeln!(r, "    OPEN: {c}"); }
    for v in &record.violations { let _ = writeln!(r, "    VIOLATION: {v}"); }
    match record.status {
        EvaluationStatus::Rejected => {
            o.irrs_status = format!("REJECTED — {}", record.failure_reason.clone().unwrap_or_default());
            let _ = writeln!(r, "  A contradiction is established. Nothing is executed.");
            return;
        }
        EvaluationStatus::Open => {
            o.irrs_status = format!("OPEN — {} condition(s), {} violation(s); not authorized", o.open_conditions, o.violations);
        }
        ref other => o.irrs_status = format!("{other:?}"),
    }

    // ── Step 3: IRRS observation admission ────────────────────────────────
    let _ = writeln!(r, "\n== Step 3 — IRRS observation admission (relational_gate) ==");
    let (Some(floor), Some(ceiling)) = (rp.extent_floor, rp.extent_ceiling) else {
        o.admission = "OPEN — observable extent not declared".into(); let _ = writeln!(r, "  {}", o.admission); return;
    };
    let Some(mapping) = mapping else {
        o.admission = "OPEN — no mapping declared".into(); let _ = writeln!(r, "  {}", o.admission); return;
    };
    let extent = match ObservableExtent::new(floor, ceiling, leak(&unit)) {
        Ok(e) => e,
        Err(e) => { o.admission = format!("{e}"); let _ = writeln!(r, "  {e}"); return; }
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
        o.admission = "OPEN — no observed locus declared".into(); let _ = writeln!(r, "  {}", o.admission); return;
    };
    let region = match builder.establish(observed_locus) {
        Ok(reg) => reg,
        Err(e) => { o.admission = format!("{e}"); let _ = writeln!(r, "  Region not established — {e}"); return; }
    };
    let _ = writeln!(r, "  Region '{}' established; extent [{:?}, {:?}] {}", region.name, region.extent.floor, region.extent.ceiling, region.extent.unit);
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
            "Declared M: the declared reading enters D unchanged in the declared unit",
            |primary, _reference| Ok(primary),
            |v: &f64| *v,
        )],
    );
    let links: Vec<(String, String)> = lesson.get("provenance_links").and_then(Value::as_array)
        .map(|a| a.iter().filter_map(|l| Some((text(l, "step")?.to_string(), text(l, "description")?.to_string()))).collect())
        .unwrap_or_default();
    for n in &topo.nodes {
        let Some(value) = readings.and_then(|m| m.get(&n.observable)).and_then(Value::as_f64) else {
            let _ = writeln!(r, "  {}: OPEN — no reading declared", n.id);
            continue;
        };
        let mut chain = ProvenanceChain::new();
        for (s, d) in &links { chain = chain.link(leak(s), leak(d)); }
        let package = DeclarationPackage {
            source: Reading { id: n.observable.clone(), value },
            mapping_name: m_name,
            topology_id,
            provenance: chain,
        };
        match gate.submit(package) {
            GateOutcome::Admitted { observation, trace } => {
                let p = &trace.provenance;
                let _ = writeln!(r, "  {}: ADMITTED — raw {:?}, gate-computed {:?} {unit}", n.id, p.primary_input, p.gate_computed_result);
                o.admitted.insert(n.id.clone(), observation.gate_result());
            }
            GateOutcome::Divergent { divergence, .. } => { let _ = writeln!(r, "  {}: DIVERGENT — {}", n.id, divergence.description); }
            GateOutcome::Open { condition, .. } => { let _ = writeln!(r, "  {}: {condition}", n.id); }
            GateOutcome::Failed { error, .. } => { let _ = writeln!(r, "  {}: FAILED — {error}", n.id); }
            GateOutcome::Rejected { violation, .. } => { let _ = writeln!(r, "  {}: {violation}", n.id); }
        }
    }
    if o.admitted.len() != topo.nodes.len() || topo.nodes.is_empty() {
        o.admission = "OPEN — not every reading was admitted".into();
        let _ = writeln!(r, "  Only gate-admitted observations may enter the kernel. Stopping here.");
        return;
    }
    o.admission = format!("ADMITTED — all {} readings", o.admitted.len());

    // ── Step 4: Kernel selection (§10) ────────────────────────────────────
    let _ = writeln!(r, "\n== Step 4 — Kernel selection (kernel_reference.md §10) ==");
    let declared_edges: Vec<(String, String)> = topo.edges.iter().map(|e| (e.from_node.clone(), e.to_node.clone())).collect();
    let mut established = Vec::new();
    for il in &interior {
        let node = text(il, "node").unwrap_or("");
        let ok = match (text(il, "incoming").and_then(parse_edge), text(il, "continuing").and_then(parse_edge)) {
            (Some(i), Some(c)) => declared_edges.contains(&i) && declared_edges.contains(&c) && i.1 == node && c.0 == node,
            _ => false,
        };
        if ok { established.push(node.to_string()); }
        else { let _ = writeln!(r, "  {node}: declared incoming/continuing relations not both declared at this node — not established"); }
    }
    if established.is_empty() {
        o.calculation = "OPEN — no interior locus established; Primary condition (not executed here)".into();
        let _ = writeln!(r, "  {}", o.calculation);
        return;
    }
    if text(lesson, "kernel_selection") != Some("abr") {
        o.calculation = "REJECTED — interior loci established but declared selection is not 'abr'".into();
        let _ = writeln!(r, "  {}", o.calculation);
        return;
    }
    let _ = writeln!(r, "  ABR kernel A → B → R — interior loci established by M: {}. R is not optional (§10).", established.join(", "));

    // ── Step 5: Completeness (§5 Step 2) ──────────────────────────────────
    let _ = writeln!(r, "\n== Step 5 — Completeness of the declaration for A → B → R ==");
    let mut missing = Vec::new();
    if rho_base.is_none() { missing.push("ρ_base"); }
    if chi_0.is_none() { missing.push("χ₀"); }
    if tolerance.is_none() { missing.push("detection tolerance"); }
    let mut classes = Vec::new();
    for (i, _) in topo.edges.iter().enumerate() {
        match edge_values.get(i).and_then(|v| text(v, "class")) {
            Some("continuation") => classes.push(EdgeProvenance::Continuation),
            Some("coupling") => classes.push(EdgeProvenance::Coupling),
            _ => missing.push("a spatial provenance class (continuation or coupling) for every edge"),
        }
    }
    if !missing.is_empty() {
        missing.dedup();
        o.calculation = format!("OPEN — not executed. Missing declaration(s): {}", missing.join("; "));
        let _ = writeln!(r, "  STOP at the missing declaration(s), returned to Origin: {}", missing.join("; "));
        return;
    }
    let _ = writeln!(r, "  Every input the A → B → R path uses is declared.");

    // ── Step 6: Canonical execution ───────────────────────────────────────
    let _ = writeln!(r, "\n== Step 6 — Canonical execution (irrs_session::execute::execute) ==");
    let index: BTreeMap<&str, usize> = topo.nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();
    let edges_idx: Vec<(usize, usize)> = topo.edges.iter().map(|e| (index[e.from_node.as_str()], index[e.to_node.as_str()])).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(topo.nodes.len(), edges_idx.clone(), classes.clone());
    let field = NodeField::new(vec![topo.nodes.iter().map(|n| o.admitted[&n.id]).collect()]);
    let labels: Vec<String> = topo.edges.iter().map(|e| format!("{}->{}", e.from_node, e.to_node)).collect();
    let _ = writeln!(r, "  Inputs: NodeField x[0] = {:?} {unit}", field.data[0]);
    let _ = writeln!(r, "          edges {:?}, provenance {:?}", edges_idx, classes);
    let _ = writeln!(r, "  Structural check relational_isolation(rel) = {}", relational_isolation(&rel));
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
        Ok(v) => v,
        Err(e) => {
            o.calculation = match &e {
                IrrsError::VerificationFailed(v) => format!("REJECTED — RPPI re-execution diverged: {v:?}"),
                other => format!("OPEN — structural invalidity returned by IRRS: {other:?}"),
            };
            let _ = writeln!(r, "  NOT EXECUTED — {}", o.calculation);
            return;
        }
    };
    o.executed = true;
    let _ = writeln!(r, "  Executed. Independent RPPI re-execution (verify_abr_irrs) agreed.");
    let _ = writeln!(r, "\n  Edge outputs (unit {unit}; 𝟙 is 0.0 or 1.0):");
    let _ = writeln!(r, "    {:30} {:>14} {:>14} {:>14} {:>4}", "relation", "A", "B", "R", "𝟙");
    for (e, l) in labels.iter().enumerate() {
        let (a, b, rr, ind) = (out.a.spatial[0][e], out.b.spatial[0][e], out.r.spatial[0][e], out.detection_indicators[0][e]);
        let _ = writeln!(r, "    {:30} {:>14} {:>14} {:>14} {:>4}", l, fmt(a), fmt(b), fmt(rr), format!("{ind:?}"));
        o.a.insert(l.clone(), a);
        o.b.insert(l.clone(), b);
        o.r.insert(l.clone(), rr);
        o.indicator.insert(l.clone(), ind);
    }
    let _ = writeln!(r, "  Node outputs (ρ dimensionless):");
    for (i, n) in topo.nodes.iter().enumerate() {
        let _ = writeln!(r, "    ρ[{}] = {}", n.id, fmt(out.rho[i]));
        o.rho.insert(n.id.clone(), out.rho[i]);
    }
    let _ = writeln!(r, "  Kernel V9 open conditions surfaced by IRRS:");
    for k in &out.kernel_open {
        let d = k.describe().to_string();
        let _ = writeln!(r, "    {d}");
        o.kernel_open.push(d);
    }
    if out.detection_indicators[0].iter().any(|&v| v != 0.0) {
        let _ = writeln!(r, "  𝟙[e] = 1.0 on at least one relation: a REVIEW SIGNAL (§11.4), preserved; not a finding.");
    }
    o.calculation = format!("CALCULATED — A, ρ, B, R, 𝟙 over {} relations, {} loci (RPPI re-execution agreed); ρ_base/χ₀ {:?}/{:?}",
        labels.len(), topo.nodes.len(), rho_base.unwrap().provenance, chi_0.unwrap().provenance);
    let _ = writeln!(r, "\n  V9 provenance admission for A → B → R: OPEN — outside abr-kernel 1c1ef70 admission scope ('OPEN: ABR kernel A → B → R').");
}

/// Shortest exact round-trip form, rounded for display only to 12 significant
/// places to avoid printing float noise as if it were information.
pub fn fmt(v: f64) -> String {
    if v == 0.0 { return "0.0".into(); }
    let s = format!("{:.12e}", v);
    let parsed: f64 = s.parse().unwrap_or(v);
    let short = format!("{parsed}");
    if short.len() > 16 { format!("{v:.9}") } else { short }
}
