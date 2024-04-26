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
        if let Some(titles) = item.title.clone() {
            if titles.len() > 0 {
                let l = titles.get(0).unwrap();
                println!("{}", l.truecolor(255, 155, 0));
            }
        }

        if let Some(a_description) = item.description.clone() {
            println!("{}", a_description.cyan());
        }

        if let Some(a_link) = item.link.clone() {
            println!("{}", a_link.yellow());
        }

        if let Some(an_enclosure) = item.enclosure.clone() {
            println!(
                "{}: {}",
                "Link".magenta(),
                an_enclosure.url.bold().magenta()
            );
        }
        if let Some(dt) = item.pub_date.clone() {
            println!("{}", dt.truecolor(46, 139, 87));
        }

        println!();
    });
}
