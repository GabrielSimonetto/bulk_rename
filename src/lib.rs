use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use glob::glob;

/// Arguments defined by cli.
pub struct Args {
    /// "./test_inputs/star_copy_test_1/"
    pub parent_dir: String,
    /// *_bolovo.txt
    pub input_pattern: String,
    /// *_abacaxi.txt    
    pub output_pattern: String,
}

impl Args {
    pub fn new(mut args: env::Args) -> Result<Args, &'static str> {
        args.next();

        let parent_dir = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a parent_dir string"),
        };

        let input_pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a input_pattern string"),
        };

        let output_pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a output_pattern string"),
        };

        Ok(Args {
            parent_dir,
            input_pattern,
            output_pattern,
        })
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    copy_command(args)
}

pub fn new_name(base_str: String, from: &str, to: &str) -> String {
    base_str.replace(from, to)
}

pub fn get_conversions(
    base_filenames: Vec<PathBuf>,
    from: &str,
    to: &str,
) -> Vec<(PathBuf, String)> {
    base_filenames
        .into_iter()
        .map(|filename| {
            (
                filename.clone(),
                new_name(filename.into_os_string().into_string().unwrap(), from, to),
            )
        })
        .collect::<Vec<(PathBuf, String)>>()
}

pub fn copy_command(args: Args) -> Result<(), Box<dyn Error>> {
    let base_paths: Vec<PathBuf> = glob(&format!("{}{}", args.parent_dir, args.input_pattern))
        .expect("Failed to read glob pattern")
        .map(|path_res| path_res.unwrap())
        .collect();

    println!("{:?}", base_paths);

    // It's naive because assumes things have the same order.
    let naive_replace_from = args.input_pattern.replace("*", "");
    let naive_replace_to = args.output_pattern.replace("*", "");

    let conversions = get_conversions(base_paths, &naive_replace_from, &naive_replace_to);

    conversions
        .iter()
        .map(|(from, to)| fs::copy(from, to))
        .for_each(drop);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_simple_composite_conversions() {
        let base_filenames: Vec<PathBuf> = vec![
            "./test_inputs/star_copy_test_1/input_abacaxi.txt",
            "./test_inputs/star_copy_test_1/output_abacaxi.txt",
            "./test_inputs/star_copy_test_1/intermediary_abacaxi.txt",
        ]
        .iter()
        .map(|str| PathBuf::from(str))
        .collect();

        let conversions = get_conversions(base_filenames, "abacaxi", "bolovo");

        assert_eq!(
            vec![
                (
                    PathBuf::from("./test_inputs/star_copy_test_1/input_abacaxi.txt"),
                    String::from("./test_inputs/star_copy_test_1/input_bolovo.txt")
                ),
                (
                    PathBuf::from("./test_inputs/star_copy_test_1/output_abacaxi.txt"),
                    String::from("./test_inputs/star_copy_test_1/output_bolovo.txt")
                ),
                (
                    PathBuf::from("./test_inputs/star_copy_test_1/intermediary_abacaxi.txt"),
                    String::from("./test_inputs/star_copy_test_1/intermediary_bolovo.txt")
                ),
            ],
            conversions
        );
    }

    #[test]
    fn test_create_simple_local_conversions() {
        let base_filenames: Vec<PathBuf> = vec![
            "input_abacaxi.txt",
            "output_abacaxi.txt",
            "intermediary_abacaxi.txt",
        ]
        .iter()
        .map(|str| PathBuf::from(str))
        .collect();

        let conversions = get_conversions(base_filenames, "abacaxi", "bolovo");

        assert_eq!(
            vec![
                (
                    PathBuf::from("input_abacaxi.txt"),
                    String::from("input_bolovo.txt")
                ),
                (
                    PathBuf::from("output_abacaxi.txt"),
                    String::from("output_bolovo.txt")
                ),
                (
                    PathBuf::from("intermediary_abacaxi.txt"),
                    String::from("intermediary_bolovo.txt")
                ),
            ],
            conversions
        );
    }
}
