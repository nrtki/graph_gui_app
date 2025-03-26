#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Manager;
use rand::Rng;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
    // Add other fields if needed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Edge {
    id: usize,
    source: usize,
    target: usize,
    // Add other fields if needed
}

#[derive(Default)]
struct GraphState {
    nodes: Mutex<Vec<Node>>,
    edges: Mutex<Vec<Edge>>,
    next_node_id: Mutex<usize>,
    next_edge_id: Mutex<usize>,
}

impl GraphState {
    fn clear(&self) {
        self.nodes.lock().unwrap().clear();
        self.edges.lock().unwrap().clear();
        *self.next_node_id.lock().unwrap() = 0;
        *self.next_edge_id.lock().unwrap() = 0;
    }
}

#[tauri::command]
fn add_node(state: tauri::State<GraphState>, x: f64, y: f64) -> Result<Node, String> {
    let mut nodes = state.nodes.lock().unwrap();
    let mut next_node_id = state.next_node_id.lock().unwrap();
    let node = Node {
        id: *next_node_id,
        x,
        y,
    };
    nodes.push(node.clone());
    *next_node_id += 1;
    Ok(node)
}

#[tauri::command]
fn add_edge(state: tauri::State<GraphState>, source: usize, target: usize) -> Result<Edge, String> {
    let mut edges = state.edges.lock().unwrap();
    let mut next_edge_id = state.next_edge_id.lock().unwrap();
    let nodes = state.nodes.lock().unwrap();
    if !nodes.iter().any(|n| n.id == source) || !nodes.iter().any(|n| n.id == target) {
        return Err("Source or target node not found".into());
    }
    let edge = Edge {
        id: *next_edge_id,
        source,
        target,
    };
    edges.push(edge.clone());
    *next_edge_id += 1;
    Ok(edge)
}

#[tauri::command]
fn update_node_position(state: tauri::State<GraphState>, node_id: usize, x: f64, y: f64) -> Result<(), String> {
    let mut nodes = state.nodes.lock().unwrap();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
        node.x = x;
        node.y = y;
        Ok(())
    } else {
        Err("Node not found".into())
    }
}

#[tauri::command]
fn get_graph(state: tauri::State<GraphState>) -> Result<(Vec<Node>, Vec<Edge>), String> {
    let nodes = state.nodes.lock().unwrap().clone();
    let edges = state.edges.lock().unwrap().clone();
    Ok((nodes, edges))
}

#[tauri::command]
fn delete_node(state: tauri::State<GraphState>, node_id: usize) -> Result<(), String> {
    let mut nodes = state.nodes.lock().unwrap();
    let mut edges = state.edges.lock().unwrap();

    if let Some(index) = nodes.iter().position(|n| n.id == node_id) {
        nodes.remove(index);
        edges.retain(|e| e.source != node_id && e.target != node_id);
        Ok(())
    } else {
        Err("Node not found".into())
    }
}

#[tauri::command]
fn delete_edge(state: tauri::State<GraphState>, edge_id: usize) -> Result<(), String> {
    let mut edges = state.edges.lock().unwrap();
    if let Some(index) = edges.iter().position(|e| e.id == edge_id) {
        edges.remove(index);
        Ok(())
    } else {
        Err("Edge not found".into())
    }
}

#[tauri::command]
fn clear_graph(state: tauri::State<GraphState>) {
    state.clear();
}

#[tauri::command]
fn generate_complete_graph(state: tauri::State<GraphState>, num_nodes: usize) -> Result<(), String> {
    let mut nodes = state.nodes.lock().unwrap();
    let mut edges = state.edges.lock().unwrap();
    let mut next_node_id = state.next_node_id.lock().unwrap();
    let mut next_edge_id = state.next_edge_id.lock().unwrap();

    nodes.clear();
    edges.clear();
    *next_node_id = 0;
    *next_edge_id = 0;

    for i in 0..num_nodes {
        nodes.push(Node {
            id: i,
            x: (i as f64) * 50.0, // Example positioning
            y: (i as f64) * 50.0,
        });
        *next_node_id += 1;
    }

    for i in 0..num_nodes {
        for j in (i + 1)..num_nodes {
            edges.push(Edge {
                id: *next_edge_id,
                source: i,
                target: j,
            });
            *next_edge_id += 1;
        }
    }

    Ok(())
}

#[tauri::command]
fn align_graph(state: tauri::State<GraphState>) -> Result<(), String> {
    let mut nodes = state.nodes.lock().unwrap();
    let num_nodes = nodes.len();
    if num_nodes == 0 {
        return Ok(());
    }

    let radius = 150.0; // 円の半径
    let center_x = 200.0; // 円の中心のX座標
    let center_y = 200.0; // 円の中心のY座標

    for (i, node) in nodes.iter_mut().enumerate() {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / num_nodes as f64);
        node.x = center_x + radius * angle.cos();
        node.y = center_y + radius * angle.sin();
    }

    Ok(())
}

#[tauri::command]
fn generate_random_graph(state: tauri::State<GraphState>, num_nodes: usize) -> Result<(), String> {
    let mut nodes = state.nodes.lock().unwrap();
    let mut edges = state.edges.lock().unwrap();
    let mut next_node_id = state.next_node_id.lock().unwrap();
    let mut next_edge_id = state.next_edge_id.lock().unwrap();

    nodes.clear();
    edges.clear();
    *next_node_id = 0;
    *next_edge_id = 0;

    let mut rng = rand::thread_rng();

    for i in 0..num_nodes {
        nodes.push(Node {
            id: i,
            x: rng.gen_range(0.0..400.0), // ランダムな位置
            y: rng.gen_range(0.0..400.0),
        });
        *next_node_id += 1;
    }

    for i in 0..num_nodes {
        for j in (i + 1)..num_nodes {
            if rng.gen_bool(0.5) { // ランダムにエッジを生成
                edges.push(Edge {
                    id: *next_edge_id,
                    source: i,
                    target: j,
                });
                *next_edge_id += 1;
            }
        }
    }

    Ok(())
}


fn main() {
    tauri::Builder::default()
        .manage(GraphState {
            nodes: Default::default(),
            edges: Default::default(),
            next_node_id: Mutex::new(0),
            next_edge_id: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            add_node,
            add_edge,
            update_node_position,
            get_graph,
            delete_node,
            delete_edge,
            clear_graph,
            generate_complete_graph,
            align_graph,
            generate_random_graph,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
