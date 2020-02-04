use exitfailure::ExitFailure;
use structopt::StructOpt;

mod rename_episodes;

#[derive(StructOpt)]
#[structopt(name = "scripts by rodogir")]
enum Scripts {
    #[structopt(about = "rename (incorrect) single episodes to multi episodes syntax")]
    RenameEpisodes {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
        #[structopt(short, long, help = "Performs rename")]
        write: bool,
    },
}

fn main() -> Result<(), ExitFailure> {
    match Scripts::from_args() {
        Scripts::RenameEpisodes { path, write } => rename_episodes::run(path, write),
    }
}
