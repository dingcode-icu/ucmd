use std::{io, fs};
use std::path::{Path, PathBuf};
use std::fs::{File};
use std::io::{Seek, Write, Read};
use zip::write::FileOptions;
use walkdir::DirEntry;


///拷贝当前文件夹内所有文件到指定文件夹
///##Exmample
/// ```
/// //todo
/// //copy_dir_all()
/// ```
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}


///压缩指定路径的所有文件到zip
///###Example:
///```
/// use rcmd_core::util::filesys::zip_dir;
/// use rcmd_core::Ex::walkdir::WalkDir;
///
/// let zipfile = std::fs::File::create(target).unwrap();
/// let dir  = WalkDir::new(src_dir);
/// zip_dir(&mut dir.into_iter().filter_map(|e|e.ok()), src_dir.to_str().unwrap(), zipfile);
///```
///
///
pub fn zip_dir<T>(it: &mut dyn Iterator<Item=DirEntry>, prefix: &str, writer: T) -> zip::result::ZipResult<()>
    where T: Write + Seek {
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)//直接用了bzip2压缩方式，其它参看枚举
        .unix_permissions(0o755);//unix系统权限

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        //zip压缩一个文件时，会把它的全路径当成文件名(在下面的解压函数中打印文件名可知)
        //这里是去掉目录前缀
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {//目录
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}