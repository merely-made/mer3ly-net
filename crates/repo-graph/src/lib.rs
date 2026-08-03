use std::collections::{HashMap, HashSet};

use arrangements::camera::CanvasViewport;
use arrangements::scene::{CanvasEdge, CanvasNode, CanvasSceneInput};
use arrangements::{
    AxisValue, Layout, LayoutExtras, LayoutRegistry, Radial, RadialAngularPolicy, RadialConfig,
    RadialUnreachablePolicy, StaticLayoutState, Timeline, TimelineConfig,
};
use euclid::default::Point2D;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const FOCUS_REPOSITORY: &str = "mere";
const DEFAULT_ARRANGEMENT: &str = "graph_layout:radial";
const TIMELINE_TRACK_SIZE: usize = 5;
const TIMELINE_BAND_GAP: f32 = 210.0;
const TIMELINE_LANE_GAP: f32 = 85.0;
const ARRANGEMENT_ORDER: &[&str] = &[
    "graph_layout:radial",
    "graph_layout:grid",
    "graph_layout:phyllotaxis",
    "graph_layout:timeline",
    "graph_layout:kanban",
    "graph_layout:penrose",
    "graph_layout:lsystem",
];
const UNAVAILABLE_ARRANGEMENTS: &[(&str, &str)] = &[(
    "graph_layout:semantic_embedding",
    "This site does not yet publish semantic coordinates.",
)];

#[derive(Clone, Debug, Deserialize)]
struct GraphInput {
    schema: String,
    nodes: Vec<GraphNodeInput>,
    edges: Vec<GraphEdge>,
}

#[derive(Clone, Debug, Deserialize)]
struct GraphNodeInput {
    id: String,
    name: String,
    class: String,
    status: String,
    pushed_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GraphEdge {
    id: String,
    source: String,
    target: String,
    kind: String,
    provenance: String,
}

#[derive(Clone, Debug, Serialize)]
struct GraphLayout {
    schema: &'static str,
    authority_schema: String,
    engine: String,
    focus: &'static str,
    default_arrangement: &'static str,
    nodes: Vec<GraphNodeLayout>,
    edges: Vec<GraphEdge>,
    arrangements: Vec<GraphArrangement>,
    unavailable_arrangements: Vec<UnavailableArrangement>,
}

#[derive(Clone, Debug, Serialize)]
struct GraphNodeLayout {
    id: String,
    name: String,
    class: String,
    status: String,
    pushed_at: String,
    x: f32,
    y: f32,
}

#[derive(Clone, Debug, Serialize)]
struct GraphArrangement {
    id: String,
    name: String,
    description: String,
    engine: String,
    nodes: Vec<GraphNodePosition>,
}

#[derive(Clone, Debug, Serialize)]
struct GraphNodePosition {
    id: String,
    x: f32,
    y: f32,
}

#[derive(Clone, Debug, Serialize)]
struct UnavailableArrangement {
    id: String,
    name: String,
    reason: String,
}

#[wasm_bindgen]
pub fn layout_graph(input: &str) -> Result<String, JsValue> {
    layout_graph_json(input).map_err(|error| JsValue::from_str(&error))
}

fn layout_graph_json(input: &str) -> Result<String, String> {
    let input: GraphInput =
        serde_json::from_str(input).map_err(|error| format!("invalid graph JSON: {error}"))?;
    validate(&input)?;

    let scene = CanvasSceneInput {
        nodes: input
            .nodes
            .iter()
            .map(|node| CanvasNode {
                id: node.id.clone(),
                position: Point2D::origin(),
                radius: 24.0,
                label: Some(node.name.clone()),
            })
            .collect(),
        edges: input
            .edges
            .iter()
            .map(|edge| CanvasEdge::untagged(edge.source.clone(), edge.target.clone()))
            .collect(),
    };
    let registry = LayoutRegistry::<String>::default();
    let mut arrangements = Vec::with_capacity(ARRANGEMENT_ORDER.len());
    for arrangement_id in ARRANGEMENT_ORDER {
        let provider = registry
            .resolve(arrangement_id)
            .ok_or_else(|| format!("Mere arrangement registry is missing {arrangement_id}"))?;
        let capability = provider.capability();
        let positions = arrangement_positions(&input, &scene, arrangement_id, &provider)?;
        arrangements.push(GraphArrangement {
            id: capability.id.clone(),
            name: capability.display_name,
            description: arrangement_description(&capability.id, capability.description),
            engine: capability.id,
            nodes: positions,
        });
    }
    let unavailable_arrangements = UNAVAILABLE_ARRANGEMENTS
        .iter()
        .map(|(arrangement_id, reason)| {
            let capability = registry
                .resolve(arrangement_id)
                .ok_or_else(|| format!("Mere arrangement registry is missing {arrangement_id}"))?
                .capability();
            Ok(UnavailableArrangement {
                id: capability.id,
                name: capability.display_name,
                reason: (*reason).to_owned(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let default_arrangement = arrangements
        .iter()
        .find(|arrangement| arrangement.id == DEFAULT_ARRANGEMENT)
        .ok_or_else(|| "default repository arrangement is unavailable".to_owned())?;
    let default_positions = default_arrangement
        .nodes
        .iter()
        .map(|position| (position.id.as_str(), position))
        .collect::<HashMap<_, _>>();
    let nodes = input
        .nodes
        .iter()
        .map(|node| {
            let position = default_positions
                .get(node.id.as_str())
                .ok_or_else(|| format!("default arrangement lost node {}", node.id))?;
            Ok(GraphNodeLayout {
                id: node.id.clone(),
                name: node.name.clone(),
                class: node.class.clone(),
                status: node.status.clone(),
                pushed_at: node.pushed_at.clone(),
                x: position.x,
                y: position.y,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    serde_json::to_string(&GraphLayout {
        schema: "mer3ly.repo-graph-layout/v2",
        authority_schema: input.schema.clone(),
        engine: default_arrangement.engine.clone(),
        focus: FOCUS_REPOSITORY,
        default_arrangement: DEFAULT_ARRANGEMENT,
        nodes,
        edges: input.edges.clone(),
        arrangements,
        unavailable_arrangements,
    })
    .map_err(|error| format!("could not serialize graph layout: {error}"))
}

fn arrangement_positions(
    input: &GraphInput,
    scene: &CanvasSceneInput<String>,
    arrangement_id: &str,
    provider: &std::sync::Arc<dyn arrangements::LayoutProvider<String>>,
) -> Result<Vec<GraphNodePosition>, String> {
    let mut extras = LayoutExtras::default();
    match arrangement_id {
        "graph_layout:timeline" => {
            for node in &input.nodes {
                extras.axis_value_by_node.insert(
                    node.id.clone(),
                    AxisValue::Numeric(timestamp_coordinate(&node.pushed_at)?),
                );
            }
        }
        "graph_layout:kanban" => {
            for node in &input.nodes {
                extras
                    .axis_value_by_node
                    .insert(node.id.clone(), AxisValue::Categorical(node.status.clone()));
            }
        }
        _ => {}
    }

    let deltas = if arrangement_id == DEFAULT_ARRANGEMENT {
        let mut layout = Radial::new(RadialConfig {
            focus: Some(FOCUS_REPOSITORY.to_owned()),
            center: Point2D::origin(),
            ring_spacing: 190.0,
            angular_policy: RadialAngularPolicy::DegreeWeighted,
            rotation_offset: 0.0,
            unreachable_policy: RadialUnreachablePolicy::LeaveInPlace,
        });
        layout.step(
            scene,
            &mut StaticLayoutState::default(),
            0.0,
            &CanvasViewport::default(),
            &extras,
        )
    } else if arrangement_id == "graph_layout:timeline" {
        let mut layout = Timeline::new(TimelineConfig {
            row_gap: 120.0,
            ..TimelineConfig::default()
        });
        layout.step(
            scene,
            &mut StaticLayoutState::default(),
            0.0,
            &CanvasViewport::default(),
            &extras,
        )
    } else {
        let mut layout = provider.create_default();
        let mut state = layout.default_state_erased();
        layout.step_dyn(scene, &mut state, 0.0, &CanvasViewport::default(), &extras)
    };

    let mut host_positions = HashMap::new();
    if arrangement_id == DEFAULT_ARRANGEMENT {
        let unreachable = input
            .nodes
            .iter()
            .filter(|node| node.id != FOCUS_REPOSITORY && !deltas.contains_key(&node.id))
            .collect::<Vec<_>>();
        let lane_center = unreachable.len().saturating_sub(1) as f32 * 0.5;
        for (index, node) in unreachable.into_iter().enumerate() {
            host_positions.insert(
                node.id.clone(),
                Point2D::new(-470.0, (index as f32 - lane_center) * 190.0),
            );
        }
    }

    let raw_positions = input
        .nodes
        .iter()
        .map(|node| {
            let point = host_positions.get(&node.id).copied().unwrap_or_else(|| {
                deltas
                    .get(&node.id)
                    .map_or_else(Point2D::origin, |delta| Point2D::origin() + *delta)
            });
            (node.id.clone(), point)
        })
        .collect::<Vec<_>>();
    let raw_positions = if arrangement_id == "graph_layout:timeline" {
        wrap_timeline_tracks(raw_positions)
    } else {
        raw_positions
    };
    normalize_positions(arrangement_id, raw_positions)
}

fn wrap_timeline_tracks(mut positions: Vec<(String, Point2D<f32>)>) -> Vec<(String, Point2D<f32>)> {
    positions.sort_by(|(left_id, left), (right_id, right)| {
        left.x
            .total_cmp(&right.x)
            .then_with(|| left.y.total_cmp(&right.y))
            .then_with(|| left_id.cmp(right_id))
    });
    positions
        .into_iter()
        .enumerate()
        .map(|(index, (id, _))| {
            let band = (index / TIMELINE_TRACK_SIZE) as f32;
            let lane = (index % TIMELINE_TRACK_SIZE) as f32;
            (
                id,
                Point2D::new(band * TIMELINE_BAND_GAP, lane * TIMELINE_LANE_GAP),
            )
        })
        .collect()
}

fn normalize_positions(
    arrangement_id: &str,
    positions: Vec<(String, Point2D<f32>)>,
) -> Result<Vec<GraphNodePosition>, String> {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for (_, position) in &positions {
        if !position.x.is_finite() || !position.y.is_finite() {
            return Err(format!(
                "arrangement {arrangement_id} emitted a non-finite position"
            ));
        }
        min_x = min_x.min(position.x);
        max_x = max_x.max(position.x);
        min_y = min_y.min(position.y);
        max_y = max_y.max(position.y);
    }
    let width = max_x - min_x;
    let height = max_y - min_y;
    if width <= f32::EPSILON && height <= f32::EPSILON {
        return Err(format!(
            "arrangement {arrangement_id} collapsed every repository"
        ));
    }
    let height_limit = if arrangement_id == "graph_layout:timeline" {
        650.0
    } else {
        520.0
    };
    let scale = (620.0 / width.max(1.0)).min(height_limit / height.max(1.0));
    let center_x = (min_x + max_x) * 0.5;
    let center_y = (min_y + max_y) * 0.5;
    Ok(positions
        .into_iter()
        .map(|(id, position)| GraphNodePosition {
            id,
            x: (position.x - center_x) * scale,
            y: (position.y - center_y) * scale,
        })
        .collect())
}

fn timestamp_coordinate(value: &str) -> Result<f64, String> {
    let digits = value
        .chars()
        .filter(char::is_ascii_digit)
        .take(8)
        .collect::<String>();
    if digits.len() != 8 {
        return Err(format!(
            "repository push timestamp is not sortable: {value}"
        ));
    }
    digits
        .parse::<f64>()
        .map_err(|error| format!("repository push timestamp is not sortable: {error}"))
}

fn arrangement_description(id: &str, registry_description: Option<String>) -> String {
    match id {
        "graph_layout:timeline" => {
            "Repositories grouped by their last public push date.".to_owned()
        }
        "graph_layout:kanban" => "Repositories grouped by public project status.".to_owned(),
        _ => registry_description.unwrap_or_else(|| "Mere positional arrangement.".to_owned()),
    }
}

fn validate(input: &GraphInput) -> Result<(), String> {
    if input.schema != "mer3ly.repo-graph/v1" {
        return Err(format!("unsupported graph schema {}", input.schema));
    }
    if input.nodes.is_empty() {
        return Err("repository graph has no nodes".to_owned());
    }

    let mut node_ids = HashSet::with_capacity(input.nodes.len());
    for node in &input.nodes {
        if node.id.is_empty()
            || node.name.is_empty()
            || node.class.is_empty()
            || node.status.is_empty()
            || node.pushed_at.is_empty()
        {
            return Err("repository graph contains an incomplete node".to_owned());
        }
        if !node_ids.insert(node.id.as_str()) {
            return Err(format!("duplicate repository graph node {}", node.id));
        }
    }
    if !node_ids.contains(FOCUS_REPOSITORY) {
        return Err(format!(
            "repository graph is missing focal node {FOCUS_REPOSITORY}"
        ));
    }

    let mut edge_ids = HashSet::with_capacity(input.edges.len());
    for edge in &input.edges {
        if !edge_ids.insert(edge.id.as_str()) {
            return Err(format!("duplicate repository graph edge {}", edge.id));
        }
        if !node_ids.contains(edge.source.as_str()) || !node_ids.contains(edge.target.as_str()) {
            return Err(format!(
                "repository graph edge {} has an unknown endpoint",
                edge.id
            ));
        }
    }

    let mut positions = HashMap::with_capacity(input.nodes.len());
    for (index, node) in input.nodes.iter().enumerate() {
        positions.insert(node.id.as_str(), index);
    }
    if positions.len() != input.nodes.len() {
        return Err("repository graph node ordering is not stable".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "schema": "mer3ly.repo-graph/v1",
      "nodes": [
        {"id":"mere","name":"Mere","class":"platform","status":"active","pushed_at":"2026-07-30T05:44:23Z"},
        {"id":"genet","name":"Genet","class":"platform","status":"active","pushed_at":"2026-07-30T05:44:24Z"},
        {"id":"turnstone","name":"Turnstone","class":"product","status":"prototype","pushed_at":"2026-07-31T05:07:42Z"}
      ],
      "edges": [
        {"id":"mere-depends-on-genet","source":"mere","target":"genet","kind":"depends_on","provenance":"derived"},
        {"id":"turnstone-hosts-mere","source":"turnstone","target":"mere","kind":"host_for","provenance":"curated"}
      ]
    }"#;

    #[test]
    fn arrangement_catalog_preserves_graph_identity() {
        let encoded = layout_graph_json(SAMPLE).expect("layout graph");
        let value: serde_json::Value = serde_json::from_str(&encoded).expect("parse layout");
        assert_eq!(value["schema"], "mer3ly.repo-graph-layout/v2");
        assert_eq!(value["engine"], "graph_layout:radial");
        assert_eq!(value["focus"], "mere");
        assert_eq!(value["default_arrangement"], "graph_layout:radial");
        assert_eq!(value["nodes"].as_array().expect("nodes").len(), 3);
        assert_eq!(value["edges"].as_array().expect("edges").len(), 2);
        assert_eq!(
            value["arrangements"]
                .as_array()
                .expect("arrangements")
                .len(),
            7
        );
        assert_eq!(
            value["unavailable_arrangements"]
                .as_array()
                .expect("unavailable arrangements")
                .len(),
            1
        );
        assert_eq!(value["nodes"][0]["id"], "mere");
        assert_eq!(value["nodes"][0]["pushed_at"], "2026-07-30T05:44:23Z");
        assert_eq!(value["edges"][1]["id"], "turnstone-hosts-mere");
        for arrangement in value["arrangements"].as_array().expect("arrangements") {
            assert_eq!(
                arrangement["nodes"].as_array().expect("scene nodes").len(),
                3
            );
            assert_eq!(arrangement["nodes"][0]["id"], "mere");
        }
    }

    #[test]
    fn arrangement_catalog_is_deterministic() {
        let first = layout_graph_json(SAMPLE).expect("first layout");
        let second = layout_graph_json(SAMPLE).expect("second layout");
        assert_eq!(first, second);
    }

    #[test]
    fn unknown_edge_endpoint_is_rejected() {
        let invalid = SAMPLE.replace("\"target\":\"genet\"", "\"target\":\"missing\"");
        let error = layout_graph_json(&invalid).expect_err("unknown endpoint should fail");
        assert!(error.contains("unknown endpoint"));
    }

    #[test]
    fn every_registered_arrangement_is_selectable_or_explained() {
        let encoded = layout_graph_json(SAMPLE).expect("layout graph catalog");
        let value: serde_json::Value = serde_json::from_str(&encoded).expect("parse layout");
        let mut arrangement_ids = value["arrangements"]
            .as_array()
            .expect("arrangements")
            .iter()
            .map(|arrangement| arrangement["id"].as_str().expect("arrangement id"))
            .collect::<Vec<_>>();
        arrangement_ids.extend(
            value["unavailable_arrangements"]
                .as_array()
                .expect("unavailable arrangements")
                .iter()
                .map(|arrangement| arrangement["id"].as_str().expect("arrangement id")),
        );
        arrangement_ids.sort_unstable();
        assert_eq!(
            arrangement_ids,
            vec![
                "graph_layout:grid",
                "graph_layout:kanban",
                "graph_layout:lsystem",
                "graph_layout:penrose",
                "graph_layout:phyllotaxis",
                "graph_layout:radial",
                "graph_layout:semantic_embedding",
                "graph_layout:timeline",
            ]
        );
    }

    #[test]
    fn timeline_tracks_wrap_dense_dates_without_collisions() {
        let positions = (0..19)
            .rev()
            .map(|index| {
                (
                    format!("node-{index:02}"),
                    Point2D::new(index as f32 * 0.01, 0.0),
                )
            })
            .collect();
        let wrapped = wrap_timeline_tracks(positions);
        let points = wrapped
            .iter()
            .map(|(_, point)| (point.x.round() as i32, point.y.round() as i32))
            .collect::<HashSet<_>>();
        let bands = wrapped
            .iter()
            .map(|(_, point)| point.x.round() as i32)
            .collect::<HashSet<_>>();

        assert_eq!(wrapped.len(), 19);
        assert_eq!(points.len(), 19);
        assert_eq!(bands.len(), 4);
        assert_eq!(wrapped.first().expect("first node").0, "node-00");
        assert_eq!(wrapped.last().expect("last node").0, "node-18");
    }
}
