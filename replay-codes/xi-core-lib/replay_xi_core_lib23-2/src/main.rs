extern crate xi_core_lib;

fn main() {
    let mut _local0 = xi_core_lib::editor::Editor::new();
    let _ = xi_core_lib::editor::Editor::dec_revs_in_flight(&mut (_local0));
}
//thread 'main' panicked at 'attempt to subtract with overflow', /home/jjf/xi-editor/rust/core-lib/src/editor.rs:186:9