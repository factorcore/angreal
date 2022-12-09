use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::sync::Mutex;

pub fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<AngrealCommand>()?;
    m.add_class::<AngrealArg>()?;
    Ok(())
}



pub static ANGREAL_TASKS: Lazy<Mutex<Vec<AngrealCommand>>> = Lazy::new(|| Mutex::new(vec![]));

pub static ANGREAL_ARGS: Lazy<Mutex<Vec<AngrealArg>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(Clone, Debug)]
#[pyclass(name = "Command")]
pub struct AngrealCommand {
    pub name: String,
    pub about: Option<String>,
    pub long_about: Option<String>,
    pub func: Py<PyAny>,
}

#[pymethods]
impl AngrealCommand {
    #[new]
    #[args(about = "None", long_about = "None")]
    fn __new__(name: &str, func: Py<PyAny>, about: Option<&str>, long_about: Option<&str>) -> Self {
        let cmd = AngrealCommand {
            name: name.to_string(),
            about: about.map(|i| i.to_string()),
            long_about: long_about.map(|i| i.to_string()),
            func,
        };
        ANGREAL_TASKS.lock().unwrap().push(cmd.clone());
        cmd
    }
}

#[derive(Clone, Debug)]
#[pyclass(name = "Arg")]
pub struct AngrealArg {
    pub name: String,
    pub command_name: Option<String>,
    pub takes_value: Option<bool>,
    pub default_value: Option<String>,
    pub require_equals: Option<bool>,
    pub multiple_values: Option<bool>,
    pub number_of_values: Option<u32>,
    pub max_values: Option<u32>,
    pub min_values: Option<u32>,
    pub python_type: Option<String>,
    pub short: Option<char>,
    pub long: Option<String>,
    pub long_help: Option<String>,
    pub help: Option<String>,
    pub required: Option<bool>,
}

#[pymethods]
impl AngrealArg {
    #[new]
    #[args(
        python_type = "\"str\"",
        takes_value = "true",
        default_value = "None",
        require_equals = "None",
        multiple_values = "None",
        number_of_values = "None",
        max_values = "None",
        min_values = "None",
        python_type = "None",
        short = "None",
        long = "None",
        long_help = "None",
        help = "None",
        required = "None",
    )]
    fn __new__(
        name: &str,
        command_name: Option<&str>,
        takes_value: Option<bool>,
        default_value: Option<&str>,
        require_equals: Option<bool>,
        multiple_values: Option<bool>,
        number_of_values: Option<u32>,
        max_values: Option<u32>,
        min_values: Option<u32>,
        python_type: Option<&str>,
        short: Option<char>,
        long: Option<&str>,
        long_help: Option<&str>,
        help: Option<&str>,
        required: Option<bool>,
    ) -> Self {
        let arg = AngrealArg {
            name: name.to_string(),
            command_name: command_name.map(|i| i.to_string()),
            takes_value: takes_value,
            default_value: default_value.map(|i| i.to_string()),
            require_equals: require_equals,
            multiple_values: multiple_values,
            number_of_values: number_of_values,
            max_values: max_values,
            min_values: min_values,
            python_type: python_type.map(|i| i.to_string()),
            short: short,
            long: long.map(|i| i.to_string()),
            long_help: long_help.map(|i| i.to_string()),
            help: help.map(|i| i.to_string()),
            required: required
        };
        ANGREAL_ARGS.lock().unwrap().push(arg.clone());
        arg
    }
}


#[cfg(test)]
#[path = "../tests"]
mod tests {

}