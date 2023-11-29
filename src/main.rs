use std::{fs::File, process::Stdio};

const GBAK: &str = r"C:\Program Files\Firebird\Firebird_3_0\gbak.exe";
const USER: &str = "SYSDBA";
const PASSWORD: &str = "masterkey";
const TEMP_PATH: &str = "C:/app/plus/copia/temp";
const TO_RESTORE_PATH: &str = r"C:\App\Plus\copia\to_restore";

fn main() {
    // restore_database("APP.GBK");

    let mut faxvec: Vec<std::path::PathBuf> = Vec::new();
    for element in std::path::Path::new(TO_RESTORE_PATH)
        .read_dir()
        .expect(&format!("Error on read files in {}", TO_RESTORE_PATH))
    {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "txt" {
                faxvec.push(path);
            }
        }
    }
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
        // .spawn()
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(outputerror))
        .output()
        .expect(&format!("Error on restore database {}", backup_name));

    println!("Término restore do {}", backup_name);

    // println!("stdout: {}", String::from_utf8_lossy(&process.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&process.stderr));
}
