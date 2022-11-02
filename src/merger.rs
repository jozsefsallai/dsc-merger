use crate::common::{
    get_time_command, timestamp_to_millis, ChallengeTime, ChallengeTimeDifficulty,
};
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

    pub fn add_challenge_time(&mut self, challenge_time: ChallengeTime) {
        let start_time = timestamp_to_millis(challenge_time.start) * 100;
        let end_time = timestamp_to_millis(challenge_time.end) * 100;

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

        let mut start_event = Event::new(start_time);
        start_event.add_command(start_mode_select_command);

        let mut end_event = Event::new(end_time);
        end_event.add_command(end_mode_select_command);

        self.events.push(start_event);
        self.events.push(end_event);
    }

    fn sort_by_time(&mut self) {
        self.events.sort_by(|a, b| a.time.cmp(&b.time));
    }

    pub fn to_dsc(&mut self) -> DSCVM {
        self.sort_by_time();

        let mut dsc_vm = DSCVM::new();

        for event in &self.events {
            let time_command = get_time_command(event.time);
            dsc_vm.add_command(time_command);

            for command in &event.commands {
                dsc_vm.add_command(command.clone());
            }
        }

        dsc_vm
    }
}
