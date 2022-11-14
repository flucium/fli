use clap::Arg;
use clap::ArgAction;
use clap::Command;
use std::io;
use std::io::Write;
use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::path::PathBuf;
fn clap_app() -> Command {
    Command::new("fli")
        .version("1.0.0")
        .author("flucium")
        .about("program to get files")
        .args(vec![
            Arg::new("all")
                .long("all")
                .short('a')
                .action(ArgAction::SetTrue)
                .required(false),
            Arg::new("recursive")
                .long("recursive")
                .short('r')
                .action(ArgAction::SetTrue)
                .required(false),
            Arg::new("path")
                .long("path")
                .short('p')
                .action(ArgAction::Set)
                .required(false),
            // Arg::new("format")
            //     .long("format")
            //     .short('f')
            //     .action(ArgAction::Set)
            //     .required(false),
        ])
}

fn walk(path: &Path, is_recursive: bool, is_dot_file: bool) -> io::Result<Vec<PathBuf>> {
    let mut buffer = Vec::new();

    let entries = path.read_dir()?;

    for entry in entries {
        let entry = entry?;

        if entry
            .file_name()
            .to_string_lossy()
            .get(0..1)
            .unwrap_or_default()
            == "."
            && is_dot_file == false
        {
            continue;
        }

        buffer.push(entry.path());

        if entry.file_type()?.is_dir() == true && is_recursive {
            let entries = walk(&entry.path(), is_recursive, is_dot_file)?;
            for entry in entries {
                buffer.push(entry);
            }
        }
    }

    Ok(buffer)
}

fn main() {
    let app = clap_app();

    let matches = app.get_matches();

    let path = match matches.get_one::<String>("path") {
        Some(val) => Path::new(val),
        None => Path::new("./"),
    };

    match walk(path, matches.get_flag("recursive"), matches.get_flag("all")) {
        Ok(entries) => {
            let mut stdout = io::stdout().lock();
            for entry in entries {
                if let Some(file_name) = entry.file_name() {
                    stdout.write(file_name.as_bytes()).unwrap();
                    stdout.write("  ".as_bytes()).unwrap();
                }
            }
            stdout.write("\n".as_bytes()).unwrap();
        }
        Err(err) => {
            io::stderr()
                .lock()
                .write(format!("{err}").as_bytes())
                .unwrap();
        }
    }
}
