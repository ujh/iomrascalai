use board::move::Move;
use board::Color;

#[deriving(Show)]
pub enum Command {
    Play(Move),
    GenMove(Color),
    ProtocolVersion,
    Name,
    Version,
    KnownCommand(bool),
    ListCommands,
    Quit,
    BoardSize(u8),
    ClearBoard,
    Komi(f32),
    Empty,
    Error
}

pub struct GTPInterpreter {
    pub known_commands: Vec<String>
}

impl GTPInterpreter {
    pub fn new() -> GTPInterpreter {
        let mut known_commands = Vec::new();
        known_commands.push(String::from_str("play"));
        known_commands.push(String::from_str("genmove"));
        known_commands.push(String::from_str("protocol_version"));
        known_commands.push(String::from_str("name"));
        known_commands.push(String::from_str("version"));
        known_commands.push(String::from_str("known_command"));
        known_commands.push(String::from_str("list_commands"));
        known_commands.push(String::from_str("quit"));
        known_commands.push(String::from_str("boardsize"));
        known_commands.push(String::from_str("clear_board"));
        known_commands.push(String::from_str("komi"));

        GTPInterpreter {known_commands: known_commands}
    }

    pub fn read(&self, input: &str) -> Command {
        let preprocessed = GTPInterpreter::preprocess(input);

        if preprocessed.len() == 0 {return Empty};

        let command: Vec<&str> = preprocessed.as_slice().split(' ').collect();

        match command.get(0) {
            &"name"             => return Name,
            &"version"          => return Version,
            &"protocol_version" => return ProtocolVersion,
            &"list_commands"    => return ListCommands,
            &"known_command"    => return KnownCommand(self.known_commands.contains(&String::from_str(command.get(1).clone()))),
            &"quit"             => return Quit,
            _                   => return Error         
        }
    }

    pub fn preprocess(input: &str) -> String {
        let mut out = String::from_str(input);

        // We remove every control character except for LF et HT
        // the unsafe block is there because we push_byte
        unsafe {
            out = out.as_bytes().iter().fold(String::new(), |mut s, &b| if b == 9 || b == 10 || (b > 31 && b != 127) {s.push_byte(b); s} else {s});
        }

        // Then we remove anything after a #
        out = out.as_slice().split('#').next().unwrap().to_string();

        // We convert HT to SPACE (ASCII 9 to ASCII 32)
        unsafe {
            out = out.as_bytes().iter().fold(String::new(), |mut s, &b| if b == 9 {s.push_byte(32); s} else {s.push_byte(b); s});
        }

        // We remove the whitespaces before/after the string
        out = out.as_slice().trim().to_string();

        out
    }

    pub fn gen_list_known_commands(&self) -> String {
        let mut result = String::new();

        for c in self.known_commands.iter() {
            result.push_str(c.as_slice());
            result.push_str("\n");
        }

        result
    }
}