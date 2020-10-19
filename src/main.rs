mod paper;
mod database;

use json;
use nom_bibtex::Bibtex;
use structopt::clap;
use structopt::StructOpt;
use std::path::PathBuf;
// use crate::paper::Paper;
// use crate::database::Database;

#[derive(Debug, StructOpt)]
struct Add {
    #[structopt(parse(from_os_str))]
    bibtex: PathBuf,
    #[structopt(parse(from_os_str))]
    pdf: PathBuf,
    remaining_args: Vec<String>,
}

#[derive(Debug, StructOpt)]
struct BibtexCmd {
    #[structopt(parse(from_os_str))]
    bibtex: PathBuf,
    remaining_args: Vec<String>,
}

#[derive(Debug, StructOpt)]
struct ListCmd {
    field: String
}

#[derive(Debug, StructOpt)]
enum Command{
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
}

#[derive(Debug, StructOpt)]
struct Opt{
    #[structopt(subcommand)]
    command: Command,
}

struct State {
    db: json::JsonValue,
    selection: json::JsonValue,
}


fn main() {
    let args = Opt::from_args();
    // println!("{:?}", args);
    let db = load_db();
    match_command(State{ selection: db.clone(), db: db }, args.command);
}

fn match_command(state: State, command: Command) {
    match command {
        Command::Add => add(state),
        Command::Remove => remove(state),
        Command::Bibtex(params) => bibtex(state, params),
        Command::Print => print(state),
        Command::List(params) => list(state, params)
    }
}

fn parse_remaining_args(state: State, remaining_args: Vec<String>) {
    let app = Opt::clap().setting(clap::AppSettings::NoBinaryName);
    let matches = app.get_matches_from(remaining_args);
    let new_args = Opt::from_clap(&matches);
    // println!("{:?}", new_args);
    match_command(state, new_args.command);
}

fn add(state: State) {
    println!("Adding");
    println!("{:#}", state.selection);
    //TODO: confirm
    let mut new_db = state.db;
    for (key, paper) in state.selection.entries() {
        new_db[key] = paper.clone()
    }
    save_db(new_db)
}

fn remove(state: State) {
    println!("Removing");
    println!("{:#}", state.selection);
    //TODO: confirm
    let mut new_db = state.db;
    for (key, paper) in state.selection.entries() {
        new_db.remove(key);
    }
    save_db(new_db)
}

fn save_db(db: json::JsonValue) {
    let db_dir = PathBuf::from("/home/ossi/.paperman/db.json");
    let result = std::fs::write(db_dir, db.dump());
    match result {
        Ok(_) => return,
        Err(error) => panic!(format!("Could not save database file: {:?}", error))
    }
}

fn bibtex(state: State, params: BibtexCmd) {
    let bibtex_string = std::fs::read_to_string(&params.bibtex).unwrap();
    let bibtex = Bibtex::parse(&bibtex_string).unwrap();
    let mut new_state = State{ selection: state.selection, .. state };
    for biblio in bibtex.bibliographies(){
        let key = biblio.citation_key();
        let paper_object = parse_paper(&biblio);
        new_state.selection[key] = paper_object;
        
    }
    parse_remaining_args(new_state, params.remaining_args);
}

fn parse_paper(biblio: &nom_bibtex::Bibliography) -> json::JsonValue {
    let mut paper_object = json::object!{};

    let entry_type = biblio.entry_type();
    paper_object["entry_type"] = json::from(entry_type);

    for (key, value) in biblio.tags(){
        paper_object[key] = json::from(value.clone());
    }
    paper_object
}

fn print(state: State) {
    println!("{:#}", state.selection)
}

fn list(state: State, params: ListCmd) {
    for (_key, paper) in state.selection.entries() {
        println!("{}", paper[&params.field])
    }
}

fn load_db() -> json::JsonValue{
    let db_dir = PathBuf::from("/home/ossi/.paperman/db.json");
    let json_result = std::fs::read_to_string(&db_dir);
    match json_result {
        Ok(json) => json::parse(&json).unwrap(),
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => json::object!{},
                _ => panic!(format!("Could not open database file: {:?}", error))
            }
        }
    }
}
