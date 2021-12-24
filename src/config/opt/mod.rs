use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "env")]
pub struct Opt {
    #[structopt(
        name = "config",
        short,
        long,
        env = "MQRT_CONFIG",
        default_value = "/etc/mqrt/mqrt.toml"
    )]
    pub config_path: String,

    #[structopt(short, long, env = "MQRT_THREADS")]
    pub threads: Option<usize>,
}
