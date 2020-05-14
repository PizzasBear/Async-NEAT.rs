use super::Config;
use super::{Links, Nodes};
use std::future::Future;

pub struct Net {
    inputs_count: usize,
    outputs_count: usize,
    links: Links,
    nodes: Nodes,
    fitness: f64,
}

impl Net {
    pub async fn new<T>(inputs_count: usize, outputs_count: usize, conf: &Config<T>) -> Self
    where
        T: Future<Output = f64>,
    {
        let init_links_count = (inputs_count + 1) * outputs_count;
        let init_nodes_count = inputs_count + 1 + outputs_count;

        let mut out = Self {
            fitness: 0.0,
            inputs_count: inputs_count,
            outputs_count: outputs_count,
            links: Links {
                enabled: Vec::with_capacity(init_links_count),
                innovs: (0..init_links_count).collect::<Vec<usize>>(),
                weights: Vec::with_capacity(init_links_count),
            },
            nodes: Nodes {
                indices: (0..init_nodes_count).collect::<Vec<usize>>(),
                input_links: Vec::with_capacity(init_nodes_count),
            },
        };

        out.links.enabled.resize(init_links_count, true);

        for _ in 0..init_links_count {
            out.links.weights.push(conf.init_weight().await);
        }

        out.nodes.input_links.resize(inputs_count + 1, Vec::new());
        out.nodes
            .input_links
            .resize(init_nodes_count, Vec::with_capacity(inputs_count + 1));
        for i in 0..outputs_count {
            for j in 0..(inputs_count + 1) {
                out.nodes.input_links[i + inputs_count + 1].push(i * (inputs_count + 1) + j);
            }
        }

        out.fitness = conf.eval(&out).await;

        out
    }
}

impl Clone for Net {
    fn clone(&self) -> Self {
        Self {
            fitness: 0.0,
            inputs_count: self.inputs_count,
            outputs_count: self.outputs_count,
            links: self.links.clone(),
            nodes: self.nodes.clone(),
        }
    }
}
