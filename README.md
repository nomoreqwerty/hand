# 🗃️ A simple log library, similar to <a href="https://github.com/jesusprubio/leg">leg</a>, but with macros

Thanks to <a href="github.com/jesusprubio">jesusprubio</a> for the idea

- Prints to ```stderr```
- NO_COLOR is **not** supported (you can change it)

## Example

```rust
use hand::*;

fn main() {
    // Basic usage
    infoln!("This is an info message");
    warnln!("This is a warning message");
    errorln!("This is an error message");

    // Without new line
    info!("Scanning dogs ... ");
    successln!("complete");

    // With scope
    scopeinfoln!("scope", "Some message with a scope");

    // Support formatting!
    let error_msg = "file not found";
    scopeerrorln!("critical", "Critical error: {}", error_msg);

    infoln!("Continuing in {} seconds", 3);
    waitln!("Cooldown {} seconds", std::time::Duration::from_millis(3000).as_secs());
}

```

### Terminal output
<img src="https://github.com/nomoreqwerty/hand/assets/72273722/7c86a4c6-44e2-49c8-a47f-4454b5b4d121" alt="usage example">

