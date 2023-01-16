pub struct HelperCommand {
    pub(crate) name:String,
    pub(crate) aliases:Vec<String>,
    pub(crate) args:Vec<usize>,
}

impl PartialEq for HelperCommand {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.args == other.args && self.aliases == other.aliases
    }
}

impl Eq for HelperCommand {}

pub fn new_command(i_name:String, i_aliases:Vec<String>, i_args:Vec<usize>) -> HelperCommand {
    HelperCommand {name:i_name, aliases:i_aliases, args: i_args}
}
