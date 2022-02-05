use crate::core::{Args, Result};

pub fn render(args: &Args, request_dir: bool) -> Result<()> {
    render_with_standard_option(args, request_dir, true)
}

pub fn render_with_standard_option(args: &Args, request_dir: bool, standard: bool) -> Result<()> {
    let flags = &args.flags;
    if flags.show_direction {
        if request_dir {
            args.terminal()
                .message_with_style(&args.theme.request().direction(standard), "> ")?;
        } else {
            args.terminal()
                .message_with_style(&args.theme.response().direction(standard), "< ")?;
        }
    }
    Ok(())
}
