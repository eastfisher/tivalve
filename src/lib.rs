mod handler;
mod model;
mod writer;

use wapc_guest as wapc;

use handler::*;

#[no_mangle]
pub fn wapc_init() {
    wapc::register_function("sink_add_table", sink_add_table);
    wapc::register_function("sink_emit_row_changed_events", sink_emit_row_changed_events);
    wapc::register_function("sink_emit_ddl_event", sink_emit_ddl_event);
    wapc::register_function("sink_remove_table", sink_remove_table);
}
