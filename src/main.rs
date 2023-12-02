use std::{
    fs::File,
    fs::{self},
    process::Stdio,
    thread,
    time::Duration,
};

const TEMP_PATH: &str = "./temp";
const TO_RESTORE_PATH: &str = "./to_restore";
const RESTORED_PATH: &str = "./restored";

fn main() {
    dotenv::dotenv().ok();
    let user = std::env::var("USER").expect(".env, USER not found");
    let gbk_exe = std::env::var("GBAK").expect(".env, GBAK not found");
    let z_path = std::env::var("Z_PATH").expect(".env, Z_PATH not found");
    let password = std::env::var("PASSWORD").expect(".env, PASSWORD not found");

    create_dir(&TEMP_PATH);
    create_dir(&TO_RESTORE_PATH);
    create_dir(&RESTORED_PATH);

    loop {
        println!("Verificando arquivos");
        for file in get_all_files(&TO_RESTORE_PATH, "7Z") {
            let file_name = file.file_name().unwrap().to_str().unwrap();
            let path = file.to_str().unwrap();

            print!("Iniciando do restore {}", &file_name);

            unzip(&z_path, path);

            if let Some(gbk) = get_all_files(&TEMP_PATH, "GBK").first() {
                let file_gbk = gbk.file_name().unwrap().to_str().unwrap();

                restore_database(&gbk_exe, &user, &password, &file_gbk);

                zip(&z_path, &file_name);

                fs::copy(
                    format!("{}/{}", &TEMP_PATH, &file_name),
                    format!("{}/{}", &RESTORED_PATH, &file_name),
                )
                .expect(&format!("Error on copy {}", &file_name));
            }

            fs::remove_dir_all(&TEMP_PATH).expect(&format!("Error on delete dir {}", &TEMP_PATH));
            fs::create_dir(&TEMP_PATH).expect(&format!("Error on create dir {}", &TEMP_PATH));

            fs::remove_file(&file).unwrap();

            print!("Término do restore {}", file_name);
        }

        thread::sleep(Duration::from_secs(10));
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

fn restore_database(gbk: &str, user: &str, password: &str, backup_name: &str) {
    let log = format!("{}/log.txt", TEMP_PATH);
    let error_log = format!("{}/error.txt", TEMP_PATH);
    let backup = format!("{}/{}", TEMP_PATH, backup_name);
    let fdb = format!("localhost:{}/APPDATABASE.fdb", TEMP_PATH);

    let outputs = File::create(&log).expect(&format!("Error to create file {}", &log));
    let outputerror = File::create(&error_log).expect(&format!("Error to create {}", &error_log));

    println!("Iniciando restore do {}", backup_name);

    std::process::Command::new(&gbk)
        .args(&[
            "-c", &backup, &fdb, "-user", &user, "-pass", &password, "-v",
        ])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(outputerror))
        .output()
        .expect(&format!("Error on restore database {}", backup_name));

    println!("Término restore do {}\n\r", backup_name);
}

fn unzip(z_path: &str, file: &str) {
    std::process::Command::new(&z_path)
        .args(&["x", &format!("-o{}", &TEMP_PATH), file])
        .output()
        .expect(&format!("Error on unzip file {}", &file));
}

fn zip(z_path: &str, name_zip: &str) {
    println!("Iniciando do zip");

    std::process::Command::new(&z_path)
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

fn create_dir(dir: &str) {
    if !fs::read_dir(&dir).is_ok() {
        fs::create_dir(&dir).expect("Error on create temp dir");
    }
}
