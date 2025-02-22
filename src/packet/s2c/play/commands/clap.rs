use super::{ CommandsS2CPlayPacket, CommandNode, CommandNodeKind, CommandNodeParser, StringCommandNode };
use crate::value::{ VarInt, Identifier };
use crate::value::Item;
use crate::registry::RegEntry;
use core::any::TypeId;
use core::{ mem, fmt };
use clap::{ CommandFactory, Command };
use clap::builder::ValueRange;


struct FromClap {
    nodes : Vec<CommandNode>
}
impl FromClap {

    fn new() -> Self {
        Self { nodes : vec![
            CommandNode {
                kind           : CommandNodeKind::Root,
                is_executable  : false,
                child_indices  : Vec::new().into(),
                redirect_index : None
            }
        ] }
    }

    fn add_literal<'l>(&mut self, parents : impl IntoIterator<Item = &'l usize>, name : &str, is_executable : bool) -> usize {
        let i = self.nodes.len();
        let iv = VarInt::from(i);
        self.nodes.push(CommandNode {
            kind           : CommandNodeKind::Literal { name : name.to_string() },
            is_executable,
            child_indices  : Vec::new().into(),
            redirect_index : None
        });
        for parent in parents {
            self.nodes[*parent].child_indices.push(iv);
        }
        i
    }

    fn add_argument<'l>(&mut self, parents : impl IntoIterator<Item = &'l usize>, name : &str, is_executable : bool, parser : CommandNodeParser) -> usize {
        let i = self.nodes.len();
        let iv = VarInt::from(i);
        self.nodes.push(CommandNode {
            kind           : CommandNodeKind::Argument { name : name.to_string(), parser, suggestions : None },
            is_executable,
            child_indices  : Vec::new().into(),
            redirect_index : None
        });
        for parent in parents {
            self.nodes[*parent].child_indices.push(iv);
        }
        i
    }

    fn add_argument_recursive<'l>(&mut self, parents : impl IntoIterator<Item = &'l usize>, name : &str, is_executable : bool, parser : CommandNodeParser) -> usize {
        let i = self.nodes.len();
        let iv = VarInt::from(i);
        let j = i + 1;
        let jv = VarInt::from(j);
        self.nodes.push(CommandNode {
            kind           : CommandNodeKind::Root,
            is_executable  : false,
            child_indices  : vec![ jv ].into(),
            redirect_index : None
        });
        self.nodes.push(CommandNode {
            kind           : CommandNodeKind::Argument { name : name.to_string(), parser, suggestions : None },
            is_executable,
            child_indices  : Vec::new().into(),
            redirect_index : Some(iv)
        });
        for parent in parents {
            self.nodes[*parent].child_indices.push(jv);
        }
        i
    }


    fn add_command(&mut self, root_parent : usize, command : &Command) {

        // TODO: get_visible_aliases
        // TODO: short_flag_aliases
        // TODO: long_flag_aliases
        // TODO: get_subcommands
        // TODO: get_groups
        // TODO: get_arguments
        // TODO: get_positionals
        // TODO: get_opts
        // TODO: get_arg_conflicts_with
        // TODO: is_disable_help_flag_set
        // TODO: is_disable_help_subcommand_set
        // TODO: is_allow_missing_positional_set
        // TODO: is_subcommand_required_set
        // TODO: is_allow_external_subcommands_set
        // TODO: is_args_conflicts_with_subcommands_set
        // TODO: is_subcommand_precedence_over_arg_set
        // TODO: is_subcommand_negates_reqs_set
        // TODO: is_mutlicall_set

        let mut parents = vec![ root_parent ];
        for positional in command.get_positionals() {
            let mut old_parents = parents.clone();
            if (positional.is_required_set()) {
                parents = Vec::new();
            }; // TODO: other flags

            let name           = positional.get_id().to_string();
            let num_args       = positional.get_num_args().unwrap_or(ValueRange::new(1));
            let infinite_count = num_args.max_values() >= 8;
            let anytype        = unsafe{ mem::transmute::<_, AnyTypeId>(positional.get_value_parser().type_id()) };

            if (anytype == TypeId::of::<u64>()) {
                if (! infinite_count) {
                    // TODO: Repeat
                    let node = self.add_argument(&old_parents, &name, true, CommandNodeParser::Long { min : Some(0), max : None });
                    parents.push(node);
                    old_parents.push(node);
                } else {
                    let node = self.add_argument_recursive(&old_parents, &name, true, CommandNodeParser::Long { min : Some(0), max : None });
                    parents.push(node);
                    old_parents.push(node);
                }
            }

            else if (anytype == TypeId::of::<String>()) {
                if (! infinite_count) {
                    // TODO: Repeat
                    let node = self.add_argument(&old_parents, &name, true, CommandNodeParser::String { behaviour : StringCommandNode::QuotablePhrase });
                    parents.push(node);
                    old_parents.push(node);
                } else {
                    let node = self.add_argument(&old_parents, &name, true, CommandNodeParser::String { behaviour : StringCommandNode::GreedyPhrase });
                    parents.push(node);
                    old_parents.push(node);
                }
            }

            else if (anytype == TypeId::of::<RegEntry<Item>>()) {
                if (! infinite_count) {
                    // TODO: Repeat
                    let node = self.add_argument(&old_parents, &name, true, CommandNodeParser::ResourceKey { registry : Identifier::vanilla_const("item") });
                    parents.push(node);
                    old_parents.push(node);
                } else {
                    let node = self.add_argument_recursive(&old_parents, &name, true, CommandNodeParser::ResourceKey { registry : Identifier::vanilla_const("item") });
                    parents.push(node);
                    old_parents.push(node);
                }
            }

            else { panic!("{} can not be converted to a minecraft command argument.", anytype) };

        }


        for subcommand in command.get_subcommands() {
            let subcommand_node = self.add_literal(&parents, subcommand.get_name(), true);
            self.add_command(subcommand_node, subcommand);
        }
    }


}


impl CommandsS2CPlayPacket {

    /// Please note that this feature is heavily experimental and may not work as expected.
    #[doc(cfg(feature = "commands_clap"))]
    pub fn from_clap<C : CommandFactory>() -> Self {
        let mut fromclap = FromClap::new();
        fromclap.add_command(0, &C::command());
        Self {
            nodes      : fromclap.nodes.into(),
            root_index : VarInt::from(0)
        }
    }

}


#[derive(Clone, Copy)]
pub struct AnyTypeId {
    id : TypeId,
    #[cfg(debug_assertions)]
    name : &'static str
}
impl PartialEq<TypeId> for AnyTypeId {
    fn eq(&self, other : &TypeId) -> bool {
        self.id == *other
    }
}
impl fmt::Display for AnyTypeId {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(debug_assertions)]
        { self.name.fmt(f) }
        #[cfg(not(debug_assertions))]
        { self.id.fmt(f) }
    }
}
