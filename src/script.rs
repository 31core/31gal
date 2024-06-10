use crate::game_pack::GamePack;
use std::io::Result as IOResult;

#[derive(Debug)]
pub struct Script {
    script: String,
    step: usize,
    pub instructions: Vec<Instruction>,
}

impl Script {
    pub fn new() -> Self {
        Self {
            script: String::new(),
            step: 0,
            instructions: Vec::new(),
        }
    }
    /*
     * Parse script
     */
    pub fn parse(&mut self, name: &str, pack: &mut GamePack) -> IOResult<()> {
        name.clone_into(&mut self.script);
        self.instructions.clear();
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
                "switch" => {
                    self.instructions.push(Instruction::Switch {
                        label: tokens[1].to_owned(),
                    });
                }
                "label" => {
                    self.instructions.push(Instruction::Label {
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
        &self.instructions[self.step - 1]
    }
    pub fn get_label(&self, label: &str) -> Option<usize> {
        for (count, i) in self.instructions.iter().enumerate() {
            if let Instruction::Label { label: this_label } = i {
                if this_label == label {
                    return Some(count + 1);
                }
            }
        }
        None
    }
    pub fn switch_to(&mut self, step: usize) {
        self.step = step
    }
    fn execute_single_instruction(
        &mut self,
        pack: &mut GamePack,
        content: &mut crate::Content,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        font_name: &str,
    ) -> std::io::Result<()> {
        let instruction = self.step().clone();
        match instruction {
            Instruction::Say { saying, character } => {
                content.saying = saying;
                content.character = character;

                crate::window::redraw(pack, canvas, content, font_name);
            }
            Instruction::Scene { resource } => {
                let bytes = pack.get_resource(&resource)?;
                crate::window::draw_background(&bytes, canvas);

                content.scene = resource;
            }
            Instruction::Switch { label } => {
                let step = self.get_label(&label);
                self.switch_to(step.unwrap_or_else(|| panic!("'{}' is not defined", label)));
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
    ) -> std::io::Result<()> {
        loop {
            if let Instruction::Say { .. } = self.instructions[self.step] {
                self.execute_single_instruction(pack, content, canvas, font_name)?;
                break;
            } else {
                self.execute_single_instruction(pack, content, canvas, font_name)?;
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
