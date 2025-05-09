use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FlintstoneRequestOptions {
    GetCurrentCount,
    IncrementCount,
}

// impl From<FlintstoneRequest> for HttpRequest {
//     fn from(req: FlintstoneRequest) -> Self {
//         match req {
//             FlintstoneRequest::GetCurrentCount => HttpRequest::new(
//                 HttpMethod::GET,
//                 EndpointUrl::Flintstone.as_url(),
//                 vec![HttpHeader::ContentTypeJson],
//                 None,
//             ),
//             FlintstoneRequest::IncrementCount => {
//                 let mut flintstone_url = EndpointUrl::Flintstone.as_url();
//                 flintstone_url.set_query(Some("increment=true"));

//                 HttpRequest::new(
//                     HttpMethod::GET,
//                     flintstone_url,
//                     vec![HttpHeader::ContentTypeJson],
//                     None,
//                 )
//             }
//         }
//     }
// }
