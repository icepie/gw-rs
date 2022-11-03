use crate::gw::GW;

pub mod gw;


fn main() {

    let client = reqwest::Client::new();

    let gw = GW::new(client);


    gw.test("https://www.baidu.com".to_string());


    println!("Hello, world!");
}
