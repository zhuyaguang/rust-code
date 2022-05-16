use clap::{AppSettings, Clap};

// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// A native httpie implementation with Rust, can you imagine how easy it is?
#[derive(Clap,Debug)]
#[clap(version = "1.0",author = "tyr chen <tyr@chen.com>")]
#[clap(settings = AppSettings::ColoredHelp)]

struct Opts{
    #[clap(subcommand)]
    subcmd:subcommand
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Clap,Debug)]
enum SubCommand{
    Get(Get),
    Psot(Post),
    // 我们暂且不支持其它 HTTP 方法
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Clap,Debug)]
struct Get {
    /// HTTP 请求的 URL
    url: String,
}

// post 子命令。需要输入一个URL，和若干个可选的 key=value,用于提供 json body

/// feed post with an url and optional key=value pairs. we will post the data
/// as JSON,and retrieve the response for you
#[derive(Clap,Debug)]
struct Post{
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 Body
    body: Vec<String>,
}

fn main() {
    let opts: Opts= Opts::parse();
    println!("{:?}", opts);
}