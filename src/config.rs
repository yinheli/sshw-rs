use std::fs::File;

use std::io::Read;

use std::path::Path;

use anyhow::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Host {
    pub name: String,
    pub host: String,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub keypath: Option<String>,
}

impl Host {
    pub fn to_ssh(&self, opt: Option<&str>) -> String {
        let mut ssh = String::new();
        ssh.push_str("ssh ");
        if self.user.is_some() {
            ssh.push_str(&self.user.clone().unwrap())
        } else {
            ssh.push_str("root")
        }
        ssh.push('@');
        ssh.push_str(&self.host);

        if let Some(keypath) = self.keypath.clone() {
            ssh.push_str(" -i ");
            ssh.push_str(&keypath);
        }

        if let Some(port) = self.port {
            ssh.push_str(" -p ");
            ssh.push_str(&port.to_string());
        }

        if let Some(opt) = opt {
            ssh.push(' ');
            ssh.push_str(opt);
        }
        ssh
    }
}

impl ToString for Host {
    fn to_string(&self) -> String {
        format!(
            "{} {}@{}",
            self.name,
            self.user.clone().unwrap_or_else(|| "root".to_string()),
            self.host
        )
    }
}

pub(crate) fn load(paths: Vec<&str>) -> Result<Vec<Host>, Error> {
    for p in paths {
        let path = Path::new(p);

        println!("{:?} {:?}", path, path.is_symlink());
        if !path.exists() {
            continue;
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let hosts: Vec<Host> = serde_yaml::from_str(&contents)?;
        return Ok(hosts);
    }

    Err(anyhow::anyhow!("No config file found"))
}
