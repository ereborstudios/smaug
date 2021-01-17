use serde::Serialize;
use smaug::config::Config;
use std::path::Path;
use tinytemplate::TinyTemplate;

#[derive(Debug, Serialize)]
pub struct GameMetadata {
    pub devid: String,
    pub devtitle: String,
    pub gameid: String,
    pub gametitle: String,
    pub version: String,
    pub icon: String,
}

pub fn from_config(config: &Config) -> GameMetadata {
    let project = config
        .project
        .clone()
        .expect("Smaug.toml is not a project configuration");
    GameMetadata {
        devid: project.authors.join(" "),
        devtitle: project.authors.join(" "),
        gameid: project.name,
        gametitle: project.title,
        version: project.version,
        icon: project.icon,
    }
}

impl GameMetadata {
    pub fn write<P: AsRef<Path>>(&self, path: &P) -> std::io::Result<()> {
        let template = include_str!("../templates/game_metadata.txt.template");
        let mut tt = TinyTemplate::new();
        tt.add_template("game_metadata.txt", template)
            .expect("couldn't add template.");

        let contents = tt
            .render("game_metadata.txt", self)
            .expect("Could not render template.");
        std::fs::write(path, contents)?;

        Ok(())
    }
}
