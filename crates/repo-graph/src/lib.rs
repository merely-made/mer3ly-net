use std::collections::{HashMap, HashSet};

use arrangements::camera::CanvasViewport;
use arrangements::scene::{CanvasEdge, CanvasNode, CanvasSceneInput};
use arrangements::{
    Layout, LayoutExtras, Radial, RadialAngularPolicy, RadialConfig, RadialUnreachablePolicy,
    StaticLayoutState,
};
use euclid::default::Point2D;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const FOCUS_REPOSITORY: &str = "mere";

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
    engine: &'static str,
    focus: &'static str,
    nodes: Vec<GraphNodeLayout>,
    edges: Vec<GraphEdge>,
}

#[derive(Clone, Debug, Serialize)]
struct GraphNodeLayout {
    id: String,
    name: String,
    class: String,
    status: String,
    x: f32,
    y: f32,
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
    let mut layout = Radial::new(RadialConfig {
        focus: Some(FOCUS_REPOSITORY.to_owned()),
        center: Point2D::origin(),
        ring_spacing: 190.0,
        angular_policy: RadialAngularPolicy::DegreeWeighted,
        rotation_offset: 0.0,
        unreachable_policy: RadialUnreachablePolicy::LeaveInPlace,
    });
    let mut state = StaticLayoutState::default();
    let deltas = layout.step(
        &scene,
        &mut state,
        0.0,
        &CanvasViewport::default(),
        &LayoutExtras::default(),
    );

    let unreachable = input
        .nodes
        .iter()
        .filter(|node| node.id != FOCUS_REPOSITORY && !deltas.contains_key(&node.id))
        .map(|node| node.id.clone())
        .collect::<HashSet<_>>();
    let unreachable_count = unreachable.len();
    let mut unreachable_index = 0;
    let nodes = input
        .nodes
        .into_iter()
        .map(|node| {
            let position = if unreachable.contains(node.id.as_str()) {
                let y = (unreachable_index as f32
                    - (unreachable_count.saturating_sub(1)) as f32 * 0.5)
                    * 210.0;
                unreachable_index += 1;
                Point2D::new(-470.0, y)
            } else {
                deltas
                    .get(&node.id)
                    .map_or_else(Point2D::origin, |delta| Point2D::origin() + *delta)
            };
            GraphNodeLayout {
                id: node.id,
                name: node.name,
                class: node.class,
                status: node.status,
                x: position.x,
                y: position.y,
            }
        })
        .collect();
    serde_json::to_string(&GraphLayout {
        schema: "mer3ly.repo-graph-layout/v1",
        authority_schema: input.schema,
        engine: "mere-arrangements/radial+unreachable-lane",
        focus: FOCUS_REPOSITORY,
        nodes,
        edges: input.edges,
    })
    .map_err(|error| format!("could not serialize graph layout: {error}"))
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
        {"id":"mere","name":"Mere","class":"platform","status":"active"},
        {"id":"genet","name":"Genet","class":"platform","status":"active"},
        {"id":"turnstone","name":"Turnstone","class":"product","status":"prototype"}
      ],
      "edges": [
        {"id":"mere-depends-on-genet","source":"mere","target":"genet","kind":"depends_on","provenance":"derived"},
        {"id":"turnstone-hosts-mere","source":"turnstone","target":"mere","kind":"host_for","provenance":"curated"}
      ]
    }"#;

    #[test]
    fn radial_projection_preserves_graph_identity() {
        let encoded = layout_graph_json(SAMPLE).expect("layout graph");
        let value: serde_json::Value = serde_json::from_str(&encoded).expect("parse layout");
        assert_eq!(value["engine"], "mere-arrangements/radial+unreachable-lane");
        assert_eq!(value["focus"], "mere");
        assert_eq!(value["nodes"].as_array().expect("nodes").len(), 3);
        assert_eq!(value["edges"].as_array().expect("edges").len(), 2);
        assert_eq!(value["nodes"][0]["id"], "mere");
        assert_eq!(value["edges"][1]["id"], "turnstone-hosts-mere");
    }

    #[test]
    fn radial_projection_is_deterministic() {
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
    fn unreachable_nodes_use_a_stable_lane() {
        let input = SAMPLE.replace(
            r#"{"id":"turnstone","name":"Turnstone","class":"product","status":"prototype"}"#,
            r#"{"id":"turnstone","name":"Turnstone","class":"product","status":"prototype"},
        {"id":"smolweb","name":"Smolweb","class":"foundation","status":"active"}"#,
        );
        let encoded = layout_graph_json(&input).expect("layout graph with unreachable node");
        let value: serde_json::Value = serde_json::from_str(&encoded).expect("parse layout");
        let smolweb = value["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| node["id"] == "smolweb")
            .expect("unreachable node");
        assert_eq!(smolweb["x"], -470.0);
        assert_eq!(smolweb["y"], 0.0);
    }
}
