use std::{fs::File, process::Stdio};

fn main() {
    let outputs = File::create("C:/App/Plus/copia/out.txt").expect("Error on create file log");

    let process = std::process::Command::new("C:/Program Files/Firebird/Firebird_3_0/gbak.exe")
        .args(&[
            "-c",
            "c:/app/plus/copia/app.GBK",
            "localhost:c:/app/plus/copia/app.fdb",
            "-user",
            "SYSDBA",
            "-pass",
            "masterkey",
            "-v",
        ])
        // .spawn()
        .stdout(Stdio::from(outputs))
        //.stderr(Stdio::from(errors))
        .output()
        .expect("Erro para iniciar o processo de restauração");

    println!("status: {}", process.status);
    // println!("stdout: {}", String::from_utf8_lossy(&process.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&process.stderr));

    // if let Err(e) = process.wait_with_output() {
    //     println!("Erro no restore, {}", e);
    // }

    println!("Processo terminado");
}
