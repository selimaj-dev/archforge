use archforge::runtime::Runtime;

fn main() {
    let mut runtime = Runtime::new();

    runtime.call_function("main", vec![]);
}
