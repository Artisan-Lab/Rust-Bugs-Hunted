extern crate xi_core_lib;

fn main() {
    let mut _local0 = xi_core_lib::selection::Selection::new();
    let _ = xi_core_lib::selection::Selection::collapse(&mut (_local0));
}
//thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /home/jjf/xi-editor/rust/core-lib/src/selection.rs:72:33