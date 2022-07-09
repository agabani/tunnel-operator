pub mod localtunnel;

#[derive(Debug)]
enum Error {
    FinalizerError(kube::runtime::finalizer::Error<kube::Error>),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}
