use crate::logger::setup_logger;
use crate::sedaro_twin::TwinMode;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Parser)]
#[command(
author = "Idaho National Laboratory, Digital Engineering", version,
about = format!("{}", banner()),
long_about = format!("{}", banner()),
styles=get_styles(),
)]

pub struct Cli {
    /// the kind of twin to create
    #[arg(env, long, short = 'm', default_value = "digital")]
    pub twin_mode: TwinMode,

    /// queue for requests (DL to TS)
    #[arg(env, long, default_value = "emitter")]
    pub emitter_queue: String,

    /// queue for responses (TS to DL)
    #[arg(env, long, default_value = "listener")]
    pub response_queue: String,

    /// URL for RabbitMQ queue
    #[arg(env, long, default_value = "amqp://sedaro:root@localhost:5672")]
    pub url_rabbitmq: String,

    /// URL for Sedaro Twin Interface
    #[arg(env, long, default_value = "http://localhost:8090")]
    pub url_sedaro_gui: String,

    /// print JSON templates
    #[clap(long, short = 't', action)]
    print_templates: bool,

    /// The logging level
    #[arg(env, long, short = 'g', default_value = "info")]
    pub log_level: String,
}
impl Cli {
    pub fn custom_parse() -> Cli {
        let parse = Cli::parse();
        setup_logger(parse.log_level.as_str());
        parse
    }
}

pub fn banner() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let line = "-".repeat(42 - 8 - 2);
    let banner_string = format!( "\n\n\x1b[93m{}\nSedaro Digital Twin ({})\nMultiverse Engineering...\nsimulate, automate, accelerate\n{}\x1b[0m",
                                 line,
                                 VERSION,
                                 line,
    );
    banner_string
}
/// set the color styles for the help display
pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
