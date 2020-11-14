use nom_bibtex::Bibtex;

pub fn generate_bibtex(selection: json::JsonValue) -> String {
    let mut result = String::new();
    for (citation_key, paper) in selection.entries() {
        result.push_str(&format!("@{}{{{},\n", paper["entry_type"], citation_key));
        for (key, value) in paper.entries() {
            if key != "entry_type" {
                result.push_str(&format!("    {} = {{{}}},\n", key, value));
            }
        }
    }
    result.push_str("}\n");
    return result;
}

pub fn parse_bibtex(bibtex_string: &str) -> json::JsonValue {
    let bibtex = Bibtex::parse(&bibtex_string).expect("Failed to parse bibtex");
    let mut new_selection = json::object!{};
    for biblio in bibtex.bibliographies(){
        let key = biblio.citation_key();
        let paper_object = parse_paper(&biblio);
        new_selection[key] = paper_object;
    }
    return new_selection
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
