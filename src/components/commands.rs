use clap::Subcommand;

#[derive(clap::Args, Debug, Clone)]
pub struct Args {
    /// Crates.io username to query (e.g., pas2rust)
    #[arg(long, default_value = "pas2rust")]
    pub user: String,

    /// Token to use for the push (avoid; prefer env var)
    #[arg(long)]
    pub token: Option<String>,

    /// Git remote name
    #[arg(long, default_value = "origin")]
    pub remote: String,

    /// Branch to push
    #[arg(long, default_value = "master")]
    pub branch: String,

    /// Badge file name
    #[arg(long, default_value = "tests.svg")]
    pub badge_name: String,

    /// Badge label (e.g., "downloads")
    #[arg(long, default_value = "tests")]
    pub label: String,

    /// Count value to display in badge
    #[arg(long)]
    pub count: Option<u64>,

    /// Badge color
    #[arg(long, default_value = "blue")]
    pub color: String,

    /// Optional badge logo (e.g., "rust")
    #[arg(long)]
    pub logo: Option<String>,

    /// Url git repo destination
    #[arg(long, default_value = "https://github.com/pas2rust/badges.git")]
    pub url: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    CratesIoBadges(Args),
    TestsBadges(Args),
}
