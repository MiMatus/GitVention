use clap::{Parser, Subcommand, Args as ClapArgs, ValueEnum};

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
    pub conventions: ConventionConfig,

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
pub struct GitConfig {
    /// absolute path to git repository /example/.git
    #[arg(long, short)]
    pub repository: Option<String>,
}
#[derive(ClapArgs, Debug)]
pub struct ConventionConfig {
    #[arg(long="title.match-regex", default_value_t=String::from(".*"))]
    pub title_regex: String,
    #[arg(long="title.max-length", default_value_t=72)]
    pub title_max_length: u32,
    #[arg(long="title.no-leading-whitespace", default_value_t=true)]
    pub title_no_leading_whitespace: bool,
}
pub enum Convention {
    TitleRegex(String),
    TitleMaxLength(u32),
    TitleNoLeadingWhitespace,
}

pub const CONVETION_TITLE_REGEX: &str = "title_regex";

impl ConventionConfig{
    pub fn title_conventions(&self) -> Vec<Convention>{
        let mut conventions = vec![
            Convention::TitleRegex(self.title_regex.to_owned()),
            Convention::TitleMaxLength(self.title_max_length),
        ];

        if self.title_no_leading_whitespace {
            conventions.push(Convention::TitleNoLeadingWhitespace)
        }

        return conventions
    }
}