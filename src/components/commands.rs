use clap::Subcommand;

#[derive(clap::Args, Debug, Clone)]
pub struct Args {
    /// Badge file name
    #[arg(long, default_value = "pas2rust")]
    pub user: String,

    /// Badge file name
    #[arg(long, default_value = "tests.svg")]
    pub badge_name: String,

    #[arg(long, default_value = "badges")]
    pub destiny: String,

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
    #[arg(long, default_value = Some("testcafe"))]
    pub logo: Option<String>,

    /// Optional badge logo color (e.g., "white")
    #[arg(long, default_value = Some("white"))]
    pub logo_color: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    CratesIoBadges(Args),
    TestsBadges(Args),
}
