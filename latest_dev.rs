macro_rules! init_all_call_wrappers {
    () => {{
        telemetry_lib::telemetry_lib::preconditions();
        telemetry_lib::telemetry_lib::invariants();
        telemetry_lib::telemetry_lib::postconditions();
    }};
}
