use clap::{Parser, Subcommand, Args as ClapArgs};

#[derive(Parser, Debug)]
#[command(name = "GitVention")]
#[command(author = "Michal M. <matusmichal2@gmail.com>")]
#[command(version = "0.1.0")]
#[command(long_about = None)]
pub struct Args {
    #[command(flatten)]
    pub git_config: GitConfig,

    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    title_rules: TitleRules


}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// validate
    Validate(ValidationArgs),
}


#[derive(ClapArgs, Debug)]
pub struct ValidationArgs {
    /// auto inc major
   pub target: String,
}

#[derive(ClapArgs, Debug)]
pub struct TitleRules {
    #[arg(long="title.regex")]
    regex: Option<String>
}

#[derive(ClapArgs, Debug)]
pub struct GitConfig {
    /// absolute path to git repository /example/.git
    #[arg(long, short)]
    pub repository: Option<String>,
}