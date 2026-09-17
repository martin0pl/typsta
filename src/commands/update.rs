use std::path::PathBuf;
use std::fs::{File,copy};
use std::io::{BufRead, BufReader};

use crate::utils::{file_names,expand_tilde};

pub fn command_update(template_dir_ok: bool, source_folder: String) {
    if template_dir_ok {
        let template_name = get_template_name();

        if template_name.is_empty() {
            println!("Your project doesn't contain a template file");
        }
        else {
            let source_path = expand_tilde(&source_folder);

            let source = source_path.join(&template_name).join("template.typ");
            let destination = PathBuf::from(".").join("template.typ");

            let _ = copy(&source, &destination);
        }

    } else {
        println!("Please configure your template directory with : typsta config <path>");
    }
}

fn get_template_name() -> String {
    if file_names(PathBuf::from(".")).contains(&"template.typ".to_string()) {
        let file = File::open("template.typ").expect("Impossible d'ouvrir le fichier");
        let reader = BufReader::new(file);

        let premiere_ligne = reader
            .lines()
            .next()
            .expect("The template file is empty")
            .expect("Error during reading");

        let template_name = premiere_ligne
            .trim_start()
            .trim_start_matches('/')
            .trim()
            .to_string();

        template_name
    }
    else {
        "".to_string()
    }
}
