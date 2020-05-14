use super::{Config, Innovs, Links, Nodes};
use std::{collections::HashMap, future::Future, iter::FromIterator};
use tokio::io;
use rand::{prelude::*, thread_rng};
use rand_distr::Uniform;

pub struct Net {
    inputs_count: usize,
    outputs_count: usize,
    links: Links,
    nodes: Nodes,
    nodes_innov_to_idx: HashMap<usize, usize>,
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
                innovs: (init_nodes_count..(init_nodes_count + init_links_count))
                    .collect::<Vec<usize>>(),
                weights: Vec::with_capacity(init_links_count),
            },
            nodes: Nodes {
                indices: (0..init_nodes_count).collect::<Vec<usize>>(),
                input_links: Vec::with_capacity(init_nodes_count),
            },
            nodes_innov_to_idx: HashMap::from_iter((0..init_nodes_count).enumerate()),
        };

        // links gen
        out.links.enabled.resize(init_links_count, true);

        for _ in 0..init_links_count {
            out.links.weights.push(conf.init_weight().await);
        }

        // nodes
        out.nodes.input_links.resize(inputs_count + 1, Vec::new());
        out.nodes
            .input_links
            .resize(init_nodes_count, Vec::with_capacity(inputs_count + 1));
        for i in 0..outputs_count {
            for j in 0..(inputs_count + 1) {
                out.nodes.input_links[i + inputs_count + 1].push(i * (inputs_count + 1) + j);
            }
        }

        // fitness
        out.fitness = conf.eval(&out).await;

        out
    }

    async fn add_link(
        &mut self,
        from_innov: usize,
        to_innov: usize,
        weight: f64,
        old_innovs_len: usize,
        innovs: &mut Innovs,
    ) -> io::Result<()> {
        // let from_idx = self.nodes_innov_to_idx[&from];
        let to_idx = self.nodes_innov_to_idx[&to_innov];

        for i in self.nodes.input_links[to_idx].iter() {
            if innovs.from_innovs[self.links.innovs[*i]] == from_innov {
                return if !self.links.enabled[*i] {
                    self.links.weights[*i] = weight;
                    self.links.enabled[*i] = true;
                    Ok(())
                } else {
                    Err(io::Error::from(io::ErrorKind::AlreadyExists))
                };
            }
        }

        let mut innov = innovs.from_innovs.len();

        for i in old_innovs_len..innovs.from_innovs.len() {
            if innovs.from_innovs[i] == from_innov && innovs.to_innovs[i] == to_innov {
                innov = i;
                break;
            }
        }

        if innov == innovs.from_innovs.len() {
            innovs.from_innovs.push(from_innov);
            innovs.to_innovs.push(to_innov);
        }

        self.links.enabled.push(true);
        self.links.innovs.push(innov);
        self.links.weights.push(weight);

        Ok(())
    }

    async fn mutate_link<T>(
        &mut self,
        conf: &Config<T>,
        old_innovs_len: usize,
        innovs: &mut Innovs,
    ) -> io::Result<()>
    where
        T: Future<Output = f64>,
    {
        let mut rng = thread_rng();
        let mut from_idx = Uniform::new(0, self.nodes.len() - self.outputs_count).sample(&mut rng);
        let mut to_idx;
        
        if self.inputs_count <= from_idx {
            from_idx += self.outputs_count;
            to_idx = Uniform::new(self.inputs_count, self.nodes.len() - 1).sample(&mut rng);
            if from_idx <= to_idx { to_idx += 1; }
        }
        else {
            to_idx = Uniform::new(self.inputs_count, self.nodes.len()).sample(&mut rng);
        }

        Ok(())
    }

    /// Checks if adding a link from `from` to `to` will creates cycles, and therefore makes the network unevaluable.
    pub async fn creates_cycles(&self, from: usize, to: usize, innovs: &Innovs) -> bool {
        let mut visited_nodes = vec![to];
        loop {
            let mut newly_visited_nodes_count = 0;
            for innov in self.links.innovs.iter() {
                let link_from = innovs.from_innovs[*innov];
                let link_to = innovs.to_innovs[*innov];

                if visited_nodes.contains(&link_from) && !visited_nodes.contains(&link_to) {
                    if link_to == from {
                        return true;
                    }

                    visited_nodes.push(link_to);
                    newly_visited_nodes_count += 1;
                }
            }

            if newly_visited_nodes_count == 0 {
                return false;
            }
        }
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
            nodes_innov_to_idx: self.nodes_innov_to_idx.clone(),
        }
    }
}
