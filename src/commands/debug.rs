#[cfg(feature = "alias")]
use crate::shell::os::OsDirs;
use std::env;
#[cfg(feature = "alias")]
use std::path::Path;

const KEY_WIDTH: usize = 25;

pub fn show() {
    show_program();
    #[cfg(feature = "alias")]
    {
        println!();
        show_directories();
    }
    println!();
    show_env_vars();
}

fn show_program() {
    println!("{:width$} {}", "Name", crate::rh_name!(), width = KEY_WIDTH);
    println!("{:width$} {}", "Version", crate::rh_version!(), width = KEY_WIDTH);
    println!("{:width$} {}", "Homepage", crate::rh_homepage!(), width = KEY_WIDTH);
}

#[cfg(feature = "alias")]
fn show_directories() {
    use crate::shell::os::DefaultOsDirs;

    let os_dirs = DefaultOsDirs::new();
    let mut config_dir_exists = false;
    match os_dirs.config_directory() {
        Some(path) => {
            println!("{:width$} {}", "Config location", path.display(), width = KEY_WIDTH);
            config_dir_exists = Path::new(&path).exists();
            if !config_dir_exists {
                println!("{:width$} Cannot find this directory on your platform", "", width = KEY_WIDTH);
            }
        }
        None => {
            println!("{:width$} cannot find the config path on your platform", "Config location", width = KEY_WIDTH);
        }
    };

    if config_dir_exists {
        let path = match os_dirs.app_config_directory() {
            Some(path) => path.display().to_string(),
            None => String::from("cannot find the alias path on your platform"),
        };
        println!("{:width$} {}", "Aliases location", path, width = KEY_WIDTH);
    } else {
        println!("{:width$} alias feature disabled on your platform", "Aliases location", width = KEY_WIDTH);
    }
}

fn show_env_vars() {
    env::vars().into_iter().for_each(|(name, value)| println!("{:width$} {}", name, value, width = KEY_WIDTH));
}
