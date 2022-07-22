use std::env;

fn main() -> Result<(), String> {
    // TODO: Replace with
    //      let query: String = env::args().skip(1).intersperse(" ").collect();
    // when Iterator::intersperse() is stabilized.
    // (Tracking issue: https://github.com/rust-lang/rust/issues/79524)
    let query = {
        let mut r = String::new();

        for arg in env::args().skip(1) {
            if !r.is_empty() {
                r.push(' ');
            }
            r.push_str(&arg);
        }

        r
    };

    // Don't Google with an empty query
    if query.is_empty() {
        println!("{}", ggrks::HELP_MESSAGE);
        return Ok(());
    }

    // ggr (ks), and check for failure
    let ggr_result = ggrks::ggrks(&query);
    if let Err(cause) = ggr_result {
        return Err(format!("{}{}", ggrks::GENERIC_ERROR_MESSAGE, cause));
    }

    // Print extras (if applicable) and exit
    ggrks::maybe_print_extras(&query);
    Ok(())
}
