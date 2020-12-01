use crate::commands::*;
use crate::rofi_picker;
use crate::string_cleaner;
use crate::filter;
use crate::bibtex;
use json;
use std::path::PathBuf;
use std::io::Read;
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

    fn save_file_name() -> PathBuf {
        if cfg!(debug_assertions) {
            std::env::current_dir()
                .expect("Unable to access current dir")
                .join(PathBuf::from("db.json"))
        } else {
            std::env::current_dir()
                .expect("Unable to access home dir")
                .join(PathBuf::from("/.paperman/db.json"))
        }
    }

    fn save_db(&self) {
        let db_dir = App::save_file_name();
        let result = std::fs::write(db_dir, self.db.dump());
        match result {
            Ok(_) => return,
            Err(error) => panic!(format!("Could not save database file: {:?}", error))
        }
    }

    fn load_db() -> json::JsonValue{
        let db_dir = App::save_file_name();
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
            Command::BibtexFile(params) => self.bibtex_file(params),
            Command::Bibtex(params) => self.bibtex_input(params),
            Command::Export => self.export(),
            Command::Print => self.print(),
            Command::List(params) => self.list(params),
            Command::Update(params) => self.update(params),
            Command::By(params) => self.filter_by(params),
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

    fn bibtex_file(mut self, params: BibtexFileCmd) {
        let bibtex_string = std::fs::read_to_string(&params.bibtex).expect("Failed to read file");
        self.selection = bibtex::parse_bibtex(&bibtex_string);
        self.parse_remaining_args(params.remaining_args);
    }

    fn bibtex_input(mut self, params: BibtexInputCmd) {
        let mut bibtex_string = String::new();
        std::io::stdin().read_to_string(&mut bibtex_string).expect("Failed to read stdin");
        self.selection = bibtex::parse_bibtex(&bibtex_string);
        self.parse_remaining_args(params.remaining_args);
    }

    fn export(self) {
        print!("{}", bibtex::generate_bibtex(self.selection));
    }

    fn print(&self) {
        println!("{:#}", self.selection)
    }

    fn list(&self, params: ListCmd) {
        for (_key, paper) in self.selection.entries() {
            let decoded = string_cleaner::clean_string(paper[&params.field].as_str().unwrap());
            println!("{}", decoded);
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
        let selection = rofi_picker::pick(self.selection);
        self.selection = selection;
        self.parse_remaining_args(params.remaining_args);
    }

    fn filter_by(mut self, params: ByCmd) {
        let field = &params.field;
        let value = &json::from(params.value);
        let clean = match field.as_str() {
            "title" => true,
            "author" => true,
            _ => false
        };
        let to_remove = self.selection.entries()
            .filter(|(_, paper)| !filter::match_values(value, &paper[field], clean))
            .map(|(key, _)| key.to_string())
            .collect::<Vec<_>>();

        for key in to_remove {
            self.selection.remove(&key);
        }
        self.parse_remaining_args(params.remaining_args);
    }
}

