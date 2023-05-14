use super::game_pack::GamePack;
use std::io::Result as IOResult;

pub struct Script {
    pub pack: GamePack,
    script: String,
    step: usize,
    instructions: Vec<Instruction>,
}

impl Script {
    pub fn new(pack: GamePack) -> Self {
        Self {
            pack,
            script: String::new(),
            step: 0,
            instructions: Vec::new(),
        }
    }
    /*
     * Parse script
     */
    pub fn parse(&mut self, name: &str) -> IOResult<()> {
        self.script = name.to_owned();
        self.instructions.clear();
        for line in self.pack.get_script(name)?.split("\n") {
            let mut tokens = Vec::new();
            for token in line.split(" ").collect::<Vec<&str>>() {
                if token == "" {
                    continue;
                }
                tokens.push(token.replace("\\n", "\n"));
            }
            if tokens.len() == 0 {
                continue;
            }

            match &tokens[0][..] {
                "say" => {
                    if tokens.len() > 2 {
                        self.instructions.push(Instruction::Say {
                            saying: tokens[1].to_owned(),
                            character: Some(tokens[2].to_owned()),
                        })
                    } else {
                        self.instructions.push(Instruction::Say {
                            saying: tokens[1].to_owned(),
                            character: None,
                        })
                    }
                }
                "scene" => {
                    self.instructions.push(Instruction::Scene {
                        resource: tokens[1].to_owned(),
                    });
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn step(&mut self) -> &Instruction {
        self.step += 1;
        &self.instructions[self.step - 1]
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Say {
        saying: String,
        character: Option<String>,
    },
    Scene {
        resource: String,
    },
}
