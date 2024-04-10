use super::Config;

impl Config {
    pub fn search(self) {
        let exit_prompts = |method: &str| {
            eprintln!("{}",format!("Do you wanna to enable {}?\r\nIf this, there is some errors in your input, Please type --help to check in detail.",method))
        };

        match &(self.arguments.cis, self.arguments.fwm) {
            (Some(cis), Some(fwm)) => match (&cis.to_owned()[..], &fwm.to_owned()[..]) {
                ("cis", "fwm") => {
                    let keyword =
                        format!("{}{}{}", " ", self.arguments.keyword, " ").to_uppercase();

                    self.filecontent
                        .0
                        .into_iter()
                        .filter(|item| item.to_uppercase().contains(&keyword))
                        .map(|item| println!("{item}"))
                        .collect::<_>()
                }
                _ => {
                    exit_prompts("Full World Match & Case Insensitive");
                }
            },
            (Some(cis), None) => match &cis.to_owned()[..] {
                "cis" => {
                    let keyword = self.arguments.keyword.to_uppercase();

                    self.filecontent
                        .0
                        .into_iter()
                        .filter(|item| item.to_uppercase().contains(&keyword))
                        .map(|item| println!("{item}"))
                        .collect::<_>()
                }
                _ => {
                    exit_prompts("Case Insensitive");
                }
            },
            (None, Some(fwm)) => match &fwm.to_owned()[..] {
                "fwm" => {
                    let keyword = format!("{}{}{}", " ", self.arguments.keyword, " ");

                    self.filecontent
                        .0
                        .into_iter()
                        .filter(|item| item.contains(&keyword))
                        .map(|item| println!("{item}"))
                        .collect::<_>()
                }
                _ => {
                    exit_prompts("Full World Match");
                }
            },
            (None, None) => self
                .filecontent
                .0
                .into_iter()
                .filter(|item| item.contains(&self.arguments.keyword))
                .map(|item| println!("{item}"))
                .collect::<_>(),
        }
    }
}
