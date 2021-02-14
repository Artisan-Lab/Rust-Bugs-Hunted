extern crate xi_core_lib;

fn main() {
    let _param0 = 3472328296227680304;
    let _param1 = 3472328296227680304;
    let _param2 = 3472328296227680304;
    let _param3 = 2823809949886328880;
    let _local0 = xi_core_lib::tabs::BufferId::new(_param0);
    let _local1 = xi_core_lib::tabs::test_helpers::new_view_id(_param1);
    let mut _local2 = xi_core_lib::view::View::new(_local1 ,_local0);
    let _ = xi_core_lib::view::View::set_scroll(&mut (_local2) ,_param2 ,_param3);
}
//thread 'main' panicked at 'attempt to subtract with overflow', /home/jjf/xi-editor/rust/core-lib/src/view.rs:333:23