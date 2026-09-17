use crate::utils::{expand_tilde,templates_names};
use crate::app::App;

pub fn command_list(app: App,template_dir_ok: bool) {
    if template_dir_ok {
        let source_path = expand_tilde(&app.source_folder);
        let templates_name: Vec<String> = templates_names(source_path);

        for name in &templates_name {
            println!("{name}");
        }
    } else {
        println!("Please configurate your template directory with : typsta config <path>");
    }
}
