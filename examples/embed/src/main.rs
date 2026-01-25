use ring_lang_rs::*;

fn main() {
    println!("=== Ring Embedding Examples ===\n");

    example_runcode();
    example_runfile();
    example_variables();
}

fn example_runcode() {
    println!("1. Running code from string (ring_state_runcode)");
    println!("   Requires: ring_state_init()\n");

    let state = ring_state_init();

    ring_state_runcode_str(state, r#"? "   Hello from Ring!""#);
    ring_state_runcode_str(state, r#"x = 10 + 20"#);
    ring_state_runcode_str(state, r#"? "   x = " + x"#);

    ring_state_delete(state);
    println!();
}

fn example_runfile() {
    println!("2. Running code from file (ring_state_runfile)");
    println!("   Requires: ring_state_new()\n");

    let state = ring_state_new();
    ring_state_runfile_str(state, "script.ring");
    ring_state_delete(state);
    println!();
}

fn example_variables() {
    println!("3. Sharing variables between Rust and Ring\n");

    let state = ring_state_init();

    // Rust -> Ring: Set variable via runcode
    let rust_value = 42.5;
    ring_state_runcode_str(state, &format!("rust_number = {}", rust_value));
    ring_state_runcode_str(state, r#"? "   rust_number = " + rust_number"#);

    // Ring computes something
    ring_state_runcode_str(state, r#"ring_result = rust_number * 2"#);

    // Ring -> Rust: Read variable back
    let var = ring_state_findvar_str(state, "ring_result");
    if !var.is_null() {
        let value = ring_list_getdouble(var, RING_VAR_VALUE);
        println!("   ring_result (read in Rust) = {}", value);
    }

    ring_state_delete(state);
}
