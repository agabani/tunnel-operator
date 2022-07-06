use futures::{FutureExt as _, StreamExt as _};
use kube::ResourceExt as _;

use crate::crd::{Tunnel, TUNNEL_FINALIZER};

struct Data {
    client: kube::Client,
}

#[derive(Debug)]
enum Error {
    FinalizerError(kube::runtime::finalizer::Error<kube::Error>),
}

pub(crate) struct Operator {}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}

impl Operator {
    pub(crate) async fn new() -> crate::Result<(Operator, impl std::future::Future<Output = ()>)> {
        let client = kube::Client::try_default().await?;
        let context = std::sync::Arc::new(Data {
            client: client.clone(),
        });

        let tunnels = kube::Api::<Tunnel>::all(client);

        tunnels
            .list(&kube::api::ListParams::default().limit(1))
            .await?;

        let drainer = kube::runtime::Controller::new(tunnels, kube::api::ListParams::default())
            .run(Operator::reconciler, Operator::error_policy, context)
            .filter_map(|future| async move { Result::ok(future) })
            .for_each(|_| futures::future::ready(()))
            .boxed();

        Ok((Operator {}, drainer))
    }

    async fn reconciler(
        tunnel: std::sync::Arc<Tunnel>,
        ctx: std::sync::Arc<Data>,
    ) -> Result<kube::runtime::controller::Action, Error> {
        let name = tunnel.name();
        let namespace = tunnel.namespace();

        let api: kube::Api<Tunnel> = match &namespace {
            Some(namespace) => kube::Api::namespaced(ctx.client.clone(), namespace),
            None => kube::Api::all(ctx.client.clone()),
        };

        let action = kube::runtime::finalizer(&api, TUNNEL_FINALIZER, tunnel, |event| async {
            match event {
                kube::runtime::finalizer::Event::Apply(tunnel) => tunnel.reconcile().await,
                kube::runtime::finalizer::Event::Cleanup(tunnel) => tunnel.cleanup().await,
            }
        })
        .await
        .map_err(Error::FinalizerError);

        tracing::info!(?name, ?namespace, "reconciliation succeeded");
        action
    }

    #[allow(clippy::needless_pass_by_value)]
    fn error_policy(
        error: &Error,
        _ctx: std::sync::Arc<Data>,
    ) -> kube::runtime::controller::Action {
        tracing::error!(?error, "reconciliation failed");
        kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(1))
    }
}
