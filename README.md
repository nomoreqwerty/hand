# 🗃️ A simple log library, similar to <a href="github.com/jesusprubio/leg">leg</a>, but with macros

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
}
```
<img src="https://github.com/nomoreqwerty/hand/assets/72273722/ad717c13-0de2-4547-a5e1-b4fcf6cebd5f" alt="usage example" border-radius="3px">

