pub use super::prelude::*;

pub async fn run() -> MyResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Git(args) => git_push(args.token, &args.remote, &args.branch).await?,
        Commands::CratesIoBadges(mut args) => {
            let count = run_total_crates_io_downloads(&args.user).await?;
            args.count = Some(count);
            update_badge(&args).await?
        }
        Commands::TestsBadges(mut args) => {
            let count = count_tests_in_tests_folder()
                .expect("Count [#test]/#[tokio::test] in tests folder failed");
            args.count = Some(count);
            update_badge(&args).await?
        }
    }

    Ok(())
}
