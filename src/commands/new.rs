use std::path::PathBuf;
use std::fs;
use std::process::Command;

use crate::utils::{expand_tilde,dir_names};
use crate::app::App;

pub fn command_new(app: App, template_dir_ok: bool, project_name: String, template_name: String) {
    if template_dir_ok {
        let source_path = expand_tilde(&app.source_folder);
        let templates_name: Vec<String> = dir_names(source_path.clone());

        if templates_name.contains(&template_name) {
            let dir_names: Vec<String> = dir_names(PathBuf::from("."));

            if !dir_names.contains(&project_name) {
                let _ = fs::create_dir(&project_name);

                let files_name = ["main.typ","template.typ"];
                for file in files_name {
                    let source = source_path.join(&template_name).join(file);
                    let destination = PathBuf::from(&project_name).join(file);

                    let _ = fs::copy(&source, &destination);
                }
                Command::new("codium")
                        .arg(format!("./{}/",project_name))
                        .status()
                        .expect("Fail to launch VSCodium");

                println!("New project \"{}\" created successfully with the template \"{}\" !", project_name, template_name);

            } else {
                println!("This directory name already exists");
            }
        } else {
            println!("This template doesn't exist");
        }
    } else {
        println!("Please configure your template directory with : typsta config <path>");
    }
}
