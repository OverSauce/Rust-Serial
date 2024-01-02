mod port_builder;
mod port;
mod common;
use port_builder::*;
use port::*;

fn main() -> std::io::Result<()> {
    let port = PortBuilder::new("\\\\.\\COM6")?
        .baud(115200)?
        .timeout(100)?
        .build()?;

    match port.write("Hello".as_ptr(), 6) {
        BytesSent::NoBytes => println!("No bytes sent chief"),
        BytesSent::NotAllBytes(x) => println!("Only {} bytes have been sent", x),
        BytesSent::AllBytes(x) => println!("All {} bytes have been sent", x),
    }

    Ok(())
}
