extern crate xi_core_lib;

fn main() {
    let _param0 = 3472328296227680304;
    let _param1 = 3472328296227680304;
    let _param2 = 3472328296227680304;
    let _param3 = 3472328296227680304;
    let _param4 = 85621376445067312;
    let mut _local0 = xi_core_lib::line_cache_shadow::RenderPlan::create(_param0 ,_param1 ,_param2);
    let _ = xi_core_lib::line_cache_shadow::RenderPlan::request_lines(&mut (_local0) ,_param3 ,_param4);
}
//thread 'main' panicked at 'attempt to subtract with overflow', /home/jjf/xi-editor/rust/core-lib/src/line_cache_shadow.rs:299:21