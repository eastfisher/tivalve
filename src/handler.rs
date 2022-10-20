use std::io::Write;

use wapc_guest as wapc;

use super::writer;
use super::model;

pub(crate) fn sink_add_table(msg: &[u8]) -> wapc::CallResult {
    // only log to host and do nothing now
    let req = serde_json::from_slice::<model::AddTableReq>(msg)?;
    wapc::console_log(&format!("receive sink_add_table: {:?}", req.table_id));

    let mut writer = writer::ADD_TABLE_WRITER.lock().unwrap();
    writer.write_all(msg)?;
    writer.write_all(b"\n")?;

    let exec_resp = model::CommonExecResp::success();
    Ok(serde_json::to_vec(&exec_resp)?)
}

pub(crate) fn sink_emit_row_changed_events(msg: &[u8]) -> wapc::CallResult {
    wapc::console_log(&format!("receive sink_emit_row_changed_events: {:?}", String::from_utf8_lossy(msg)));
    let req = serde_json::from_slice::<Vec<model::RowChangedEvent>>(msg)?;
    wapc::console_log(&format!("parse row changed events: {:?}", req));

    let mut writer = writer::ROW_EVENT_WRITER.lock().unwrap();
    for event in req {
        if let Some(columns) = event.columns {
            let row = columns.iter().map(|col| match col {
                Some(col) => col.value.to_string(),
                // Some(col) => col.value.as_ref().unwrap().to_string(),
                None => String::new(),
            }).collect::<Vec<_>>().join(",");
            writer.write_all(row.as_bytes())?;
            writer.write_all(b"\n")?;
        }
    }

    let exec_resp = model::CommonExecResp::success();
    Ok(serde_json::to_vec(&exec_resp)?)
}

pub(crate) fn sink_emit_ddl_event(msg: &[u8]) -> wapc::CallResult {
    // only log to host and do nothing now
    wapc::console_log(&format!("receive sink_emit_ddl_event: {:?}", String::from_utf8_lossy(msg)));
    let exec_resp = model::CommonExecResp::success();
    Ok(serde_json::to_vec(&exec_resp)?)
}

pub(crate) fn sink_remove_table(msg: &[u8]) -> wapc::CallResult {
    // only log to host and do nothing now
    let req = serde_json::from_slice::<model::RemoveTableReq>(msg)?;
    wapc::console_log(&format!("receive sink_remove_table: {:?}", req.table_id));
    let exec_resp = model::CommonExecResp::success();
    Ok(serde_json::to_vec(&exec_resp)?)
}
