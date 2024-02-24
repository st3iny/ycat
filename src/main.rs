use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::{fs::File, io::stdout};

use anyhow::Result;

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
        let mut empty = true;
        let mut starts_with_sep = false;

        let file = File::open(&path)?;
        let lines = BufReader::new(file).lines();
        for line in lines {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            if line == "---" {
                starts_with_sep = true;
            }

            empty = false;
            break;
        }

        if empty {
            continue;
        }

        let mut file = File::open(path)?;
        if !starts_with_sep {
            writeln!(out, "---")?;
        }
        std::io::copy(&mut file, out)?;
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

        let actual = String::from_utf8(buf).unwrap();
        let expected = read_to_string(expected).unwrap();
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
        test_actual_expected(
            [YAML_EMPTY].into_iter(),
            "testdata.out/test_empty_file.yaml",
        );
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
            "testdata.out/test_empty_and_regular.yaml",
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
