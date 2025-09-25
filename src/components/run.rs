pub use super::prelude::*;

pub async fn run() -> MyResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CratesIoBadges(mut args) => {
            let count = run_total_crates_io_downloads(&args.user).await?;
            args.count = Some(count);
            update_badge(&args).await?
        }
        Commands::TestsBadges(mut args) => {
            let count = count_tests_in_tests_folder()
                .await
                .expect("Count [#test]/#[tokio::test] in tests folder failed");
            args.count = Some(count);
            update_badge(&args).await?
        }
    }

    Ok(())
}
