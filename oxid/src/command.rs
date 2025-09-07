use std::str::FromStr;

pub enum Command {
    SaveAll,     // "wa"
    QuitAll,     // "qa"
    SaveQuitAll, // "wqa"

    QuitCurrentFile,  // "q"
    SaveCurrentFile,  // "w"
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
                    if cmd_parts.next().is_none() {
                        Ok(Self::QuitCurrentFile)
                    } else {
                        anyhow::bail!(
                            ":q command does not accept sub arguments"
                        )
                    }
                }
                "w" => {
                    if cmd_parts.next().is_none() {
                        Ok(Self::SaveCurrentFile)
                    } else {
                        anyhow::bail!(
                            ":w command does not accept sub arguments"
                        )
                    }
                }
                "e" => {
                    if let Some(file_name) = cmd_parts.next() {
                        Ok(Self::OpenFile(String::from(file_name)))
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
