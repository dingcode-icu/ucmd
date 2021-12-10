///静态帮助类
pub mod util;

///打包
pub mod pkg;

///透传暴露的lib
pub mod Ex {
    pub use walkdir;
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
///命令行模块
pub use clap;
pub use clap::{ArgMatches};


mod tests {
    use crate::util::filesys::zip_dir;
    use std::path::Path;
    use walkdir::{WalkDir, DirEntry};


    #[test]
    fn it_works() {
         use walkdir;
        let src_dir = "/Users/mac/data0/public_work/ucmd/proj/rcmd-core/test/ziptest";
        let pf = Path::new(src_dir);
        let zipfile = std::fs::File::create("test.zip").unwrap();
        let mut ws = WalkDir::new(pf);
        zip_dir(
            &mut ws.into_iter()
            .filter_entry(|e|ignore_entry(e))
            .filter_map(|v|v.ok()), pf.to_str().unwrap(), zipfile);
    }

    fn ignore_entry(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            //忽略.开头的文件
            .map(|s|!s.starts_with("."))
            .unwrap_or(false)
    }
}


