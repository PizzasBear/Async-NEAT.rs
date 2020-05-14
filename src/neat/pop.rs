use super::{Config, Innovs, Net};
use std::future::Future;

pub struct Pop {
    innovs: Innovs,
    nets: Vec<Net>,
}

impl Pop {
    async fn new<T>(input_size: usize, output_size: usize, conf: &Config<T>) -> Self
    where
        T: Future<Output = f64>,
    {
        let init_links_count = (input_size + 1) * output_size;
        let init_nodes_count = input_size + 1 + output_size;

        let mut out = Self {
            innovs: Innovs {
                offset: init_nodes_count,
                from_innovs: Vec::with_capacity(init_links_count),
                to_innovs: Vec::with_capacity(init_links_count),
            },
            nets: Vec::with_capacity(conf.size),
        };

        for i in (input_size + 1)..init_nodes_count {
            for j in 0..(input_size + 1) {
                out.innovs.from_innovs.push(j);
                out.innovs.to_innovs.push(i);
            }
        }

        for _ in 0..conf.size {
            out.nets.push(Net::new(input_size, output_size, conf).await);
        }

        out
    }
}
