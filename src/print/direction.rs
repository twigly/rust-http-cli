use crate::terminal;
use crate::core::Args;

pub fn render(args: &Args, request_dir: bool) {
    render_with_standard_option(&args, request_dir, true);
}

pub fn render_with_standard_option(args: &Args, request_dir: bool, standard: bool) {
    let flags = &args.flags;
    if flags.show_direction {
        if request_dir {
            terminal::print_and_space(&args.theme.request().direction(standard), ">");
        } else {
            terminal::print_and_space(&args.theme.response().direction(standard), "<");
        }
    }
}
