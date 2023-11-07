use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::Edition;
use crate::Result;
use crate::BoxError;

pub struct Mainfest {
    pub crate_name: String,
    pub edition: Edition,
}

impl Mainfest {
    pub fn parse_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut crate_name = None;
        let mut edition = None;

        for line in fs::read_to_string(path)?.lines() {
            let mut split = line.split('=');
            let field = split.next().unwrap().trim();
            let value = split.next().unwrap().trim();

            match field {
                "name" => crate_name = Some(value.replace('"', "")),
                "edition" => {
                    // edition = Some(match value.replace('"', "").parse()? {
                    //     2015 => Edition::E2015,
                    //     2018 => Edition::E2018,
                    //     2021 => Edition::E2021,
                    //     edition => return Err(format!("Edition {edition} is unsupported").into()),
                    // })
                    edition = Some(Edition::from_str(&value.replace('"', ""))?);
                }
                field => return Err(format!("Field {field} is unsupport").into()),
            }
        }

        Ok(Mainfest {
            crate_name: crate_name.ok_or::<BoxError>("name is a required field".into())?,
            edition: edition.ok_or::<BoxError>("edition is a required field".into())?,
        })
    }
}
