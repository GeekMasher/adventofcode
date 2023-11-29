use clap::Parser;
use console::style;
use std::path::PathBuf;

pub const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const BANNER: &str = r#"      __       ________  ___      ___  _______  _____  ___  ___________         ______    _______       ______    ______    ________    _______  
     /""\     |"      "\|"  \    /"  |/"     "|(\"   \|"  \("     _   ")       /    " \  /"     "|     /" _  "\  /    " \  |"      "\  /"     "| 
    /    \    (.  ___  :)\   \  //  /(: ______)|.\\   \    |)__/  \\__/       // ____  \(: ______)    (: ( \___)// ____  \ (.  ___  :)(: ______) 
   /' /\  \   |: \   ) || \\  \/. ./  \/    |  |: \.   \\  |   \\_ /         /  /    ) :)\/    |       \/ \    /  /    ) :)|: \   ) || \/    |   
  //  __'  \  (| (___\ ||  \.    //   // ___)_ |.  \    \. |   |.  |        (: (____/ // // ___)       //  \ _(: (____/ // (| (___\ || // ___)_  
 /   /  \\  \ |:       :)   \\   /   (:      "||    \    \ |   \:  |         \        / (:  (         (:   _) \\        /  |:       :)(:      "| 
(___/    \___)(________/     \__/     \_______) \___|\____\)    \__|          \"_____/   \__/          \_______)\"_____/   (________/  \_______)"#;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(long, help = "Enable Debugging", default_value_t = false)]
    pub debug: bool,

    #[clap(long, help = "Disable Banner", default_value_t = false)]
    pub disable_banner: bool,

    #[clap(short, long, help = "Day of Advent of Code", default_value_t = 0)]
    pub day: i32,

    #[clap(short, long, help = "Input File")]
    pub input: Option<PathBuf>,
}

pub fn setup() -> Arguments {
    let arguments = Arguments::parse();

    let log_level = match arguments.debug {
        false => log::LevelFilter::Info,
        true => log::LevelFilter::Debug,
    };

    env_logger::builder()
        .parse_default_env()
        .filter_level(log_level)
        .init();

    if !arguments.disable_banner {
        println!(
            "{}    {} - v{}\n",
            style(BANNER).green(),
            style(AUTHOR).red(),
            style(VERSION_NUMBER).blue()
        );
    }

    arguments
}
