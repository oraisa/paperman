use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Add {
    #[structopt(parse(from_os_str))]
    pub bibtex: PathBuf,
    #[structopt(parse(from_os_str))]
    pub pdf: PathBuf,
    pub remaining_args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct BibtexCmd {
    #[structopt(parse(from_os_str))]
    pub bibtex: PathBuf,
    pub remaining_args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct ListCmd {
    pub field: String
}

#[derive(Debug, StructOpt)]
pub struct UpdateCmd {
    pub field: String,
    pub value: String
}
#[derive(Debug, StructOpt)]
pub struct DoiCmd {
    pub doi: String,
    pub remaining_args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct PickCmd {
    pub remaining_args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct ByCmd {
    pub field: String,
    pub value: String,
    pub remaining_args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub enum Command{
    /// Add selection
    Add,

    /// Remove selection
    Remove,

    /// Select all entries from BibTeX file
    Bibtex(BibtexCmd),

    /// Print selection as json
    Print,

    /// Print value of given field for selection 
    List(ListCmd),

    // Arxiv,

    /// Select paper by DOI
    Doi(DoiCmd),

    /// Open selected papers
    Open,

    /// Pick one or more selected papers from a menu
    Pick(PickCmd),

    /// Filter selected paper by given field and value
    By(ByCmd),

    // AddTag,

    // RemoveTag,

    /// Update the value of a field for selected papers
    Update(UpdateCmd),

    // Edit,
}

#[derive(Debug, StructOpt)]
pub struct Opt{
    #[structopt(subcommand)]
    pub command: Command,
}
