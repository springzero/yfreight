use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // #[cfg(not(stage1))]
    // {
    //     use std::fs;
    //     use std::path::PathBuf;
    //     use yfreight::CrateType;
    //     use yfreight::Edition;
    //     use yfreight::Rustc;
    //     const BOOTSTRAP_STAGE1: &str = "bootstrap_stage1";

    //     let target_dir = PathBuf::from("target");
    //     let bootstrap_dir = target_dir.join(BOOTSTRAP_STAGE1);
    //     fs::create_dir_all(&bootstrap_dir)?;
    //     Rustc::builder()
    //         .edition(Edition::E2021)
    //         .crate_type(CrateType::Lib)
    //         .crate_name("yfreight")
    //         .out_dir(bootstrap_dir.clone())
    //         .lib_dir(bootstrap_dir.clone())
    //         .cfg("stage1")
    //         .done()
    //         .run("src/lib.rs")?;
    //     Rustc::builder()
    //         .edition(Edition::E2021)
    //         .crate_type(CrateType::Bin)
    //         .crate_name("yfreight_stage1")
    //         .out_dir(bootstrap_dir.clone())
    //         .lib_dir(bootstrap_dir)
    //         .cfg("stage1")
    //         .externs("yfreight")
    //         .done()
    //         .run("src/main.rs")?;

    //     println!("Completed Stage1 Build");
    // }
    // #[cfg(stage1)]
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
