/*!

This module is responsible for parsing command lines (`Arglist`, an alias for `&[&str]`) into `Command` instances.
It removes some redundancy from the command line, for example combining the multiple ways to modify a task into a single `Modification` struct.

The module is organized as a nom parser over ArgList, and each struct has a `parse` method to parse such a list.

The exception to this rule is the `args` sub-module, which contains string parsers that are applied to indivdual command-line elements.

All of the structs produced by this module are fully-owned, data-only structs.
That is, they contain no references, and have no methods to aid in their execution -- that is the `invocation` module's job.

*/
mod args;
mod command;
mod filter;
mod modification;
mod subcommand;

pub(crate) use args::TaskId;
pub(crate) use command::Command;
pub(crate) use filter::{Condition, Filter};
pub(crate) use modification::{DescriptionMod, Modification};
pub(crate) use subcommand::Subcommand;

use crate::usage::Usage;

type ArgList<'a> = &'a [&'a str];

pub(crate) fn get_usage(usage: &mut Usage) {
    Subcommand::get_usage(usage);
    Filter::get_usage(usage);
    Modification::get_usage(usage);
}
