pub mod render;
pub mod secrets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let secrets = secrets::get_secrets().await?;
    secrets::install_secrets(".secret".as_ref(), secrets.iter()).await?;

    render::copy_files(".secret".as_ref(), ".secret-2".as_ref()).await?;

    Ok(())
}
