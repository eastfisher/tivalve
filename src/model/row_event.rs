use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RowChangedEvent {
    #[serde(rename = "start-ts")]
    pub start_ts: u64,
    #[serde(rename = "commit-ts")]
    pub commit_ts: u64,
    #[serde(rename = "row-id")]
    pub row_id: i64,
    pub table: Option<TableName>,
    // ignore ColInfos
    pub columns: Option<Vec<Option<Column>>>,
    #[serde(rename = "pre-columns")]
    pub pre_columns: Option<Vec<Option<Column>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TableName {
    #[serde(rename = "db-name")]
    pub schema: String,
    #[serde(rename = "tbl-name")]
    pub table: String,
    #[serde(rename = "tbl-id")]
    pub table_id: i64,
    #[serde(rename = "is-partition")]
    pub is_partition: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: u8,
    pub charset: String,
    pub flag: u64,
    pub value: serde_json::Value,
    pub default: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_event_deserialize() -> anyhow::Result<()> {
        let event = "[{\"start-ts\":1,\"commit-ts\":1,\"row-id\":1,\"table\":{\"db-name\":\"db1\",\"tbl-name\":\"tbl1\",\"tbl-id\":1,\"is-partition\":false},\"column-infos\":null,\"replica-id\":0,\"columns\":[{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":\"v11\",\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":111,\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":false,\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":\"v12\",\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":222,\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":true,\"default\":null}],\"pre-columns\":null},{\"start-ts\":1,\"commit-ts\":1,\"row-id\":1,\"table\":{\"db-name\":\"db1\",\"tbl-name\":\"tbl1\",\"tbl-id\":1,\"is-partition\":false},\"column-infos\":null,\"replica-id\":0,\"columns\":[{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":\"v21\",\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":111,\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":false,\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":\"v22\",\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":222,\"default\":null},{\"name\":\"\",\"type\":0,\"charset\":\"\",\"flag\":0,\"value\":true,\"default\":null}],\"pre-columns\":null}]";
        let ret = serde_json::from_str::<Vec<RowChangedEvent>>(event)?;
        Ok(())
    }
}
