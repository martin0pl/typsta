use crate::app::App;

pub fn command_config(mut app: App, path: Option<String>, config_file_name:  &str, template_dir_ok: bool) {
    match path {
        Some(new_path) => {
            app.source_folder = new_path;
            app.save(config_file_name);
            println!("Source folder set to : {}", app.source_folder);
        }
        None => {
            if template_dir_ok {
                println!("Current source folder : {}", app.source_folder);
            } else {
                println!("Please configure your template directory with : typsta config <path>");
            }
        }
    }
}
