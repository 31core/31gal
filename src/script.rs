use crate::game_pack::GamePack;
use std::io::Result as IOResult;

#[derive(Debug)]
pub struct Script {
    script: String,
    step: usize,
    pub insts: Vec<Instruction>,
}

impl Script {
    pub fn new() -> Self {
        Self {
            script: String::new(),
            step: 0,
            insts: Vec::new(),
        }
    }
    /**
     * Parse script.
     *
     * Args:
     * * name: Script relative path to `scripts/` directory.
     */
    pub fn parse(&mut self, name: &str, pack: &mut GamePack) -> IOResult<()> {
        name.clone_into(&mut self.script);
        self.insts.clear();
        for line in pack.get_script(name)?.split('\n') {
            let mut tokens = Vec::new();
            for token in line.split(' ').collect::<Vec<&str>>() {
                if token.is_empty() {
                    continue;
                }
                tokens.push(token.replace("\\n", "\n"));
            }
            if tokens.is_empty() {
                continue;
            }

            match &tokens[0][..] {
                "say" => {
                    if tokens.len() > 2 {
                        self.insts.push(Instruction::Say {
                            saying: tokens[1].to_owned(),
                            character: Some(tokens[2].to_owned()),
                        })
                    } else {
                        self.insts.push(Instruction::Say {
                            saying: tokens[1].to_owned(),
                            character: None,
                        })
                    }
                }
                "scene" => {
                    self.insts.push(Instruction::Scene {
                        resource: tokens[1].to_owned(),
                    });
                }
                "switch" => {
                    self.insts.push(Instruction::Switch {
                        label: tokens[1].to_owned(),
                    });
                }
                "label" => {
                    self.insts.push(Instruction::Label {
                        label: tokens[1].to_owned(),
                    });
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn step(&mut self) -> &Instruction {
        self.step += 1;
        &self.insts[self.step - 1]
    }
    pub fn current_inst(&self) -> &Instruction {
        &self.insts[self.step]
    }
    pub fn get_label(&self, label: &str) -> Option<usize> {
        for (count, i) in self.insts.iter().enumerate() {
            if let Instruction::Label { label: this_label } = i {
                if this_label == label {
                    return Some(count + 1);
                }
            }
        }
        None
    }
    pub fn switch_to(&mut self, step: usize) {
        self.step = step;
    }
    fn exec_single_inst(
        &mut self,
        pack: &mut GamePack,
        content: &mut crate::Content,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        font_name: &str,
    ) -> anyhow::Result<()> {
        let instruction = self.step().clone();
        match instruction {
            Instruction::Say { saying, character } => {
                content.saying = saying;
                content.character = character;

                crate::window::redraw(pack, canvas, content, font_name)?;
            }
            Instruction::Scene { resource } => {
                let bytes = pack.get_resource(&resource)?;
                crate::window::draw_background(&bytes, canvas);

                content.scene = resource;
            }
            Instruction::Switch { label } => {
                let label_parsed = label.split(':').collect::<Vec<&str>>();
                if label_parsed.len() == 1 {
                    let step = self.get_label(label_parsed[0]);
                    self.switch_to(step.unwrap_or_else(|| {
                        panic!(
                            "{}:{}: '{}' is not defined",
                            self.script, self.script, label_parsed[0]
                        )
                    }));
                } else {
                    self.parse(label_parsed[0], pack)?;
                    let step = self.get_label(label_parsed[1]);
                    self.switch_to(step.unwrap_or_else(|| {
                        panic!(
                            "{}:{}: '{}' is not defined",
                            self.script, self.script, label_parsed[1]
                        )
                    }));
                }
            }
            _ => {}
        }

        Ok(())
    }
    pub fn execute_script(
        &mut self,
        pack: &mut GamePack,
        content: &mut crate::Content,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        font_name: &str,
    ) -> anyhow::Result<()> {
        loop {
            if let Instruction::Say { .. } = self.current_inst() {
                self.exec_single_inst(pack, content, canvas, font_name)?;
                break;
            } else {
                self.exec_single_inst(pack, content, canvas, font_name)?;
            }
        }

        Ok(())
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
    Switch {
        label: String,
    },
    Label {
        label: String,
    },
}
