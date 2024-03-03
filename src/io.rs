use colored::*;
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
    ($head:expr, $($arg:tt)*) => { eprint!("{} {}", $head, format_args!($($arg)*)); }
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
#[cfg(not(test))]
#[macro_export]
macro_rules! customln {
    ($head:expr, $($arg:tt)*) => { eprintln!("{} {}", $head, format_args!($($arg)*)); }
}

#[cfg(test)]
macro_rules! customln {
    ($head:expr, $($arg:tt)*) => {
        format!("{} {}\n", $head, format_args!($($arg)*))
    };
}

/// Prints log message to stderr with a custom prefix and head without new line.
///
/// # Examples
/// ```
/// use hand::*;
/// use colored::*;
///
/// precustom!("Fetching", "🌐", "fetching data from {} ... ", "www.example.com"); // [Fetching] 🌐 fetching data from www.example.com ...
/// precustom!("Scanning", "🚨".bright_red().bold(), "{} viruses detected ... ", 3); // [Scanning] 🚨 3 viruses detected ...
/// ```
#[cfg(not(test))]
#[macro_export]
macro_rules! precustom {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        eprint!("\u{1b}[2m[{}]\u{1b}[0m {} {}", $prefix, $head, format_args!($($arg)*));
    };
}

#[cfg(test)]
macro_rules! precustom {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        format!("\u{1b}[2m[{}]\u{1b}[0m {} {}", $prefix, $head, format_args!($($arg)*))
    };
}

/// Prints log message to stderr with a custom prefix and head with new line.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// precustomln!("Phone", "🔋", "charging is done"); // [Phone] 🔋 charging is done
/// precustomln!("www.download.com", "💾", "file saved to {}", "./downloads"); // [www.download.com] 💾 file saved to ./downloads
/// ```
#[cfg(not(test))]
#[macro_export]
macro_rules! precustomln {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        eprintln!("\u{1b}[2m[{}]\u{1b}[0m {} {}", $prefix, $head, format_args!($($arg)*));
    };
}

#[cfg(test)]
macro_rules! precustomln {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        format!("\u{1b}[2m[{}]\u{1b}[0m {} {}\n", $prefix, $head, format_args!($($arg)*))
    };
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
    ($($arg:tt)*) => { custom!("\u{1b}[1;94mℹ\u{1b}[0m", $($arg)*); }
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
    ($($arg:tt)*) => { customln!("\u{1b}[1;94mℹ\u{1b}[0m", $($arg)*); }
}

/// Prints log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// preinfo!("working hard", "long time no see ..."); // [working hard] ℹ long time no see ...
/// preinfo!("eta: 1 hour", "waiting file `{}` to download ... ", "tlauncher.exe"); // [eta: 1 hour] ℹ waiting file `tlauncher.exe` to download ...
/// ```
#[macro_export]
macro_rules! preinfo {
    ($prefix:expr, $($arg:tt)*) => { precustom!($prefix, "\u{1b}[1;94mℹ\u{1b}[0m", $($arg)*); }
}

/// Prints log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// use hand::*;
///
/// preinfoln!("Building", "Not completed"); // [Building] ℹ Not completed
/// preinfoln!("Dogs", "Dogs are {}", if true { "good" } else { "bad" }); // [Dogs] ℹ Dogs are good
/// ```
#[macro_export]
macro_rules! preinfoln {
    ($prefix:expr, $($arg:tt)*) => { precustomln!($prefix, "\u{1b}[1;94mℹ\u{1b}[0m", $($arg)*); }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => { custom!("\u{1b}[1;33m⚠️\u{1b}[0m", $($arg)*); }
}

#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => { customln!("\u{1b}[1;33m⚠️\u{1b}[0m", $($arg)*); }
}

#[macro_export]
macro_rules! prewarn {
    ($prefix:expr, $($arg:tt)*) => { precustom!($prefix, "\u{1b}[1;33m⚠️\u{1b}[0m", $($arg)*); }
}

#[macro_export]
macro_rules! prewarnln {
    ($prefix:expr, $($arg:tt)*) => { precustomln!($prefix, "\u{1b}[1;33m⚠️\u{1b}[0m", $($arg)*); }
}

#[cfg(test)]
mod tests {
    use colored::*;
    use crate::*;

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
            preinfo!("preinfo", "some formatting {}", 12.333 as f32),
            "\u{1b}[2m[preinfo]\u{1b}[0m \u{1b}[1;94mℹ\u{1b}[0m some formatting 12.333"
        );
        assert_eq!(
            preinfoln!("preinfoln", "some formatting {}", 123),
            "\u{1b}[2m[preinfoln]\u{1b}[0m \u{1b}[1;94mℹ\u{1b}[0m some formatting 123\n"
        );
    }

    #[test]
    fn warn() {
        assert_eq!(
            warn!("test warn"),
            "\u{1b}[1;33m⚠️\u{1b}[0m test warn"
        );
        assert_eq!(
            warnln!("test warnln"),
            "\u{1b}[1;33m⚠️\u{1b}[0m test warnln\n"
        );
        assert_eq!(
            prewarn!("prewarn", "some formatting {}", 12.333 as f32),
            "\u{1b}[2m[prewarn]\u{1b}[0m \u{1b}[1;33m⚠️\u{1b}[0m some formatting 12.333"
        );
        assert_eq!(
            prewarnln!("prewarnln", "some formatting {}", 123),
            "\u{1b}[2m[prewarnln]\u{1b}[0m \u{1b}[1;33m⚠️\u{1b}[0m some formatting 123\n"
        );
    }
}