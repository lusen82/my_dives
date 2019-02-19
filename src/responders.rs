//#[derive(Responder)]
//#[response(status = 500, content_type = "json")]
//struct MyResponder {
//    inner: OtherResponder,
//    header: SomeHeader,
//    data: String,
//    #[response(ignore)]
//    unrelated: MyType,
//}
//
//impl Responder<'static> for String {
//    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
//        Response::build()
//            .header(ContentType::Plain)
//            .sized_body(Cursor::new(self))
//            .ok()
//    }
//}


use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct CompDiveForBarChart {
    pub label: String,
    pub value: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompDivesForBarChart {
    pub comp_dives_for_bar_chart: Vec<CompDiveForBarChart>
}
