use colored::Colorize;

use crate::model::Channel;

pub fn display_channel(channel: &Channel) {
    println!(
        "{}: {}\n{}",
        "Channel".bold().cyan(),
        channel.title.cyan().bold(),
        channel.description.cyan()
    );
}
