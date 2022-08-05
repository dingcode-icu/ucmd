mod subcmd;
use rcmd_core::clap::{load_yaml, App};
use rcmd_core::Log::{debug, init_logger, LevelFilter};

fn main() {
    // init logger
    let _ = init_logger(if cfg!(debug_assertions) {
        Some(LevelFilter::Debug)
    } else {
        Some(LevelFilter::Info)
    });
    debug!("Init logger ");
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let mut app = App::from(yaml);
    let app_m = app.clone().get_matches();

    match app_m.subcommand() {
        Some((external, sub_m)) => match external {
            "init" => {
                subcmd::gen_conf::handle(sub_m);
            }
            "build-player" => {
                subcmd::build_player::handle(sub_m);
            }
            "build-ab" => {
                subcmd::build_ab::handle(sub_m);
            }
            _ => {
                app.print_help().unwrap();
            }
        },
        _ => {
            app.print_help().unwrap();
        }
    }
}