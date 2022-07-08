use std::sync::Arc;

use futures::{FutureExt as _, StreamExt as _};
use kube::ResourceExt as _;

struct Context {
    client: kube::Client,
}

#[derive(Debug)]
enum Error {
    FinalizerError(kube::runtime::finalizer::Error<kube::Error>),
}

pub(crate) struct Operator {
    _context: Arc<Context>,
}

#[derive(
    Clone, Debug, kube::CustomResource, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[kube(
    kind = "Tunnel",
    group = "tunnel-operator.agabani",
    version = "v1",
    status = "Status",
    namespaced
)]
pub struct Spec {}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Status {}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}

impl Operator {
    /// Create a new `TunnelOperator` instance.
    pub(crate) async fn new() -> crate::Result<(Operator, impl std::future::Future<Output = ()>)> {
        let client = kube::Client::try_default().await?;
        let context = std::sync::Arc::new(Context {
            client: client.clone(),
        });

        let tunnels = kube::Api::<Tunnel>::all(client.clone());

        tunnels
            .list(&kube::api::ListParams::default().limit(1))
            .await?;

        let drainer = kube::runtime::Controller::new(tunnels, kube::api::ListParams::default())
            .run(
                Operator::reconciler,
                Operator::error_policy,
                context.clone(),
            )
            .filter_map(|future| async move { Result::ok(future) })
            .for_each(|_| futures::future::ready(()))
            .boxed();

        Ok((Operator { _context: context }, drainer))
    }

    async fn reconciler(
        tunnel: std::sync::Arc<Tunnel>,
        context: std::sync::Arc<Context>,
    ) -> Result<kube::runtime::controller::Action, Error> {
        let name = tunnel.name();
        let namespace = tunnel.namespace();

        let api: kube::Api<Tunnel> = match &namespace {
            Some(namespace) => kube::Api::namespaced(context.client.clone(), namespace),
            None => kube::Api::all(context.client.clone()),
        };

        let action = kube::runtime::finalizer(
            &api,
            "tunnel.tunnel-operator.agabani",
            tunnel,
            |event| async {
                match event {
                    kube::runtime::finalizer::Event::Apply(_tunnel) => {
                        Ok(kube::runtime::controller::Action::requeue(
                            std::time::Duration::from_secs(60 * 60),
                        ))
                    }
                    kube::runtime::finalizer::Event::Cleanup(_tunnel) => {
                        Ok(kube::runtime::controller::Action::await_change())
                    }
                }
            },
        )
        .await
        .map_err(Error::FinalizerError);

        tracing::info!(?name, ?namespace, "reconciliation succeeded");
        action
    }

    #[allow(clippy::needless_pass_by_value)]
    fn error_policy(
        error: &Error,
        _context: std::sync::Arc<Context>,
    ) -> kube::runtime::controller::Action {
        tracing::error!(?error, "reconciliation failed");
        kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(1))
    }
}
