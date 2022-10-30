use crate::dsc::DSCVM;
use crate::opcodes::{Command, Opcode, OpcodeMeta};

pub struct Event {
    pub time: i32,
    pub commands: Vec<Command>,
}

impl Event {
    pub fn new(time: i32) -> Self {
        Self {
            time,
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }
}

pub struct DSCMerger {
    events: Vec<Event>,
}

impl DSCMerger {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn add_dsc(&mut self, dsc_vm: DSCVM) {
        let mut event: Option<Event> = None;

        for command in dsc_vm.command_buffer {
            if command.meta.opcode == Opcode::TIME {
                if event.is_some() {
                    self.events.push(event.unwrap());
                }

                event = Some(Event::new(command.args[0]));
            } else {
                if event.is_some() {
                    event.as_mut().unwrap().add_command(command);
                }
            }
        }

        if event.is_some() {
            self.events.push(event.unwrap());
        }
    }

    fn sort_by_time(&mut self) {
        self.events.sort_by(|a, b| a.time.cmp(&b.time));
    }

    pub fn to_dsc(&mut self) -> DSCVM {
        self.sort_by_time();

        let mut dsc_vm = DSCVM::new();

        for event in &self.events {
            let time_command = self.get_time_command(event.time);
            dsc_vm.add_command(time_command);

            for command in &event.commands {
                dsc_vm.add_command(command.clone());
            }
        }

        dsc_vm
    }

    fn get_time_command(&self, time: i32) -> Command {
        let id = 1;
        let opcode = Opcode::TIME;
        let param_count: usize = 1;

        let meta = OpcodeMeta::new(id, opcode, param_count);
        return Command::new(meta, vec![time]);
    }
}
