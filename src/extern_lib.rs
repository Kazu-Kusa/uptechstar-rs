use libloading::Library;
use std::io::Write;
use tempfile::NamedTempFile;

use once_cell::sync::Lazy;


pub(crate) static LIBRARY: Lazy<Library> = Lazy::new(|| unsafe {
    // 步骤 1: 从资源中读取 .so 的字节
    let so_bytes = include_bytes!("../lib/libuptech.so");

    // 步骤 2: 创建一个临时文件，并写入 .so 内容
    let mut tmp_file: NamedTempFile = NamedTempFile::new().expect("Failed to create temp file");
    tmp_file.write_all(so_bytes).expect("Failed to write .so to temp file");

    // 步骤 3: 获取临时文件路径
    let so_path = tmp_file.into_temp_path();

    // 步骤 4: 加载 .so 库
    Library::new(so_path.as_os_str()).expect("Failed to load library")
});


