use super::{links::Links, nodes::Nodes};

pub struct Net {
    inputs_count: usize,
    outputs_count: usize,
    eval: fn(&Self) -> f64,
    links: Links,
    nodes: Nodes,
}

impl Net {
    fn new(inputs_count: usize, outputs_count: usize, eval: fn(&Self) -> f64) -> Self {
        let init_links_count = (inputs_count + 1) * outputs_count;
        let init_nodes_count = inputs_count + 1 + outputs_count;

        let mut out = Self {
            inputs_count: inputs_count,
            outputs_count: outputs_count,
            eval: eval,
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
        out.nodes.input_links.resize(inputs_count + 1, Vec::new());
        out.nodes
            .input_links
            .resize(init_nodes_count, Vec::with_capacity(inputs_count + 1));
        for i in (inputs_count + 1)..init_nodes_count {
            for j in 0..(inputs_count + 1) {
                out.nodes.input_links[i].push(i * (inputs_count + 1) + j);
            }
        }

        out
    }
}

impl Clone for Net {
    fn clone(&self) -> Self {
        Self {
            inputs_count: self.inputs_count,
            outputs_count: self.outputs_count,
            eval: self.eval,
            links: self.links.clone(),
            nodes: self.nodes.clone(),
        }
    }
}
