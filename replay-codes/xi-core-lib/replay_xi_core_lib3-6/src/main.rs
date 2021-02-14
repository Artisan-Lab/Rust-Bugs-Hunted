extern crate xi_core_lib;

fn main() {
    let _param0 = 9236935819261915184;
    let _param1 = 10750145294058401840;
    let _param2 = 10750145294058401840;
    let _local0 = xi_core_lib::line_cache_shadow::Builder::new();
    let _local1 = xi_core_lib::line_cache_shadow::Builder::build(_local0);
    let _local2 = xi_core_lib::line_cache_shadow::RenderPlan::create(_param0 ,_param1 ,_param2);
}
//thread 'main' panicked at 'attempt to add with overflow', /home/jjf/xi-editor/rust/core-lib/src/line_cache_shadow.rs:267:30