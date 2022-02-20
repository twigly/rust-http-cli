// FIXME Duplicated code with HTTP command

const LONG_FLAG_WIDTH: usize = 15;
use crate::commands::alias::COMMAND_ALIAS;
use crate::rh_name;

macro_rules! newline {
    () => {
        println!("")
    };
}
macro_rules! flags {
    ($description:expr, $long:expr) => {
        println!("      --{:long$} {}", $long, $description, long = LONG_FLAG_WIDTH)
    };
    ($description:expr, $long:expr, $short:expr) => {
        println!("  -{}, --{:long$} {}", $short, $long, $description, long = LONG_FLAG_WIDTH)
    };
}
macro_rules! key_value {
    ($description:expr, $long:expr) => {
        println!("      {:long$} {}", $long, $description, long = LONG_FLAG_WIDTH + 2)
    };
}
macro_rules! text {
    ($description:expr) => {
        println!("  {:long$} {}", "", $description, long = 3)
    };
}
macro_rules! right_text {
    ($description:expr) => {
        println!("  {:long$} {}", "", $description, long = LONG_FLAG_WIDTH + 6)
    };
}

macro_rules! action {
    () => {
        println!("ACTION:");
        flags!("Create a new alias (default action if no action is specified)", "add");
        flags!("Delete an alias", "delete");
        flags!("List all aliases", "list");
    };
}
macro_rules! alias {
    () => {
        println!("ALIAS:");
        key_value!("An alias starts with a @", "@alias");
        right_text!("If there is no alias then it will be the default alias");
    };
}
macro_rules! options {
    () => {
        println!("OPTIONS:");
        text!(format!("Any options you can use with the {} command", rh_name!()));
        text!("There are no options for --delete and --list actions");
    };
}

macro_rules! thanks {
    () => {
        println!("Thanks for using {}!", rh_name!())
    };
}

pub fn show() {
    println!("USAGE:");
    text!(format!("{} {} [action] [@alias] [options]", rh_name!(), COMMAND_ALIAS));

    newline!();
    action!();
    newline!();
    alias!();
    newline!();
    options!();
    newline!();
    thanks!();
}
