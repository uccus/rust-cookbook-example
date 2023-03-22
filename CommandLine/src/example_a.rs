use clap::{Arg, Command, ArgAction, arg};

pub fn test(){
    let app = Command::new("这是一个测试程序")
        .version("2.1.0")
        .about("这里是关于")
        .author("臧超")
        .after_help("访问xxxxxx获取更多信息")
        .propagate_version(true)
        .arg_required_else_help(true)
        // .disable_help_flag(true)
        .arg(Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .help("这个选项需要取值"))
        .arg(Arg::new("num")
                .short('n')
                .long("num")
                .action(ArgAction::SetTrue)
                .help("这个选项不取值"))
        .subcommand(Command::new("init")
                            .about("这里是子命令的说明")
                            .arg(arg!([dir] "这个是必填项")
                                    .value_parser(clap::value_parser!(std::path::PathBuf)))
                            .arg(arg!(--file <file> "这里指定文件"))
                            .arg(arg!(--proxy "这是代理操作")))
        ;

    let m = app.get_matches();
    if let Some(file_name) = m.get_one::<String>("file"){
        println!("执行了文件操作, {}", file_name);
    }
    if m.get_flag("num") {
        println!("执行了计数操作");
    }
}