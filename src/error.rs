use regex::Error as RegexError;
use semver::SemVerError;
use std::io::Error as IOError;
use std::string::FromUtf8Error;
use toml::de::Error as TomlError;

quick_error! {
    #[derive(Debug)]
    pub enum FatalError {
        IOError(err: IOError) {
            from()
            cause(err)
            description(err.description())
            display("IO Error: {}", err)
        }
        InvalidCargoFileFormat(err: TomlError) {
            display("Invalid TOML file format: {}", err)
            description(err.description())
            from()
            cause(err)
        }
        InvalidCargoConfigKeys {
            display("Invalid cargo-release config item found")
            description("Invalid cargo-release config item found")
        }
        SemVerError(err: SemVerError) {
            from()
            cause(err)
            display("SemVerError {}", err)
            description(err.description())
        }
        FromUtf8Error(err: FromUtf8Error) {
            from()
            cause(err)
            display("FromUtf8Error {}", err)
            description(err.description())
        }
        InvalidReleaseLevel(level: String) {
            display("Unsupported release level {}", level)
            description("Unsupported release level, only major, minor and patch are supported")
        }
        UnsupportedPrereleaseVersionScheme {
            display("This version scheme is not supported by cargo-release.")
            description("This version scheme is not supported by cargo-release. Use format like `pre`, `dev` or `alpha.1` for prerelease symbol")
        }
        ReplacerConfigError {
            display("Insuffient replacer config: file, search and replace are required.")
            description("Insuffient replacer config: file, search and replace are required.")
        }
        ReplacerRegexError(err: RegexError) {
            from()
            cause(err)
            display("RegexError {}", err)
            description(err.description())
        }
    }
}
