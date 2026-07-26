pub fn service_name(port: u16) -> String {

    match port {

        22 => "SSH",

        80 => "HTTP",

        443 => "HTTPS",

        3306 => "MySQL",

        5432 => "PostgreSQL",

        _ => "Unknown"

    }
    .to_string()
}
