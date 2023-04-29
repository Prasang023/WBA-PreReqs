pub fn run() {
    println!("Hello from the print.rs file");

    println!("{} is from {}", "Brad", "Mass");

    // positional arg
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named arg
    println!("{name} likes ro play {activity}", name="John", activity="Bseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Plcaeholder for debug
    println!("{:?}", (12, true, "Hello"));
}