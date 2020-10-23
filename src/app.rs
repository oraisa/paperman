use crate::commands::*;
use json;
use nom_bibtex::Bibtex;
use std::path::PathBuf;
use structopt::clap;
use structopt::StructOpt;

pub struct App {
    db: json::JsonValue,
    selection: json::JsonValue,
}

impl App {
    fn new() -> App {
        let db = App::load_db();
        let selection = db.clone();
        App {
            db: db, selection: selection
        }
    }

    pub fn run() {
        let app = App::new();
        let args = Opt::from_args();
        app.match_command(args.command)
    }

    fn save_db(&self) {
        let db_dir = PathBuf::from("/home/ossi/.paperman/db.json");
        let result = std::fs::write(db_dir, self.db.dump());
        match result {
            Ok(_) => return,
            Err(error) => panic!(format!("Could not save database file: {:?}", error))
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

    fn match_command(self, command: Command) {
        match command {
            Command::Add => self.add(),
            Command::Remove => self.remove(),
            Command::Bibtex(params) => self.bibtex(params),
            Command::Print => self.print(),
            Command::List(params) => self.list(params),
            Command::Update(params) => self.update(params),
            Command::By(params) => self.filter_by(params),
            Command::Doi(params) => self.doi(params),
            Command::Open => self.open(),
            Command::Pick(params) => self.pick(params),
        }
    }

    fn parse_remaining_args(self, remaining_args: Vec<String>) {
        let app = Opt::clap().setting(clap::AppSettings::NoBinaryName);
        let matches = app.get_matches_from(remaining_args);
        let new_args = Opt::from_clap(&matches);
        // println!("{:?}", new_args);
        self.match_command(new_args.command);
    }

    fn add(mut self) {
        println!("Adding");
        println!("{:#}", self.selection);
        //TODO: confirm
        for (key, paper) in self.selection.entries() {
            self.db[key] = paper.clone()
        }
        self.save_db()
    }

    fn remove(mut self) {
        println!("Removing");
        println!("{:#}", self.selection);
        //TODO: confirm
        for (key, _) in self.selection.entries() {
            self.db.remove(key);
        }
        self.save_db()
    }

    fn update(mut self, params: UpdateCmd) {
        let json_value = json::from(params.value);
        for (key, _) in self.selection.entries() {
            self.db[key][&params.field] = json_value.clone();
        }
        self.save_db()
    }

    fn bibtex(mut self, params: BibtexCmd) {
        let bibtex_string = std::fs::read_to_string(&params.bibtex).unwrap();
        let bibtex = Bibtex::parse(&bibtex_string).unwrap();
        let mut new_selection = json::object!{};
        for biblio in bibtex.bibliographies(){
            let key = biblio.citation_key();
            let paper_object = App::parse_paper(&biblio);
            new_selection[key] = paper_object;
            
        }
        self.selection = new_selection;
        self.parse_remaining_args(params.remaining_args);
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

    fn print(&self) {
        println!("{:#}", self.selection)
    }

    fn list(&self, params: ListCmd) {
        for (_key, paper) in self.selection.entries() {
            println!("{}", paper[&params.field])
        }
    }

    fn open(&self) {
        for (_, paper) in self.selection.entries(){
            let file_name = paper["file"].clone();
            println!("Opening {}", &file_name);
            std::process::Command::new("xdg-open")
                .arg(file_name.as_str()
                .unwrap()).spawn()
                .expect("Failed to open file.");
        }
    }

    fn pick(mut self, params: PickCmd) {
        self.parse_remaining_args(params.remaining_args);
    }

    fn filter_by(mut self, params: ByCmd) {
        let field = &params.field;
        let value = &json::from(params.value);
        let to_remove = self.selection.entries()
            .filter(|(_, paper)| !App::match_values(value, &paper[field]))
            .map(|(key, _)| key.to_string())
            .collect::<Vec<_>>();

        for key in to_remove {
            self.selection.remove(&key);
        }
        self.parse_remaining_args(params.remaining_args);
    }

    fn match_values(value: &json::JsonValue, field_value: &json::JsonValue) -> bool {
        match value {
            json::JsonValue::String(str_val) => {
                match field_value {
                    json::JsonValue::String(field_str) => field_str.contains(str_val),
                    json::JsonValue::Array(field_arr) => field_arr
                        .iter().any(|s| s.as_str().unwrap().contains(str_val)),
                    _ => false
                }
            },
            _ => false,
        }
    }

    fn doi(mut self, params: DoiCmd) {
        self.parse_remaining_args(params.remaining_args);
    }
}

