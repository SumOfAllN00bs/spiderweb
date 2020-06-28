use gumdrop::Options;

#[derive(Options)]
struct SpiderOptions {
    #[options(help = "Cipher Tool Help")]
    help: bool,

    #[options(help = "Will use the default tool for input")]
    input: Vec<String>,

    #[options(command)]
    command: Option<Command>,
}

#[derive(Options)]
enum Command {
    #[options(help = "Use a Cipher Tool")]
    Use(UseOptions),

    #[options(help = "Brute force everything")]
    Brute(BruteOptions),
}

#[derive(Options)]
struct UseOptions {
    #[options(help = "The Cipher Tool to use")]
    tool: Option<String>,

    #[options(free)]
    input: Vec<String>,
}

#[derive(Options)]
struct BruteOptions {
    #[options(free)]
    input: Vec<String>,
}