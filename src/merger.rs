use std::collections::HashMap;

use crate::common::{get_time_command, ChallengeTime, ChallengeTimeDifficulty};
use crate::dsc::DSCVM;
use crate::opcodes::{Command, Opcode, OpcodeMeta};

const TARGET_COMMAND_OPCODES: [Opcode; 5] = [
    Opcode::TARGET,
    Opcode::TARGET_FLYING_TIME,
    Opcode::TARGET_EFFECT,
    Opcode::TARGET_FLAG,
    Opcode::EDIT_TARGET,
];

pub struct Event {
    pub time: i32,
    pub commands: Vec<Command>,
}

impl Event {
    pub fn new(time: i32, commands: Vec<Command>) -> Self {
        Self { time, commands }
    }
}

pub struct DSCMerger {
    events: HashMap<i32, Vec<Command>>,
}

impl DSCMerger {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    fn add_command(&mut self, timestamp: i32, command: Command) {
        if let Some(commands) = self.events.get_mut(&timestamp) {
            let mut is_new_command = true;

            for existing_command in commands.clone().iter() {
                if existing_command == &command {
                    is_new_command = false;
                    break;
                }
            }

            if is_new_command {
                commands.push(command);
            }
        } else {
            self.events.insert(timestamp, vec![command]);
        }
    }

    pub fn add_dsc(&mut self, dsc_vm: DSCVM) {
        let mut current_ts = 0;

        for command in dsc_vm.command_buffer {
            if dsc_vm.remove_targets && TARGET_COMMAND_OPCODES.contains(&command.meta.opcode) {
                continue;
            }

            if command.meta.opcode == Opcode::TIME {
                current_ts = command.args[0];
            } else {
                self.add_command(current_ts, command);
            }
        }
    }

    pub fn add_challenge_time(&mut self, challenge_time: ChallengeTime) {
        let start_time = challenge_time.start * 100;
        let end_time = challenge_time.end * 100;

        let mode_select_type = match challenge_time.difficulty {
            ChallengeTimeDifficulty::Easy => 17,
            ChallengeTimeDifficulty::Normal => 2,
        };

        let start_mode_select_command = Command::new(
            OpcodeMeta::new(26, Opcode::MODE_SELECT, 2),
            vec![mode_select_type, 1],
        );

        let end_mode_select_command = Command::new(
            OpcodeMeta::new(26, Opcode::MODE_SELECT, 2),
            vec![mode_select_type, 3],
        );

        self.add_command(start_time, start_mode_select_command);
        self.add_command(end_time, end_mode_select_command);
    }

    fn create_event_vector(&self) -> Vec<Event> {
        let mut events: Vec<Event> = self
            .events
            .iter()
            .map(|(time, commands)| Event::new(*time, commands.clone()))
            .collect();

        events.sort_by(|a, b| a.time.cmp(&b.time));

        events
    }

    pub fn to_dsc(&mut self) -> DSCVM {
        let events = self.create_event_vector();

        let mut dsc_vm = DSCVM::new(false);

        for event in events {
            let time_command = get_time_command(event.time);
            dsc_vm.add_command(time_command);

            for command in &event.commands {
                dsc_vm.add_command(command.clone());
            }
        }

        dsc_vm
    }
}
