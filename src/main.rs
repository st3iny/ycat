use std::io::Write;
use std::path::Path;
use std::{fs::File, io::stdout};

use anyhow::Result;
use serde::Deserialize;
use serde_yaml::Value;

fn main() -> Result<()> {
    for arg in std::env::args().skip(1) {
        if arg == "--help" || arg == "-h" {
            print_help();
            return Ok(());
        }
    }

    concatenate_yaml_files(std::env::args().skip(1), &mut stdout())
}

fn print_help() {
    println!("Concatenate multiple YAML files into a single YAML stream and write it to stdout.");
    println!();
    println!("Usage: {} FILE [FILE ...]", env!("CARGO_BIN_NAME"));
}

fn concatenate_yaml_files<P>(paths: impl Iterator<Item = P>, out: &mut impl Write) -> Result<()>
where
    P: AsRef<Path>,
{
    for path in paths {
        let mut file = File::open(path)?;
        for document in serde_yaml::Deserializer::from_reader(&mut file) {
            let value = Value::deserialize(document)?;
            if value.is_null() {
                continue;
            }

            writeln!(out, "---")?;
            serde_yaml::to_writer(&mut *out, &value)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    static YAML_REGULAR: &str = "testdata.in/regular.yaml";
    static YAML_REGULAR_2X: &str = "testdata.in/regular-2x.yaml";
    static YAML_REGULAR_START_SEP: &str = "testdata.in/regular-start-sep.yaml";
    static YAML_REGULAR_END_SEP: &str = "testdata.in/regular-end-sep.yaml";
    static YAML_REGULAR_BOTH_SEP: &str = "testdata.in/regular-both-sep.yaml";
    static YAML_EMPTY: &str = "testdata.in/empty.yaml";
    static YAML_EMPTY_1_SEP: &str = "testdata.in/empty-1-sep.yaml";
    static YAML_EMPTY_2_SEP: &str = "testdata.in/empty-2-sep.yaml";

    fn test_actual_expected(paths: impl Iterator<Item = &'static str>, expected: impl AsRef<Path>) {
        let mut buf = Vec::new();
        concatenate_yaml_files(paths, &mut buf).unwrap();

        #[cfg(feature = "gentest")]
        if !expected.as_ref().exists() {
            let mut file = File::create(expected.as_ref()).unwrap();
            file.write_all(&buf).unwrap();
        }

        let actual: Vec<Value> = serde_yaml::Deserializer::from_slice(&buf)
            .filter_map(|document| Value::deserialize(document).ok())
            .collect();

        let expected = read_to_string(expected).unwrap();
        let expected: Vec<Value> = serde_yaml::Deserializer::from_str(&expected)
            .filter_map(|document| Value::deserialize(document).ok())
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_no_paths() {
        let mut buf = Vec::new();
        concatenate_yaml_files(std::iter::empty::<&'static str>(), &mut buf).unwrap();
        assert!(buf.is_empty());
    }

    #[test]
    fn test_empty_file() {
        test_actual_expected([YAML_EMPTY].into_iter(), "testdata.out/test_empty.yaml");
    }

    #[test]
    fn test_regular() {
        test_actual_expected([YAML_REGULAR].into_iter(), "testdata.out/test_regular.yaml");
    }

    #[test]
    fn test_regular_2x() {
        test_actual_expected(
            [YAML_REGULAR_2X].into_iter(),
            "testdata.out/test_regular_2x.yaml",
        );
    }

    #[test]
    fn test_regular_twice() {
        test_actual_expected(
            [YAML_REGULAR, YAML_REGULAR].into_iter(),
            "testdata.out/test_regular_twice.yaml",
        );
    }

    #[test]
    fn test_regular_and_empty() {
        test_actual_expected(
            [YAML_REGULAR, YAML_EMPTY_1_SEP].into_iter(),
            "testdata.out/test_regular_and_empty.yaml",
        );
    }

    #[test]
    fn test_empty_and_regular() {
        test_actual_expected(
            [YAML_EMPTY_1_SEP, YAML_REGULAR].into_iter(),
            "testdata.out/test_regular_and_empty.yaml",
        );
    }

    #[test]
    fn test_empty() {
        test_actual_expected(
            [YAML_EMPTY_1_SEP].into_iter(),
            "testdata.out/test_empty.yaml",
        );
    }

    #[test]
    fn test_empty_and_empty() {
        test_actual_expected(
            [YAML_EMPTY_1_SEP, YAML_EMPTY_2_SEP].into_iter(),
            "testdata.out/test_empty_and_empty.yaml",
        );
    }

    #[test]
    fn test_regular_start_sep() {
        test_actual_expected(
            [YAML_REGULAR_START_SEP].into_iter(),
            "testdata.out/test_regular_start_sep.yaml",
        );
    }

    #[test]
    fn test_regular_end_sep() {
        test_actual_expected(
            [YAML_REGULAR_END_SEP].into_iter(),
            "testdata.out/test_regular_end_sep.yaml",
        );
    }

    #[test]
    fn test_regular_both_sep() {
        test_actual_expected(
            [YAML_REGULAR_BOTH_SEP].into_iter(),
            "testdata.out/test_regular_both_sep.yaml",
        );
    }

    #[test]
    fn test_regular_both_sep_and_regular() {
        test_actual_expected(
            [YAML_REGULAR_BOTH_SEP, YAML_REGULAR].into_iter(),
            "testdata.out/test_regular_both_sep_and_regular.yaml",
        );
    }

    #[test]
    fn test_regular_both_sep_and_start_sep() {
        test_actual_expected(
            [YAML_REGULAR_BOTH_SEP, YAML_REGULAR_START_SEP].into_iter(),
            "testdata.out/test_regular_both_sep_and_start_sep.yaml",
        );
    }

    #[test]
    fn test_regular_both_sep_and_empty_and_start_sep() {
        test_actual_expected(
            [
                YAML_REGULAR_BOTH_SEP,
                YAML_EMPTY_1_SEP,
                YAML_REGULAR_START_SEP,
            ]
            .into_iter(),
            "testdata.out/test_regular_both_sep_and_empty_and_start_sep.yaml",
        );
    }

    #[test]
    fn test_regular_and_empty_and_regular() {
        test_actual_expected(
            [YAML_REGULAR, YAML_EMPTY_1_SEP, YAML_REGULAR].into_iter(),
            "testdata.out/test_regular_and_empty_and_regular.yaml",
        );
    }
}
