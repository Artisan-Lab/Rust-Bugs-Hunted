extern crate xi_core_lib;

fn main() {
    let _param0 = 3472328296227680352;
    let _param1 = 3472328296227692592;
    let _param2 = 3472328502386110512;
    let _param3 = 3472328296227680304;
    let _local0 = xi_core_lib::selection::SelRegion::new(_param0 ,_param1);
    let _local1 = xi_core_lib::selection::Selection::new_simple(_local0);
    let _ = xi_core_lib::selection::Selection::regions_in_range(&(_local1) ,_param2 ,_param3);
}
//thread 'main' panicked at 'slice index starts at 1 but ends at 0', /home/jjf/xi-editor/rust/core-lib/src/selection.rs:136:10
