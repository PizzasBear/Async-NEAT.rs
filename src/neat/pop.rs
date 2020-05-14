use super::Innovs;
use super::Net;
use std::future::Future;

pub struct Pop<T>
where
    T: Future<Output = f64>,
{
    innovs: Innovs,
    nets: Vec<Net<T>>,
}

impl<T> Pop<T>
where
    T: Future<Output = f64>,
{
    async fn new() {}
}
