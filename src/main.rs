use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {

    {
        const HELP: &str = "\
        Alternative for Cargo\n\n\
        Usage: freight [COMMAND] [OPTIONS]\n\n\
        Commands:\n    \
            help    Print out this message
        ";

        // 跳过第一个参数，因为这只是正在运行的可执行文件的名称，我们不需要它，我们只关心后面的内容
        let mut args = env::args().skip(1);
        match args.next().as_ref().map(String::as_str) {
            Some("build") => yfreight::build()?,
            // Some("test") => {
            //     yfreight::build_tests()?;
            //     yfreight::run_tests()?;
            // }
            Some("help") => println!("{HELP}"),
            _ => {
                println!("Unsupported command");
                println!("{HELP}");

                process::exit(1);
            }
        }

        println!("Bootstrapped successfully!");
    }
    Ok(())
}
