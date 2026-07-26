use crate::models::PortInfo;
use colored::*;

pub fn print(list: &[PortInfo]) {

    println!(
        "{:<8} {:<10} {}",
        "PORT",
        "STATUS",
        "SERVICE"
    );

    println!("{}", "-".repeat(32));

    for p in list {

        let status = if p.status == "OPEN" {
            p.status.green()
        } else {
            p.status.red()
        };

        println!(
            "{:<8} {:<18} {}",
            p.port,
            status,
            p.service
        );
    }
}
