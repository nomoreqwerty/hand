
/// Prints log message to stderr with a custom head without new line.
///
/// # Examples
///
/// ```
/// custom!("⚠️".yellow().bold(), "error message"); // ⚠️ error message
/// custom!("🔍".bold(), "searching for something {} ... ", 11); // 🔍 searching for something 11
///
/// ```
macro_rules! custom {
    ($head:expr, $($arg:tt)*) => {
        eprint!("{} {}", $head, format_args!($($arg)*));
    };
}

/// Prints log message to stderr with a custom head with new line.
///
/// # Examples
///
/// ```
/// customln!("#".bright_yellow().bold(), "Task completed"); // # Task completed
/// customln!("@", "The dog is seeking for you ... {}", "too long"); // @ The dog is seeking for you ... too long
/// ```
macro_rules! customln {
    ($head:expr, $($arg:tt)*) => {
        eprintln!("{} {}", $head, format_args!($($arg)*));
    };
}

/// Prints log message to stderr with a custom prefix and head without new line.
///
/// # Examples
/// ```
/// precustom!("Fetching", "🌐" "fetching data from {} ... ", "www.example.com"); // [Fetching] 🌐 fetching data from www.example.com ...
/// precustom!("Scanning", "🚨".bright_red().bold(), "{} viruses detected ... ", 3); // [Scanning] 🚨 3 viruses detected ...
/// ```
macro_rules! precustom {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        eprint!("[{}] {} {}", $prefix.dimmed(), $head, format_args!($($arg)*));
    };
}

/// Prints log message to stderr with a custom prefix and head with new line.
///
/// # Examples
///
/// ```
/// precustomln!("Phone", "🔋", "charging is done"); // [Phone] 🔋 charging is done
/// precustomln!("www.download.com", "💾", "file saved to {}", "./downloads"); // [www.download.com] 💾 file saved to ./downloads
/// ```
macro_rules! precustomln {
    ($prefix:expr, $head:expr, $($arg:tt)*) => {
        eprintln!("[{}] {} {}", $prefix.dimmed(), $head, format_args!($($arg)*));
    };
}

/// Prints info log message to stderr without new line.
///
/// # Examples
/// ```
/// info!("testing info ..."); // ℹ testing info ...
/// info!("testing info for {} users", 4); // ℹ testing info for 4 users
/// ```
macro_rules! info {
    ($($arg:tt)*) => { custom!("ℹ".bright_blue().bold(), $($arg)*); }
}

/// Prints info log message to stderr with a new line.
///
/// # Examples
/// ```
/// infoln!("Waiting all jobs to complete"); // ℹ Waiting all jobs to complete
/// infoln!("Next checking will be at {}", "10:00 AM"); // ℹ Next checking will be at 10:00 AM
/// ```
macro_rules! infoln {
    ($($arg:tt)*) => { customln!("ℹ".bright_blue().bold(), $($arg)*); }
}

/// Prints log message to stderr without a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// preinfo!("working hard", "long time no see ..."); // [working hard] ℹ long time no see ...
/// preinfo!("eta: 1 hour", "waiting file `{}` to download ... ", "tlauncher.exe"); // [eta: 1 hour] ℹ waiting file `tlauncher.exe` to download ...
/// ```
macro_rules! preinfo {
    ($prefix:expr, $($arg:tt)*) => {
        precustom!($prefix.dimmed(), "ℹ".bright_blue().bold(), $($arg)*);
    }
}

/// Prints log message to stderr with a new line, with a specified prefix.
///
/// # Examples
///
/// ```
/// preinfoln!("Building", "Not completed"); // [Building] ℹ Not completed
/// preinfoln!("Dogs", "Dogs are {}", if true { "good" } else { "bad" }); // [Dogs] ℹ Dogs are good
/// ```
macro_rules! preinfoln {
    ($prefix:expr, $($arg:tt)*) => {
        precustomln!($prefix, "ℹ".bright_blue().bold(), $($arg)*);
    }
}

#[cfg(test)]
mod tests {
    use colored::*;

    #[test]
    fn info_infoln() {
        info!("test info");
        infoln!("test infoln");
        preinfo!("preinfo", "some formatting {}", 12.333 as f32);
        preinfoln!("preinfoln", "some formatting {}", 123);
    }
}