use std::fmt;
use std::io::Result as IoResult;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;

pub trait Resolver: Send + Sync {
    fn resolve(&self, netloc: &str) -> IoResult<Vec<SocketAddr>>;
}

#[derive(Debug)]
pub(crate) struct StdResolver;

impl Resolver for StdResolver {
    fn resolve(&self, netloc: &str) -> IoResult<Vec<SocketAddr>> {
        ToSocketAddrs::to_socket_addrs(netloc).map(|iter| iter.collect())
    }
}

impl<F> Resolver for F
where
    F: Fn(&str) -> IoResult<Vec<SocketAddr>>,
    F: Send + Sync,
{
    fn resolve(&self, netloc: &str) -> IoResult<Vec<SocketAddr>> {
        self(netloc)
    }
}

#[derive(Clone)]
pub(crate) struct ArcResolver(Arc<dyn Resolver>);

impl<R> From<R> for ArcResolver
where
    R: Resolver + 'static,
{
    fn from(r: R) -> Self {
        Self(Arc::new(r))
    }
}

impl fmt::Debug for ArcResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArcResolver(...)")
    }
}

impl std::ops::Deref for ArcResolver {
    type Target = dyn Resolver;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl Default for ArcResolver {
    fn default() -> Self {
        StdResolver.into()
    }
}
