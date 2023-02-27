//! Core structures for desribing tasks and arguments
//! 

use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::sync::Mutex;

/// Registers the Command and Arg structs to the python api in the `angreal` module
pub fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<AngrealCommand>()?;
    m.add_class::<AngrealArg>()?;
    Ok(())
}

/// A long lived structure that stores AngrealCommands upon registration
pub static ANGREAL_TASKS: Lazy<Mutex<Vec<AngrealCommand>>> = Lazy::new(|| Mutex::new(vec![]));

/// A long lived structure that stores AngrealArgs for commands upon registration
pub static ANGREAL_ARGS: Lazy<Mutex<Vec<AngrealArg>>> = Lazy::new(|| Mutex::new(vec![]));


/// A command describes a subcommand to be registered with the CLI
#[derive(Clone, Debug)]
#[pyclass(name = "Command")]
pub struct AngrealCommand {
    /// The name of the sub command 
    #[pyo3(get)]
    pub name: String,
    /// A short description of what the command does 
    #[pyo3(get)]
    pub about: Option<String>,
    /// A longer description of what the command does
    #[pyo3(get)]
    pub long_about: Option<String>,
    /// The actual function that is exectuted when the command is run
    #[pyo3(get)]
    pub func: Py<PyAny>,
}


/// Methods exposed to the python API
#[pymethods]
impl AngrealCommand {

    /// Initialization method for the object. The command is registered to `ANGREAL_TASKS` upon instantiation from the python api
    /// 
    /// 
    /// The decorated version is the most ergonmoic way to use this object.
    /// # Example
    /// ```python
    /// import angreal
    /// 
    /// @angreal.command(name='test',about='a short message', 
    /// long_about='a much longer message`)
    /// def test-message():
    ///     pass
    /// ```
    /// 
    /// ```python
    /// import angreal
    /// 
    /// def test-message():
    ///     pass
    /// 
    /// angreal.Command(name='test',about='a short message', 
    /// long_about='a much longer message`, func=test-message)
    /// ```
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


/// An argument to augment the behavior of an angreal command
#[derive(Clone, Debug)]
#[pyclass(name = "Arg")]
pub struct AngrealArg {
    /// The name of the argument, required to match the name in the function being executed by the command
    #[pyo3(get)]
    pub name: String,
    /// The command name associated with this argument
    #[pyo3(get)]
    pub command_name: String,
    /// Whether or not the argument consumes a value from the command line 
    #[pyo3(get)]
    pub takes_value: Option<bool>,
    /// The default value to be applied to the arg.
    #[pyo3(get)]
    pub default_value: Option<String>,
    /// Whether or not the argument requires an `=` behind it to set a value
    #[pyo3(get)]
    pub require_equals: Option<bool>,
    /// Whether or not the argument takes multiple values
    #[pyo3(get)]
    pub multiple_values: Option<bool>,
    /// The number of values the argument takes
    #[pyo3(get)]
    pub number_of_values: Option<u32>,
    /// The maximum number of values the argument takes
    #[pyo3(get)]
    pub max_values: Option<u32>,
    /// The minimum number of values the argument takes
    #[pyo3(get)]
    pub min_values: Option<u32>,
    /// The python type to apply the the consumed value (int, string, float)
    #[pyo3(get)]
    pub python_type: Option<String>,
    /// the short flag to be used on the command line (i.e. `-s`)
    #[pyo3(get)]
    pub short: Option<char>,
    /// the long flag to be used on the command line (i.e. `--long`)
    #[pyo3(get)]
    pub long: Option<String>,
    /// A verbose help message to be displayed
    #[pyo3(get)]
    pub long_help: Option<String>,
    /// a shorter help message to be displayed
    #[pyo3(get)]
    pub help: Option<String>,
    /// whether or not the argument is required 
    #[pyo3(get)]
    pub required: Option<bool>,
}



#[pymethods]
impl AngrealArg {
    /// Adds an argument to an angreal command.
    ///
    /// The decorated version is the most ergonmoic way to use this object.
    /// # Example
    /// ```python
    /// import angreal
    /// 
    /// @angreal.command(name='echo',about='a needless echo replacement',
    /// @angreal.argument(name="phrase", help="the phrase to echo", required=True)
    ///    def task_echo(phrase):
    ///        print(phrase)
    /// ```
    /// 
    /// ```python
    /// import angreal
    /// 
    /// def echo(phrase):
    ///     print(phrase)
    /// 
    /// angreal.Command(name='echo',about='a needless echo replacement', func=test-message)
    /// angreal.Arg(name="phrase", help="the phrase to echo", required=True, command_name='echo')
    /// ```
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
        short = "None",
        long = "None",
        long_help = "None",
        help = "None",
        required = "None"
    )]
    #[allow(clippy::too_many_arguments)]
    fn __new__(
        name: &str,
        command_name: &str,
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
            command_name: command_name.to_string(),
            takes_value,
            default_value: default_value.map(|i| i.to_string()),
            require_equals,
            multiple_values,
            number_of_values,
            max_values,
            min_values,
            python_type: python_type.map(|i| i.to_string()),
            short,
            long: long.map(|i| i.to_string()),
            long_help: long_help.map(|i| i.to_string()),
            help: help.map(|i| i.to_string()),
            required,
        };
        ANGREAL_ARGS.lock().unwrap().push(arg.clone());
        arg
    }
}
