use benches::{extend_pyth, report::BenchmarkReport};
use futures::FutureExt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let report = futures::future::try_join_all([extend_pyth::bench().boxed()])
        .await?
        .into_iter()
        .fold(BenchmarkReport::default(), BenchmarkReport::merge_with);

    println!();
    println!("{report}");

    Ok(())
}
