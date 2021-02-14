extern crate xi_core_lib;

fn main() {
    let _param0 = 3472328296227680304;
    let _param1 = 13527612320720337851;
    let _param2 = 13527612320720337851;
    let _param3 = 3472328296227680304;
    let _param4 = false;
    let _local0 = xi_core_lib::selection::SelRegion::new(_param0 ,_param1);
    let mut _local1 = xi_core_lib::selection::Selection::new_simple(_local0);
    let _ = xi_core_lib::selection::Selection::delete_range(&mut (_local1) ,_param2 ,_param3 ,_param4);
}
//thread 'main' panicked at 'attempt to subtract with overflow', /home/jjf/xi-editor/rust/core-lib/src/selection.rs:155:47
