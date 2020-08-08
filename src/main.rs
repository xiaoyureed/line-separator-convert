use std::io::Write;
use std::io::Read;
use std::fs;

fn main() {
    handling();
}

fn handling() {
    let mut args = std::env::args();
    args.next().unwrap();//指针移动到第一个参数位置, 此时指向命令本身

    // file name
    let file_name = args.next().unwrap();// 真实的第一个参数

    let is_file = fs::metadata(&file_name).unwrap().is_file();
    if is_file {
        exec_on_file(&file_name);
    } else {// input param is a folder
        // 遍历处理
        exec_on_folder(&file_name);
    }


}

fn exec_on_folder(folder_name: &str) {
    let paths: fs::ReadDir = fs::read_dir(folder_name).unwrap();
    for path in paths {
        //        println!("Name: {}", path.unwrap().path().display())
        let path: std::path::PathBuf = path.unwrap().path();
        let is_file = fs::metadata(&path).unwrap().is_file();
        if is_file {
            exec_on_file(path.to_str().unwrap());
        } else {
            exec_on_folder(path.to_str().unwrap());
        }
    }
}


fn exec_on_file(file_name: &str) {
    let mut source_file = fs::File::open(file_name).unwrap();
    let mut tmp = String::new();
    source_file.read_to_string(&mut tmp).unwrap();

    fs::remove_file(file_name).unwrap();

    let tmp = tmp.replace("\r\n", "\n");

    let mut target_file = fs::File::create(file_name).unwrap();
    target_file.write_all(tmp.as_bytes()).unwrap();
}