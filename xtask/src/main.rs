use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Utility tasks for developing the toolbox scaffold")]
struct Xtask {
    /// Optional task name (format, build, check)
    task: Option<String>,
}

fn main() {
    let cli = Xtask::parse();

    match cli.task.as_deref() {
        Some("format") => println!("TODO: implement cargo fmt wrapper"),
        Some("build") => println!("TODO: implement workspace build task"),
        Some("check") => println!("TODO: implement lint/check orchestration"),
        Some(other) => println!("Unknown task `{}`. Try format, build, or check.", other),
        None => println!("Available tasks: format, build, check. Each currently prints TODO."),
    }
}
