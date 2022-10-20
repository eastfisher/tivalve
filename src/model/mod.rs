mod row_event;

use serde::Serialize;
use serde::Deserialize;

pub(crate) use row_event::*;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RespStatus {
    pub code: i32,
    pub message: String,
    pub cause: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CommonExecResp {
    #[serde(flatten)]
    pub resp_status: RespStatus,
}

impl CommonExecResp {
    pub fn success() -> Self {
        Self{ resp_status: RespStatus {
            code: 200,
            message: "success".to_string(),
            cause: None
        } }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddTableReq {
    pub table_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RemoveTableReq {
    pub table_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() -> anyhow::Result<()> {
        let resp = "{\"code\":200,\"message\":\"ok\"}";
        let obj = serde_json::from_str::<CommonExecResp>(resp)?;
        assert_eq!(200, obj.resp_status.code);
        assert_eq!("ok", obj.resp_status.message);
        Ok(())
    }
}
