use std::net::{TcpListener, TcpStream};
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "tangjz <36634584@qq.com>")]
struct Args {
    #[clap(short, long)]
    port: u16,
}

fn handle_client(_stream: TcpStream) {
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let addr = format!("0.0.0.0:{}", args.port);
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
