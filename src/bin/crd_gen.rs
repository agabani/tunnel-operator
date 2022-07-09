use kube::CustomResourceExt as _;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> tunnel_operator::Result<()> {
    let mut file = tokio::fs::File::create("helm/templates/crd.yaml").await?;

    let crd = tunnel_operator::operators::localtunnel::LocalTunnel::crd();
    let buffer = serde_yaml::to_vec(&crd)?;
    file.write_all(&buffer).await?;

    Ok(())
}
