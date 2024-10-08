use pyo3::prelude::*;
use pyo3::types::PyTuple;
use steel::steel_vm::engine::Engine;
use steel::SteelVal;

fn main() {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    let x: PyResult<_> = Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code_bound(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // call object without any arguments
        fun.call0(py)?;

        // pass object with Rust tuple of positional arguments
        let args = (arg1, arg2, arg3);
        fun.call1(py, args)?;

        // call object with Python tuple of positional arguments
        let args = PyTuple::new_bound(py, &[arg1, arg2, arg3]);
        fun.call1(py, args)?;
        Ok(())
    });

    let steel_engine = Engine::new();
    steel_repl::run_repl(steel_engine).unwrap();
}