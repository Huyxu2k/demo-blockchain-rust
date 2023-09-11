
#[derive(Debug,Clone)]
pub struct ResponseBody<T>{
     message:String,
     data:T
}
impl <T> ResponseBody<T> {
    pub fn new(message:String,data:T)->ResponseBody<T>{
        ResponseBody { 
            message,
            data
        }
    }
}