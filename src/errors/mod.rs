use std::fmt::Display;

pub mod compile_error;
pub mod parse_error;
pub mod s_expression_error;

pub use crate::source::Source;

pub enum ResultLog<TResult, TError: Display>
{
    Ok(TResult),
    Warn(TResult, Vec<TError>),
    Error(Vec<TError>, Vec<TError>),
}

pub trait ErrorTrait: Display
{
    fn get_source(&self) -> Option<&Source>
    {
        None
    }
    fn get_description(&self) -> Option<&str>
    {
        None
    }
    fn show(&self) -> bool
    {
        true
    }
}

const ICON_OK: &'static str = "   ";
const ICON_WARN: &'static str = "[?] WARNING:";
const ICON_ERROR: &'static str = "[!] ERROR:";

impl<TResult, TError: ErrorTrait> ResultLog<TResult, TError>
{
    pub fn new_warning(result: TResult, warning: TError) -> Self
    {
        ResultLog::Warn(result, vec![warning])
    }

    pub fn new_error(error: TError) -> Self
    {
        ResultLog::Error(vec![error], Vec::new())
    }

    pub fn maybe_warn(result: TResult, warnings: Vec<TError>) -> Self
    {
        if warnings.is_empty()
        {
            ResultLog::Ok(result)
        }
        else
        {
            ResultLog::Warn(result, warnings)
        }
    }

    pub fn maybe_error(result: TResult, warnings: Vec<TError>, errors: Vec<TError>) -> Self
    {
        match (errors.is_empty(), warnings.is_empty())
        {
            (false, _) => ResultLog::Error(errors, warnings),
            (true, false) => ResultLog::Warn(result, warnings),
            (true, true) => ResultLog::Ok(result),
        }
    }

    pub fn add_warning(self, warning: TError) -> Self
    {
        match self
        {
            ResultLog::Ok(result) => ResultLog::Warn(result, vec![warning]),
            ResultLog::Warn(result, mut warnings) =>
            {
                warnings.push(warning);
                ResultLog::Warn(result, warnings)
            }
            ResultLog::Error(errors, mut warnings) =>
            {
                warnings.push(warning);
                ResultLog::Error(errors, warnings)
            }
        }
    }

    pub fn add_error(self, error: TError) -> Self
    {
        match self
        {
            ResultLog::Ok(_result) => ResultLog::Error(vec![error], Vec::new()),
            ResultLog::Warn(_result, warnings) => ResultLog::Error(Vec::new(), warnings),
            ResultLog::Error(mut errors, warnings) =>
            {
                errors.push(error);
                ResultLog::Error(errors, warnings)
            }
        }
    }
}

const MAX_PRINT_RANGE: usize = 2;

fn print_base<TError: ErrorTrait>(items: &Vec<TError>, icon: &str)
{
    for item in items
    {
        if !item.show()
        {
            continue;
        }
        eprint!("{} {}", icon, item);

        match item.get_source()
        {
            Some(source) => eprint!(" @ line {}", source.get_start_line() + 1),
            None => (),
        }
        match item.get_description()
        {
            Some(description) => eprint!("\n\t'{}'", description),
            None => (),
        }
        match item.get_source()
        {
            Some(source) if !source.is_empty() =>
            {
                let target_line = source.get_start_line();

                let mut start_line = source.get_start_line();
                let mut end_line = source.get_end_line();

                if start_line == end_line
                {
                    if start_line > 0
                    {
                        start_line -= 1;
                    }
                    end_line += 1;
                }

                for (i, text_line) in source
                    .get_all_lines()
                    .iter()
                    .enumerate()
                    .skip(start_line)
                {
                    if i > end_line
                    {
                        eprint!("\n\t- {}\t\t...", i + 1);
                        break;
                    }
                    else if i == target_line
                    {
                        eprint!("\n\t|*{}*\t\t{}", i + 1, text_line);
                    }
                    else
                    {
                        eprint!("\n\t| {}\t\t{}", i + 1, text_line);
                    }
                }
            }
            _ => (),
        }

        eprint!("\n\n");
    }
}

pub fn print_warnings<TError: ErrorTrait>(warnings: &Vec<TError>)
{
    print_base(warnings, ICON_WARN);
}

pub fn print_errors<TError: ErrorTrait>(errors: &Vec<TError>)
{
    print_base(errors, ICON_ERROR);
}
