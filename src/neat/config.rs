use serde::Deserialize;
use std::{future::Future, path::Path};
use tokio::{
    prelude::*,
    fs::File,
    io,
};

use super::Net;

#[derive(Debug, Deserialize)]
struct RawConfig {
    size: usize,
}

pub struct Config<T>
where
    T: Future<Output = f64>,
{
    pub size: usize,

    pub eval_fn: fn(&Net) -> T,
    pub init_weight_fn: fn() -> T,
}

impl<T> Config<T>
where
    T: Future<Output = f64>,
{
    pub async fn eval(&self, net: &Net) -> f64 {
        (self.eval_fn)(net).await
    }

    pub async fn init_weight(&self) -> f64 {
        (self.init_weight_fn)().await
    }
}

impl<T> Config<T>
where
    T: Future<Output = f64>,
{
    pub async fn new(
        eval_fn: fn(&Net) -> T,
        init_weight_fn: fn() -> T,
        path: impl AsRef<Path>,
    ) -> io::Result<Self> {
        let mut file = File::open(path).await?;
        let mut file_bytes = Vec::new();

        file.read_to_end(&mut file_bytes).await?;
        let raw = match ron::de::from_bytes::<RawConfig>(&file_bytes) {
            Ok(s) => s,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        };

        Ok(Self {
            size: raw.size,
            eval_fn: eval_fn,
            init_weight_fn: init_weight_fn,
        })
    }
}
