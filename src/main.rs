mod exporter;
mod formatter;
mod models;
mod network;
mod scanner;
mod services;
mod utils;

use scanner::scan_ports;

fn main() {
    let result = scan_ports("127.0.0.1", 20, 30);

    formatter::print(&result);

    exporter::export(&result);
}
