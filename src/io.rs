pub mod marks {
    pub const INFO: &str = "\u{1b}[1;94mℹ️\u{1b}[0m";
    pub const WARN: &str = "\u{1b}[1;33m⚠️\u{1b}[0m";
    pub const ERROR: &str = "\u{1b}[1;91m❌\u{1b}[0m";
    pub const SUCCESS: &str = "\u{1b}[1;92m✅\u{1b}[0m";
    pub const WAIT: &str = "\u{1b}[1;35m⌛\u{1b}[0m";
}

/// Prints log message to stderr with a custom head without new line.
///
/// # Examples
///
/// ```
/// use hand::*;
/// use colored::*;
///
/// custom!("⚠️".yellow().bold(), "error message"); // ⚠️ error message
/// custom!("🔍".bold(), "searching for something {} ... ", 11); // 🔍 searching for something 11
///
/// ```
#[cfg(not(test))]
#[macro_export]
macro_rules! custom {
    ($head:expr, $($arg:tt)*) => { eprint!("{} {}", $head, format_args!($($arg)*)) }
}

#[cfg(test)]
macro_rules! custom {
    ($head:expr, $($arg:tt)*) => { format!("{} {}", $head, format_args!($($arg)*)) }
}

/// Prints log message to stderr with a custom head with new line.
///
/// # Examples
///
/// ```
/// use hand::*;
/// use colored::*;
///
/// customln!("#".bright_yellow().bold(), "Task completed"); // # Task completed
/// customln!("@", "The dog is seeking for you ... {}", "too long"); // @ The dog is seeking for you ... too long
/// ```
#[macro_export]
macro_rules! customln {
    ($head:expr, $($arg:tt)*) => { custom!($head, "{}\n", format_args!($($arg)*)) }
}

/// Prints log message to stderr with a custom prefix and head without new line.
///
/// # Examples
/// ```
/// use hand::*;
/// use colored::*;
///
/// scopecustom!("Fetching", "🌐", "fetching data from {} ... ", "www.example.com"); // [Fetching] 🌐 fetching data from www.example.com ...
/// scopecustom!("Scanning", "🚨".bright_red().bold(), "{} viruses detected ... ", 3); // [Scanning] 🚨 3 viruses detected ...
/// ```
#[cfg(not(test))]
#[macro_export]
macro_rules! scopecustom {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        eprint!("\u{1b}[2m[{}]\u{1b}[0m {} {}", $prefix, $head, format_args!($($arg)*))
    }
}

#[cfg(test)]
macro_rules! scopecustom {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        format!("\u{1b}[2m[{}]\u{1b}[0m {} {}", $prefix, $head, format_args!($($arg)*))
    }
}

/// Prints log message to stderr with a custom prefix and head with new line.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopecustomln!("Phone", "🔋", "charging is done"); // [Phone] 🔋 charging is done
/// scopecustomln!("www.download.com", "💾", "file saved to {}", "./downloads"); // [www.download.com] 💾 file saved to ./downloads
/// ```
#[macro_export]
macro_rules! scopecustomln {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        scopecustom!($prefix, $head, "{}\n", format_args!($($arg)*))
    }
}


/// Prints info log message to stderr without new line.
///
/// # Examples
/// ```
/// use hand::*;
///
/// info!("testing info ..."); // ℹ testing info ...
/// info!("testing info for {} users", 4); // ℹ testing info for 4 users
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => { custom!($crate::io::marks::INFO, $($arg)*) }
}

/// Prints info log message to stderr with a new line.
///
/// # Examples
/// ```
/// use hand::*;
///
/// infoln!("Waiting all jobs to complete"); // ℹ Waiting all jobs to complete
/// infoln!("Next checking will be at {}", "10:00 AM"); // ℹ Next checking will be at 10:00 AM
/// ```
#[macro_export]
macro_rules! infoln {
    ($($arg:tt)*) => { customln!($crate::io::marks::INFO, $($arg)*) }
}

/// Prints log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopeinfo!("working hard", "long time no see ..."); // [working hard] ℹ long time no see ...
/// scopeinfo!("eta: 1 hour", "waiting file `{}` to download ... ", "tlauncher.exe"); // [eta: 1 hour] ℹ waiting file `tlauncher.exe` to download ...
/// ```
#[macro_export]
macro_rules! scopeinfo {
    ($prefix:expr, $($arg:tt)*) => { scopecustom!($prefix, $crate::io::marks::INFO, $($arg)*) }
}

/// Prints log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopeinfoln!("Building", "Not completed"); // [Building] ℹ Not completed
/// scopeinfoln!("Dogs", "Dogs are {}", if true { "good" } else { "bad" }); // [Dogs] ℹ Dogs are good
/// ```
#[macro_export]
macro_rules! scopeinfoln {
    ($prefix:expr, $($arg:tt)*) => { scopecustomln!($prefix, $crate::io::marks::INFO, $($arg)*) }
}

/// Prints warn log message to stderr without a new line.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// warn!("test warn"); // ⚠ test warn
/// warn!("no more warnings {}", "i said"); // ⚠ no more warnings i said
/// ```
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => { custom!($crate::io::marks::WARN, $($arg)*) }
}

/// Prints warn log message to stderr with a new line.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// warnln!("You have not weared a mask"); // ⚠ You have not weared a mask
/// warnln!("You have not sent {} dollars to you mom", 1000); // ⚠ You have not sent 1000 dollars to you mom
/// ```
#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => { customln!($crate::io::marks::WARN, $($arg)*) }
}

/// Prints log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopewarn!("driving", "computing the distance ... "); // [driving] ⚠ computing the distance ...
/// scopewarn!("fixing", "fixing the problem ... "); // [fixing] ⚠ fixing the problem ...
/// ```
#[macro_export]
macro_rules! scopewarn {
    ($prefix:expr, $($arg:tt)*) => { scopecustom!($prefix, $crate::io::marks::WARN, $($arg)*) }
}

/// Prints log message to stderr with a new line, with a specified prefix.
///
/// # Examples
/// ```
/// use hand::*;
///
/// scopewarnln!("car", "the problem fixed"); // [car] ⚠ the problem fixed
/// scopewarnln!("boilerplate", "your code has too many boilerplate"); // [boilerplate] ⚠ your code has too many boilerplate
/// ```
#[macro_export]
macro_rules! scopewarnln {
    ($prefix:expr, $($arg:tt)*) => { scopecustomln!($prefix, $crate::io::marks::WARN, $($arg)*) }
}

/// Prints a log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// success!("Operation successful"); // ✔ Operation successful
/// success!("Data processed"); // ✔ Data processed
/// ```
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => { custom!($crate::io::marks::SUCCESS, $($arg)*) }
}

/// Prints a log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// successln!("Task completed"); // ✔ Task completed
/// successln!("Process finished"); // ✔ Process finished
/// ```
#[macro_export]
macro_rules! successln {
    ($($arg:tt)*) => { customln!($crate::io::marks::SUCCESS, $($arg)*) }
}

/// Prints a log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopesuccess!("Installing", "Completed in {} secs.", 9);
/// waitln!("Reboot in 3 seconds");
/// // [Installing] ✔ Completed in 9 secs. ⌛ Reboot in 3 seconds
/// ```
#[macro_export]
macro_rules! scopesuccess {
    ($prefix:expr, $($arg:tt)*) => { scopecustom!($prefix, $crate::io::marks::SUCCESS, $($arg)*) }
}

/// Prints a log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopesuccessln!("Deploy", "Finished in {} secs.", 9); // [Deploy] ✔ Finished in 9 secs.
/// scopesuccessln!("Cleaning up", "Finished"); // [Cleaning up] ✔ Finished
/// ```
#[macro_export]
macro_rules! scopesuccessln {
    ($prefix:expr, $($arg:tt)*) => { scopecustomln!($prefix, $crate::io::marks::SUCCESS, $($arg)*) }
}

/// Prints a log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// error!("An error occurred"); // ❌ An error occurred
/// error!("Invalid input"); // ❌ Invalid input
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => { custom!($crate::io::marks::ERROR, $($arg)*) }
}

/// Prints a log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// errorln!("Critical error: {} {} seconds", "your pc will die in", 3); // ❌ Critical error: your pc will die in 3 seconds
/// errorln!("Fatal error occurred"); // ❌ Fatal error occurred
/// ```
#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => { customln!($crate::io::marks::ERROR, $($arg)*) }
}

/// Prints a log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopeerror!("github.com", "Unable to fetch. Retrying ... ");
/// successln!("Retrying successful")
/// // [github.com] ❌ Unable to fetch. Retrying ... ✅ Retrying successful
/// ```
#[macro_export]
macro_rules! scopeerror {
    ($prefix:expr, $($arg:tt)*) => { scopecustom!($prefix, $crate::io::marks::ERROR, $($arg)*) }
}

/// Prints a log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopeerrorln!("FATAL", "Your GPU died"); // [FATAL] ❌ Your GPU died
/// scopeerrorln!("FATAL", "Your motherboard blow up"); // [FATAL] ❌ Your motherboard blow up
/// ```
#[macro_export]
macro_rules! scopeerrorln {
    ($prefix:expr, $($arg:tt)*) => { scopecustomln!($prefix, $crate::io::marks::ERROR, $($arg)*) }
}

/// Prints a log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// wait!("Waiting for input"); // ⌛ Waiting for input
/// wait!("Processing data"); // ⌛ Processing data
/// ```
#[macro_export]
macro_rules! wait {
    ($($arg:tt)*) => { custom!($crate::io::marks::WAIT, $($arg)*) }
}

/// Prints a log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// waitln!("This operation can take a while"); // ⌛ This operation can take a while
/// waitln!("Fetching results"); // ⌛ Fetching results
/// ```
#[macro_export]
macro_rules! waitln {
    ($($arg:tt)*) => { customln!($crate::io::marks::WAIT, $($arg)*) }
}

/// Prints a log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopewait!("reading config", "reading config ... ");
/// successln!("done in {} secs", 13.578);
/// // [reading config] ⌛ reading config ... ✅ done in 13.578 secs
/// ```
#[macro_export]
macro_rules! scopewait {
    ($prefix:expr, $($arg:tt)*) => { scopecustom!($prefix, $crate::io::marks::WAIT, $($arg)*) }
}

/// Prints a log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// scopewaitln!("Documenting", "Wait until Give me an Oscar will be done"); // [Documenting] ⌛ Wait until Give me an Oscar will be done
/// scopewaitln!("Testing", "Wait for the test to be done"); // [Testing] ⌛ Wait for the test to be done
/// ```
#[macro_export]
macro_rules! scopewaitln {
    ($prefix:expr, $($arg:tt)*) => { scopecustomln!($prefix, $crate::io::marks::WAIT, $($arg)*) }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn output() {
        println!("\nOUTPUTS\n");

        print!("{}", info!("info\n"));
        print!("{}", infoln!("infoln"));
        print!("{}", scopeinfo!("scope", "scopeinfo\n"));
        print!("{}", scopeinfoln!("scope", "scopeinfoln"));

        print!("{}", warn!("warn\n"));
        print!("{}", warnln!("warnln"));
        print!("{}", scopewarn!("scope", "scopewarn\n"));
        print!("{}", scopewarnln!("scope", "scopewarnln"));

        print!("{}", success!("success\n"));
        print!("{}", successln!("successln"));
        print!("{}", scopesuccess!("scope", "scopesuccess\n"));
        print!("{}", scopesuccessln!("scope", "scopesuccessln"));

        print!("{}", error!("error\n"));
        print!("{}", errorln!("errorln"));
        print!("{}", scopeerror!("scope", "scopeerror\n"));
        print!("{}", scopeerrorln!("scope", "scopeerrorln"));

        println!("\n");

        // remove dashes to show the output
        // assert!(false);
    }

    #[test]
    fn info() {
        assert_eq!(
            info!("test info"),
            "\u{1b}[1;94mℹ\u{1b}[0m test info"
        );
        assert_eq!(
            infoln!("test infoln"),
            "\u{1b}[1;94mℹ\u{1b}[0m test infoln\n"
        );
        assert_eq!(
            scopeinfo!("preinfo", "some formatting {}", 12.333 as f32),
            "\u{1b}[2m[preinfo]\u{1b}[0m \u{1b}[1;94mℹ\u{1b}[0m some formatting 12.333"
        );
        assert_eq!(
            scopeinfoln!("preinfoln", "some formatting {}", 123),
            "\u{1b}[2m[preinfoln]\u{1b}[0m \u{1b}[1;94mℹ\u{1b}[0m some formatting 123\n"
        );
    }

    #[test]
    fn warn() {
        assert_eq!(
            warn!("test warn"),
            "\u{1b}[1;33m⚠\u{1b}[0m test warn"
        );
        assert_eq!(
            warnln!("test warnln"),
            "\u{1b}[1;33m⚠\u{1b}[0m test warnln\n"
        );
        assert_eq!(
            scopewarn!("prewarn", "some formatting {}", 12.333 as f32),
            "\u{1b}[2m[prewarn]\u{1b}[0m \u{1b}[1;33m⚠\u{1b}[0m some formatting 12.333"
        );
        assert_eq!(
            scopewarnln!("prewarnln", "some formatting {}", 123),
            "\u{1b}[2m[prewarnln]\u{1b}[0m \u{1b}[1;33m⚠\u{1b}[0m some formatting 123\n"
        );
    }

    #[test]
    fn success() {
        assert_eq!(
            success!("test success"),
            "\u{1b}[1;92m✅\u{1b}[0m test success"
        );
        assert_eq!(
            successln!("test successln"),
            "\u{1b}[1;92m✅\u{1b}[0m test successln\n"
        );
        assert_eq!(
            scopesuccess!("presuccess", "some formatting {}", 12.333 as f32),
            "\u{1b}[2m[presuccess]\u{1b}[0m \u{1b}[1;92m✅\u{1b}[0m some formatting 12.333"
        );
        assert_eq!(
            scopesuccessln!("presuccessln", "some formatting {}", 123),
            "\u{1b}[2m[presuccessln]\u{1b}[0m \u{1b}[1;92m✅\u{1b}[0m some formatting 123\n"
        );
    }

    #[test]
    fn error() {
        assert_eq!(
            error!("test error"),
            "\u{1b}[1;91m❌\u{1b}[0m test error"
        );
        assert_eq!(
            errorln!("test errorln"),
            "\u{1b}[1;91m❌\u{1b}[0m test errorln\n"
        );
        assert_eq!(
            scopeerror!("preerror", "some formatting {}", 12.333 as f32),
            "\u{1b}[2m[preerror]\u{1b}[0m \u{1b}[1;91m❌\u{1b}[0m some formatting 12.333"
        );
        assert_eq!(
            scopeerrorln!("preerrorln", "some formatting {}", 123),
            "\u{1b}[2m[preerrorln]\u{1b}[0m \u{1b}[1;91m❌\u{1b}[0m some formatting 123\n"
        );
    }
}