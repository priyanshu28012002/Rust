// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// impl IpAddr {
//     fn new(address: &str) -> Self {
//         Self {
//             address: address.to_string(),
//             kind: IpAddrKind::V6,
//         }
//     }
// }
// #[derive(Debug)]
// enum IpAddr {
//     V4(u8,u8,u8,u8),
//     V6(String),
// }

// Emum of struct
// struct Ipv4Addr {
//     // --snip--
// }
// struct Ipv6Addr {
//     // --snip--
// }
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // // route("192.168.0.16", IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // route(home);

    // let loopBack = IpAddr::new("127.168.0.15");

    // route(loopBack);

    // let home = IpAddr::V6(String::from("127.0.0.1"));
    // let home6 = IpAddr::V4(127, 0, 0, 1);

    // route(home);
    // route(home6);


    // OPtion Enum
    // let op: Option<i32> =  Option::Some(1);
    // let is_op: Option<i32> =  Option::Some(1).is_some();
    // op.unwrap()
    // op.unwrap_or(default)

}

// fn route(ip: &str, kind: IpAddrKind) {
//     println!("Routing request to ip {ip} , {kind:?}")
// }

// fn route(ip: IpAddr) {
//     println!("Routing request to ip {} , {:?}", ip.address, ip.kind)
// }

// fn route(ip: IpAddr) {
//     println!("Routing request to ip {:?}", ip)
// }
