# Local Port Scanner

A lightweight TCP port scanner written in Rust.

The application scans a host, identifies open ports, maps common services, and exports the results as JSON.

## Features

- TCP scanning
- Port range
- Service detection
- Colored output
- JSON export

## Run

```bash
cargo run
```

Example

```
PORT     STATUS     SERVICE

22       OPEN       SSH
80       OPEN       HTTP
3306     CLOSED     MySQL
5432     OPEN       PostgreSQL
```

## Future Improvements

- UDP scanning
- Banner grabbing
- Multi-thread scanning
- CSV export
