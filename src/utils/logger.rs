use env_logger::{Builder,Target};

use  log::LevelFilter;

pub fn init_logger(){
    Builder::from_default_env()
            .target(Target::Stdout)
            .filter(None, LevelFilter::Info)
            .init();
}