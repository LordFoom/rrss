use colored::Colorize;

use crate::model::Channel;

pub fn display_channel(channel: &Channel) {
    println!(
        "{}: {}\n{}",
        "Channel".bold().cyan(),
        channel.title.cyan().bold(),
        channel.description.cyan()
    );
    channel.items.iter().for_each(|item| {
        if let Some(title) = item.title.clone() {
            println!("{}", title.truecolor(255, 155, 0));
        }
        println!("{}", item.link.cyan());
    });
}
