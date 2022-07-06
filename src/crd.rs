#[derive(
    Clone, Debug, kube::CustomResource, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[kube(
    kind = "Tunnel",
    group = "tunnel-operator.agabani.local",
    version = "v1",
    status = "TunnelStatus",
    namespaced
)]
pub struct TunnelSpec {}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TunnelStatus {}

pub(crate) static TUNNEL_FINALIZER: &str = "tunnel.tunnel-operator.agabani.local";

impl Tunnel {
    pub(crate) async fn reconcile(&self) -> Result<kube::runtime::controller::Action, kube::Error> {
        Ok(kube::runtime::controller::Action::requeue(
            std::time::Duration::from_secs(60 * 60),
        ))
    }

    pub(crate) async fn cleanup(&self) -> Result<kube::runtime::controller::Action, kube::Error> {
        Ok(kube::runtime::controller::Action::await_change())
    }
}
