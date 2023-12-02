use std::{
    fs::File,
    fs::{self},
    process::Stdio,
};

const Z_PATH: &str = r"C:\Program Files\7-Zip\7z.exe";

const GBAK: &str = r"C:\Program Files\Firebird\Firebird_3_0\gbak.exe";
const USER: &str = "SYSDBA";
const PASSWORD: &str = "masterkey";

const TEMP_PATH: &str = "C:/Users/dev/restore/temp";
const TO_RESTORE_PATH: &str = r"C:\Users\dev\restore\to_restore";
const RESTORED_PATH: &str = "C:/Users/dev/restore/restored";

fn main() {
    for file in get_all_files(&TO_RESTORE_PATH, "7Z") {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let path = file.to_str().unwrap();

        print!("Iniciando do restore {}", &file_name);

        unzip(path);

        if let Some(gbk) = get_all_files(&TEMP_PATH, "GBK").first() {
            let file_gbk = gbk.file_name().unwrap().to_str().unwrap();

            restore_database(&file_gbk);

            let file_ziped = format!("{}.7z", &file_name);
            zip(&file_ziped);

            fs::copy(
                format!("{}/{}", &TEMP_PATH, &file_ziped),
                format!("{}/{}", &RESTORED_PATH, &file_ziped),
            )
            .expect(&format!("Error on copy {}", &file_ziped));
        }

        fs::remove_dir_all(&TEMP_PATH).expect(&format!("Error on delete dir {}", &TEMP_PATH));
        fs::create_dir(&TEMP_PATH).expect(&format!("Error on create dir {}", &TEMP_PATH));

        fs::remove_file(&file).unwrap();

        print!("Término do restore {}", file_name);
    }
}

fn get_all_files(path: &str, ext: &str) -> Vec<std::path::PathBuf> {
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for element in std::path::Path::new(&path)
        .read_dir()
        .expect(&format!("Error on read files in {}", &path))
    {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension.to_ascii_uppercase() == *ext {
                files.push(path);
            }
        }
    }

    files
}

fn restore_database(backup_name: &str) {
    let log = format!("{}/log.txt", TEMP_PATH);
    let error_log = format!("{}/error.txt", TEMP_PATH);
    let backup = format!("{}/{}", TEMP_PATH, backup_name);
    let fdb = format!("localhost:{}/APPDATABASE.fdb", TEMP_PATH);

    let outputs = File::create(&log).expect(&format!("Error to create file {}", &log));
    let outputerror = File::create(&error_log).expect(&format!("Error to create {}", &error_log));

    println!("Iniciando restore do {}", backup_name);

    std::process::Command::new(GBAK)
        .args(&["-c", &backup, &fdb, "-user", USER, "-pass", PASSWORD, "-v"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(outputerror))
        .output()
        .expect(&format!("Error on restore database {}", backup_name));

    println!("Término restore do {}", backup_name);
}

fn unzip(file: &str) {
    std::process::Command::new(Z_PATH)
        .args(&["x", &format!("-o{}", &TEMP_PATH), file])
        .output()
        .expect(&format!("Error on unzip file {}", &file));
}

fn zip(name_zip: &str) {
    println!("Iniciando o zip");

    std::process::Command::new(Z_PATH)
        .args(&[
            "a",
            &format!("{}/{}", &TEMP_PATH, &name_zip),
            &format!("{}/APPDATABASE.fdb", &TEMP_PATH),
            &format!("{}/log.txt", &TEMP_PATH),
            &format!("{}/error.txt", &TEMP_PATH),
        ])
        .output()
        .expect(&format!("Error on zip file {}", &name_zip));

    println!("Término o zip");
}
