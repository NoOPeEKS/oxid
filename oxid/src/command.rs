use std::str::FromStr;

pub enum Command {
    SaveAll,     // "wa"
    QuitAll,     // "qa"
    SaveQuitAll, // "wqa"

    QuitFile(String), // "q file_name" // TODO: Not provide filename, just pick current buffer.
    SaveFile(String), // "w file_name" // TODO: Not provide filename, just pick current buffer.
    OpenFile(String), // "e file_name"

    NextBuffer,     // "bn"
    PreviousBuffer, // "bp"
}

impl Command {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        input.parse()
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cmd_parts = s.split_whitespace();
        if let Some(cmd) = cmd_parts.next() {
            match cmd {
                "q" => {
                    if let Some(file_name) = cmd_parts.next() {
                        Ok(Self::QuitFile(String::from(file_name)))
                    } else {
                        anyhow::bail!(
                            "For now ':q <file_name>' must be accompanied by a file name."
                        )
                    }
                }
                "w" => {
                    if let Some(file_name) = cmd_parts.next() {
                        Ok(Self::SaveFile(String::from(file_name)))
                    } else {
                        anyhow::bail!(
                            "For now ':w <file_name>' must be accompanied by a file name."
                        )
                    }
                }
                "e" => {
                    if let Some(file_name) = cmd_parts.next() {
                        Ok(Self::SaveFile(String::from(file_name)))
                    } else {
                        anyhow::bail!(
                            "For now ':e <file_name>' must be accompanied by a file name."
                        )
                    }
                }

                "qa" => Ok(Self::QuitAll),
                "wa" => Ok(Self::SaveAll),
                "wqa" => Ok(Self::SaveQuitAll),

                "bn" => Ok(Self::NextBuffer),
                "bp" => Ok(Self::PreviousBuffer),

                _ => anyhow::bail!("Unknown command: {cmd}"),
            }
        } else {
            anyhow::bail!("Empty command is not valid!")
        }
    }
}
