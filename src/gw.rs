pub use reqwest::Client as HttpClient;
use core::fmt::Result;
// Result



pub struct GW {
    client: HttpClient
}

impl GW {
    pub fn new(client: HttpClient) -> Self {
        GW { client:client}
    }

    pub fn test(&self, str: String)  {

    // let res = self.client.get(str)
    // .body("the exact body that is sent")
    // .send();

    
    println!("{:?}", str);
   
    }
}