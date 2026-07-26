use crate::models::PortInfo;
use crate::services::service_name;

pub fn scan_ports(_host: &str, start: u16, end: u16) -> Vec<PortInfo> {

    let mut result = Vec::new();

    for port in start..=end {

        let open = port == 22 || port == 80;

        result.push(
            PortInfo {
                port,
                status: if open {
                    "OPEN".into()
                } else {
                    "CLOSED".into()
                },
                service: service_name(port)
            }
        );
    }

    result
}
