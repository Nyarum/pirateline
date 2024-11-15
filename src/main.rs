mod server;

fn main() {
    server::tcp::create_server("0.0.0.0", "1973");
}
