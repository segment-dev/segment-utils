use anyhow::Result;
use clap::Parser;
use rustyline::{error::ReadlineError, Editor};
use segment_rs::{
    client::Client,
    command::{Command, CommandError},
    connection::{Connection, ConnectionOptions},
};
use segment_utils::tokenizer;
use std::collections::HashMap;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value_t = 1698)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = Client::new(ConnectionOptions::new(&args.host, args.port));
    let mut conn = client.get_connection().await?;
    let mut rl = Editor::<()>::new()?;

    loop {
        let line = rl.readline(&format!("{}:{}> ", args.host, args.port));
        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if let Ok(tokens) = tokenizer::tokenize(line) {
                    execute_command(tokens, &mut conn).await;
                } else {
                    println!("malformed input received")
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

async fn execute_command(tokens: Vec<String>, conn: &mut Connection) {
    let mut cmd = Command::new();
    let name = &tokens[0].to_lowercase();

    for token in tokens {
        cmd.arg(token);
    }

    match name.as_str() {
        "create" => match cmd.query::<bool>(conn).await {
            Ok(val) => print_boolean_result(val),
            Err(e) => print_error_result(e),
        },
        "set" => match cmd.query::<bool>(conn).await {
            Ok(val) => print_boolean_result(val),
            Err(e) => print_error_result(e),
        },
        "get" => match cmd.query::<Option<String>>(conn).await {
            Ok(maybe_val) => match maybe_val {
                Some(val) => print_string_result(&val),
                None => print_null_result(),
            },
            Err(e) => print_error_result(e),
        },
        "del" => match cmd.query::<bool>(conn).await {
            Ok(val) => print_boolean_result(val),
            Err(e) => print_error_result(e),
        },
        "drop" => match cmd.query::<bool>(conn).await {
            Ok(val) => print_boolean_result(val),
            Err(e) => print_error_result(e),
        },
        "count" => match cmd.query::<i64>(conn).await {
            Ok(val) => print_integer_result(val),
            Err(e) => print_error_result(e),
        },
        "ttl" => match cmd.query::<Option<i64>>(conn).await {
            Ok(maybe_val) => match maybe_val {
                Some(val) => print_integer_result(val),
                None => print_null_result(),
            },
            Err(e) => print_error_result(e),
        },
        "ping" => match cmd.query::<String>(conn).await {
            Ok(val) => print_string_result(&val),
            Err(e) => print_error_result(e),
        },
        "keyspaces" => match cmd.query::<Vec<HashMap<String, String>>>(conn).await {
            Ok(vec) => {
                if vec.is_empty() {
                    println!("(empty list)");
                }
                for (i, val) in vec.iter().enumerate() {
                    print!("{}) ", i + 1);
                    println!("{:?}", val);
                }
            }
            Err(e) => print_error_result(e),
        },
        _ => println!("(error) \"unknown command '{}'\"", name),
    };
}

fn print_boolean_result(val: bool) {
    println!("(boolean) {}", val)
}

fn print_error_result(e: CommandError) {
    println!("(error) {}", e)
}

fn print_string_result(val: &String) {
    println!("(string) \"{}\"", val)
}

fn print_null_result() {
    println!("(null)")
}

fn print_integer_result(val: i64) {
    println!("(integer) {}", val)
}
