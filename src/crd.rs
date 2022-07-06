#[derive(
    Clone, Debug, kube::CustomResource, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[kube(
    kind = "Tunnel",
    group = "tunnel-operator.agabani.local",
    version = "v1",
    status = "TunnelStatus"
)]
pub struct TunnelSpec {}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TunnelStatus {}
