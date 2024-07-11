use log::info;
use sedaro::cli::Cli;
use sedaro::sedaro_twin::SedaroTwin;

fn main() {
    let cli = Cli::custom_parse();
    info!("Running twin in {:?} mode...", cli.twin_mode);

    // TODO
    let twin = SedaroTwin::new(cli.twin_mode);
    twin.run(&cli);
}
