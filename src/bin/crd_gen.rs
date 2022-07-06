use kube::CustomResourceExt as _;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> tunnel_operator::Result<()> {
    let crd = tunnel_operator::crd::Tunnel::crd();
    let buffer = serde_yaml::to_vec(&crd)?;
    let mut file = tokio::fs::File::create("helm/templates/crd.yaml").await?;
    file.write_all(&buffer).await?;
    Ok(())
}
