// FIXME Duplicated code with ALIAS command

const LONG_FLAG_WIDTH: usize = 15;
#[cfg(feature = "alias")]
use crate::commands::alias::COMMAND_ALIAS;
use crate::rh_name;

macro_rules! newline {
    () => {
        println!("")
    };
}

macro_rules! logo {
    () => {
        println!(
            "╱╱╭╮
╱╱┃┃
╭━┫╰━╮
┃╭┫╭╮┃
┃┃┃┃┃┃
╰╯╰╯╰╯"
        )
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

#[cfg(feature = "alias")]
macro_rules! try_help_alias {
    () => {
        right_text!(format!("try '{} {} --help' for more information", rh_name!(), COMMAND_ALIAS));
    };
}

#[cfg(feature = "alias")]
macro_rules! alias {
    () => {
        println!("ALIAS:");
        key_value!("An alias starts with a @", "@alias");
        try_help_alias!();
    };
}
macro_rules! method {
    () => {
        println!("METHOD:");
        key_value!("If there is no data items then GET is the default method", "GET");
        key_value!("If there are data items then POST is the default method", "POST");
        key_value!("You can force any standard method (upper case)", "Standard method");
        right_text!("GET|POST|PUT|DELETE|HEAD|OPTIONS|CONNECT|PATCH|TRACE");
        key_value!("You can use a custom method (upper case)", "Custom method");
    };
}
macro_rules! options {
    () => {
        println!("OPTIONS:");
        flags!("Show version", "version");
        flags!("Show this screen", "help");
        flags!("Show a symbol for the request part and another one for the response part", "direction", "d");
        flags!("Colorize the output (shortcut: --pretty=c)", "pretty=color");
        flags!("Show more details, shortcut for -UHBshb", "verbose", "v");
        flags!("Show the request and response headers", "headers");
        flags!("Show the request URL and method", "url", "U");
        flags!("Show the request header", "req-header", "H");
        flags!("Show the request payload", "req-body", "B");
        flags!("Compact the request payload", "req-compact", "C");
        flags!("Show the response status and HTTP version", "status", "s");
        flags!("Show the response header", "header", "h");
        flags!("Show the response body (default)", "body", "b");
        flags!("Hide the response body", "body=n");
        flags!("Compact the response body", "compact", "c");
        newline!();
        key_value!("Combine any short flags, for example:", "-cUh...");
        right_text!("-c compact the response");
        right_text!("-U url and method");
        right_text!("-h response header");
    };
}
macro_rules! headers {
    () => {
        println!("HEADERS:");
        key_value!("List of key:value space-separated", "<key:value>...");
    };
}
macro_rules! body {
    () => {
        println!("PAYLOAD:");
        flags!("Set the payload and don't apply any transformation", "raw=<payload>");
        flags!("Force the 'Accept' header to 'application/json' (default)", "json");
        flags!("Set the 'Content-Type' and serialize data items as form URL encoded", "form");
        key_value!("Data items as a list of key=value space-separated", "<key=value>...");
        right_text!("Data items are converted to JSON (default) or URL encoded (--form)");
    };
}

#[cfg(feature = "alias")]
macro_rules! subcommand {
    () => {
        println!("SUBCOMMAND:");
        key_value!("Manage aliases", COMMAND_ALIAS);
        try_help_alias!();
    };
}

macro_rules! thanks {
    () => {
        println!("Thanks for using {}!", rh_name!())
    };
}

pub fn show() {
    logo!();

    newline!();
    println!("USAGE:");
    #[cfg(feature = "alias")]
    {
        text!(format!("{} [@alias] [METHOD] url [options] [headers] [payload]", rh_name!()));
    }
    #[cfg(not(feature = "alias"))]
    {
        text!(format!("{} [METHOD] url [options] [headers] [payload]", rh_name!()));
    }
    text!(format!("{} --help | -h", rh_name!()));
    text!(format!("{} --version", rh_name!()));
    #[cfg(feature = "alias")]
    {
        newline!();
        text!(format!("{} [SUBCOMMAND] [options]", rh_name!()));
        text!(format!("{} [SUBCOMMAND] --help | -h", rh_name!()));
    }

    #[cfg(feature = "alias")]
    {
        newline!();
        alias!();
    }
    newline!();
    method!();
    newline!();
    options!();
    newline!();
    headers!();
    newline!();
    body!();
    newline!();
    #[cfg(feature = "alias")]
    {
        subcommand!();
        newline!();
    }
    thanks!();
}
