use anyhow::Result;
use clap::{Parser, Subcommand};
use segment_rs::{
    client::Client,
    command::{Command, CommandError},
    connection::{Connection, ConnectionOptions},
};
use segment_utils::tokenizer;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'H', long)]
    host: Option<String>,

    #[arg(short, long)]
    port: Option<u16>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Export,
    Restore { filename: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();


    let client = Client::new(ConnectionOptions::new(
        &args.host.unwrap_or(String::from("127.0.0.1")),
        args.port.unwrap_or(1698)
    ));
    let mut conn = client.get_connection().await?;


    match &args.command {
        Commands::Export => {
            export_keyspaces(&mut conn).await;
        }
        Commands::Restore { filename } => {
            restore_keyspaces(Path::new(filename), &mut conn).await;
        }
    }
    Ok(())
}

async fn restore_keyspaces(file: &Path, conn: &mut Connection) {
    let reader = BufReader::new(File::open(file).expect("Cannot open file"));

    for l in reader.lines() {
        let line = l.expect("Error reading line");
        if let Ok(tokens) = tokenizer::tokenize(line.clone()) {
            let mut cmd = Command::new();
            for token in tokens {
                cmd.arg(token);
            }

            match cmd.query::<bool>(conn).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Could not create keyspace from: {}", line);
                    print_error_result(e)
                }
            }
        } else {
            println!("malformed input received")
        }
    }
}

async fn export_keyspaces(conn: &mut Connection) {
    let mut cmd = Command::new();

    cmd.arg(String::from("keyspaces"));

    match cmd.query::<Vec<HashMap<String, String>>>(conn).await {
        Ok(vec) => {
            if !vec.is_empty() {
                for map in vec {
                    println!("create {} evictor {}", map["name"], map["evictor"])
                }
            }
        }
        Err(e) => print_error_result(e),
    }
}

fn print_error_result(e: CommandError) {
    println!("(error) \"{}\"", e)
}
