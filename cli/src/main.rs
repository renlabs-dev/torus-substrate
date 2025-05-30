mod cli;
mod store;
mod sub_command;

#[tokio::main]
async fn main() {
    let cli = cli::parse();
}
