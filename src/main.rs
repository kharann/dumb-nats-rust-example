extern crate clap;

use std::io;

use tokio::task;

use crate::clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    StartServer,
    PublishMessage,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let opts = Opts::parse();
    let msg = "I vote for Kharann";
    match opts.subcmd {
        SubCommand::StartServer => {
            println!("STARTING SERVER");
            let nc = nats::connect("localhost:4222").unwrap();
            let sub = nc.subscribe("app.1.voting").unwrap();
            let mut count = 0;
            for msg in sub.messages() {
                println!("[SERVER]: RECIEVED {:?}", msg);
                count = count + 1;
                println!("[SERVER]: Message count :{}", count);
                match msg.data {
                    _ => {
                        todo!("Match Enum Type (or deserialize data) and add count to correct candidate")
                    }
                }
            }
        }
        SubCommand::PublishMessage => {
            println!("Sending message with text: {}", msg);
            let nc = nats::connect("localhost:4222").unwrap();
            nc.publish("app.1.voting", msg);
        }
        _ => {
            println!("kekw 😂")
        }
    }
    Ok(())
}
