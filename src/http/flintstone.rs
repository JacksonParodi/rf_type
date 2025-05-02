pub enum FlintstoneData {
    Get,
    Post(u64),
}

pub struct FlintstoneRequest {
    pub data: FlintstoneData,
}

impl FlintstoneRequest {
    pub fn get() -> FlintstoneRequest {
        FlintstoneRequest {
            data: FlintstoneData::Get,
        }
    }
    pub fn post(data: u64) -> FlintstoneRequest {
        FlintstoneRequest {
            data: FlintstoneData::Post(data),
        }
    }
}

pub struct FlintstoneResponse {
    pub count: u64,
}
