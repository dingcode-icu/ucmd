///静态帮助类
pub mod util;

///打包
pub mod pkg;

///透传暴露的lib
pub mod Ex {
    pub use chrono;
}
///日志模块
pub mod Log {
    pub use log::{*};
    pub fn init_logger() -> Result<(), fern::InitError> {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(std::io::stdout())
            // .chain(fern::log_file("output.log")?)
            .apply()?;
        Ok(())
    }
}

pub use clap;
pub use clap::{ArgMatches};

mod tests {
    #[test]
    fn it_works() {
        let resp = Get("http://www.baidu.com");
        println!("test here111");
    }
}


