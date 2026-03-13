use std::path::Path;

use archforge::runtime::Runtime;

fn main() {
    let mut runtime = Runtime::new();

    runtime.execute_script(Path::new("example.forge"));

    runtime.call_function("main", vec![]);
}
