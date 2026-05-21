use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};
use std::{fs};
use std::io::{Write};

struct TestResult {
    status: ExitStatus,
    out: String,
    err: String,
    duration: Duration,
}

fn set_data(i_path: &str) -> Result<(), String> {
    // use windows command
    let output = Command::new("tar")
        .args([
            "-xf", i_path, 
        ])
        .output()
        .unwrap();
    
    if output.status.success() {
        let _ = fs::remove_file(i_path);
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn get_dir(path: &str) -> Vec<std::path::PathBuf> {
    let dir = fs::read_dir(path).unwrap();
    let mut files = Vec::new();
    for item in dir.into_iter() {
        let file_path = match item {
            Ok(direntry) => direntry.path(),
            Err(_) => continue,
        };
        if file_path.extension().and_then(|s| s.to_str()) == Some("in") {
            files.push(file_path);
        }
    }
    files.sort();
    files
}

fn _compile_cpp(cpp: &str) -> Result<PathBuf, String> {
    let output = Command::new("g++")
        .args([cpp, "-o", "abc"])
        .output() // run
        .unwrap();

    if output.status.success() {
        Ok(PathBuf::from("./abc"))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn compile_rs(rs: &str) -> Result<PathBuf, String> {
    let output = Command::new("rustc")
        .args([rs, "-O", "-o", "abc.exe"])
        .output()
        .unwrap();

    if output.status.success() {
        Ok(PathBuf::from("./abc.exe"))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn run_test(exe: &Path, input_path: &Path) -> TestResult {
    let input = fs::read_to_string(input_path).unwrap();

    let start = Instant::now();
    // TODO: Judge TLE

    // Hit command
    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Inject stdin
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    drop(child.stdin.take());

    // Extract stdout, stderr, exit status
    let output = child.wait_with_output().unwrap();
    let status = output.status;
    let out = String::from_utf8(output.stdout).unwrap();
    let err = String::from_utf8(output.stderr).unwrap();
    let duration = start.elapsed();
    let res = TestResult { status, out, err, duration };
    
    res
}

fn compare_result(output: &str, answer_path: &Path) -> (bool, String) {
    let expected = fs::read_to_string(answer_path).unwrap();

    fn normalize(s: &str) -> Vec<String> {
        s.lines()
            .map(|l| l.trim().to_string())
            .collect()
    }

    (normalize(output) == normalize(&expected), expected)
}

fn printcl(wd: &str, clr: &str) {
    let colour = match clr {
        "Err" => 31,
        "Suc" => 32,
        "Wrn" => 33,
        _ => 36
    };
    print!("\x1b[{}m{}\x1b[m", colour, wd);
}
//C:\Users\hoang\Downloads
fn main() {
    let home = std::env::var("USERPROFILE").unwrap();
    let downloads = format!(r"{}\Downloads\at-samples.zip", home);
    match set_data(&downloads) {
        Err(e) => println!("{}", e),
        _ => (),
    }

    let exe_path = match compile_rs("../src/main.rs") {
        Ok(path) => path,
        Err(e) => {
            println!("====================");
            printcl("|CE|", "Err");
            println!("\n{}", e);
            return;
        },
    };

    for f in get_dir("./") {
        println!("====================");
        println!("case {}", &f.file_name().unwrap().to_string_lossy());
        let res = run_test(&exe_path, &f);
        if !res.status.success() {
            printcl("|RE|", "Err");
            println!(" {} ms\n<out>\n{}\n<err>\n{}", res.duration.as_millis(), res.out, res.err);
            continue;
        }
        let ans_path = &f.with_extension("out");
        if ans_path.exists() {
            let judge = compare_result(&res.out, ans_path);
            if judge.0 {
                printcl("|AC|", "Suc");
                println!(" {} ms\n<out>\n{}\n<err>\n{}", res.duration.subsec_millis(), res.out, res.err);
            } else {
                printcl("|WA|", "Wrn");
                println!(" {} ms\n<expected>\n{}\n<out>\n{}\n<err>\n{}", res.duration.subsec_millis(), judge.1, res.out, res.err);
            }
        }
    }
}
