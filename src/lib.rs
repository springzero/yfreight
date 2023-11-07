mod config;
mod logger;

use config::Mainfest;
use logger::Logger;
use std::env;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, BoxError>;
pub type BoxError = Box<dyn Error>;

fn root_dir() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    for ancestor in current_dir.ancestors() {
        if ancestor.join("Yfreight.toml").exists() {
            return Ok(ancestor.into());
        }
    }

    Err("No root dir".into())
}

pub fn build() -> Result<()> {
    let mut logger = Logger::new();
    let root_dir = root_dir()?;
    // let crate_name = root_dir
    //     .file_name()
    //     .ok_or::<Box<dyn Error>>("yfreight run in directory without a name".into())?;
    let manifest = Mainfest::parse_from_file(root_dir.join("Yfreight.toml"))?;

    let lib_rs = root_dir.join("src").join("lib.rs");
    let main_rs = root_dir.join("src").join("main.rs");
    let target = root_dir.join("target");
    let target_debug = target.join("ydebug");
    fs::create_dir_all(&target_debug)?;

    let lib_compile = |logger: &mut Logger| -> Result<()> {
        logger.compiling_crate(&manifest.crate_name);
        Rustc::builder()
            .edition(manifest.edition)
            .crate_type(CrateType::Lib)
            .crate_name(&manifest.crate_name)
            .out_dir(target_debug.clone())
            .lib_dir(target_debug.clone())
            .done()
            .run(lib_rs.to_str().unwrap())?;
        logger.done_compiling();
        Ok(())
    };

    let bin_compile = |logger: &mut Logger, externs: Vec<&str>| -> Result<()> {
        logger.compiling_bin(&manifest.crate_name);
        let mut builder = Rustc::builder()
            .edition(manifest.edition)
            .crate_type(CrateType::Bin)
            .crate_name(&manifest.crate_name)
            .out_dir(target_debug.clone())
            .lib_dir(target_debug.clone());

        for ex in externs {
            builder = builder.externs(ex);
        }

        builder.done().run(main_rs.to_str().unwrap())?;
        logger.done_compiling();
        Ok(())
    };

    match (lib_rs.exists(), main_rs.exists()) {
        (true, true) => {
            lib_compile(&mut logger)?;
            bin_compile(&mut logger, vec![&manifest.crate_name])?;
        }
        (true, false) => {
            lib_compile(&mut logger)?;
        }
        (false, true) => {
            bin_compile(&mut logger, vec![])?;
        }
        (false, false) => {
            return Err("There is nothing to compile".into());
        }
    }

    Ok(())
}

pub struct Rustc {
    edition: Edition,
    crate_type: CrateType,
    crate_name: String,
    out_dir: PathBuf,
    lib_dir: PathBuf,
    cfg: Vec<String>,
    externs: Vec<String>,
}

impl Rustc {
    pub fn run(self, path: &str) -> Result<()> {
        Command::new("rustc")
            .arg(path)
            .arg("--edition")
            .arg(self.edition.to_string())
            .arg("--crate-type")
            .arg(self.crate_type.to_string())
            .arg("--crate-name")
            .arg(self.crate_name)
            .arg("--out-dir")
            .arg(self.out_dir)
            .arg("-L")
            .arg(self.lib_dir)
            .args(
                self.externs
                    .into_iter()
                    .map(|r#extern| ["--extern".into(), r#extern])
                    .flatten(),
            )
            .args(
                self.cfg
                    .into_iter()
                    .map(|cfg| ["--cfg".into(), cfg])
                    .flatten(),
            )
            .spawn()?
            .wait()?;

        Ok(())
    }
}

impl Rustc {
    pub fn builder() -> RustcBuilder {
        RustcBuilder {
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct RustcBuilder {
    edition: Option<Edition>,
    crate_type: Option<CrateType>,
    crate_name: Option<String>,
    out_dir: Option<PathBuf>,
    lib_dir: Option<PathBuf>,
    cfg: Vec<String>,
    externs: Vec<String>,
}

impl RustcBuilder {
    pub fn edition(mut self, edition: Edition) -> Self {
        self.edition = Some(edition);
        self
    }
    pub fn out_dir(mut self, out_dir: impl Into<PathBuf>) -> Self {
        self.out_dir = Some(out_dir.into());
        self
    }
    pub fn lib_dir(mut self, lib_dir: impl Into<PathBuf>) -> Self {
        self.lib_dir = Some(lib_dir.into());
        self
    }
    pub fn crate_name(mut self, crate_name: impl Into<String>) -> Self {
        self.crate_name = Some(crate_name.into());
        self
    }
    pub fn crate_type(mut self, crate_type: CrateType) -> Self {
        self.crate_type = Some(crate_type);
        self
    }
    pub fn cfg(mut self, cfg: impl Into<String>) -> Self {
        self.cfg.push(cfg.into());
        self
    }
    pub fn externs(mut self, r#extern: impl Into<String>) -> Self {
        self.externs.push(r#extern.into());
        self
    }

    pub fn done(self) -> Rustc {
        Rustc {
            edition: self.edition.unwrap_or(Edition::E2015),
            crate_type: self.crate_type.expect("Crate type given"),
            crate_name: self.crate_name.expect("Crate name given"),
            out_dir: self.out_dir.expect("Out dir given"),
            lib_dir: self.lib_dir.expect("Lib dir given"),
            cfg: self.cfg,
            externs: self.externs,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Edition {
    E2015,
    E2018,
    E2021,
}

impl Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let edition = match self {
            Self::E2015 => "2015",
            Self::E2018 => "2018",
            Self::E2021 => "2021",
        };
        write!(f, "{edition}")
    }
}

impl FromStr for Edition {
    type Err = BoxError;
    fn from_str(input: &str) -> Result<Self> {
        match input {
            "2015" => Ok(Self::E2015),
            "2018" => Ok(Self::E2018),
            "2021" => Ok(Self::E2021),
            edition => Err(format!("Edition {edition} is not supported").into()),
        }
    }
}

pub enum CrateType {
    Bin,
    Lib,
    RLib,
    DyLib,
    CDyLib,
    StaticLib,
    ProcMacro,
}

impl Display for CrateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crate_type = match self {
            Self::Bin => "bin",
            Self::Lib => "lib",
            Self::RLib => "rlib",
            Self::DyLib => "dylib",
            Self::CDyLib => "cdylib",
            Self::StaticLib => "staticlib",
            Self::ProcMacro => "proc-macro",
        };
        write!(f, "{crate_type}")
    }
}
