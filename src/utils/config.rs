pub struct GlobalConfig {
    ports: Ports,
}

struct Ports {
    http: u16,
    https: u16,
}
