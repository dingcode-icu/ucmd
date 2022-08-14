mod subcmd;
use rcmd_core::clap::{load_yaml, App};

fn main() {
    
    // init logger
    init_logger();

    // The YAML file is found relative to the current file, similar to how modules are found
    let cli = load_yaml!("cli.yml");
    let mut app = App::from(cli);
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


#[cfg(debug_assertions)]
fn init_logger(){
    use rcmd_core::{fern, chrono};

    let _ = fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}] {}",
            chrono::Local::now().format("[%H:%M:%S]"),
            record.level(),
            message
        ))     
    })
    .level(log::LevelFilter::Debug)
    .chain(std::io::stdout())
    .apply();
}

#[cfg(not(debug_assertions))]
fn init_logger(){
    use rcmd_core::{chrono, fern, log};
    let log_path = std::env::current_dir().unwrap();
    let log_f = log_path.join(format!("{}.log",  chrono::Local::now().format("%Y_%m%d_%H%M")));
    let fern_f = fern::log_file(log_f).unwrap();

    let _ = fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}:{}",
            chrono::Local::now().format("[%H:%M:%S]"),
            message
        ))     
    })
    .level(log::LevelFilter::Info)
    .chain(std::io::stdout())
    .chain(fern_f)
    .apply();
}

#[test]
fn test_main(){
    use rcmd_core::log::{info, error};
    error!("test log file");
}