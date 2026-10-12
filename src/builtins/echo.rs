pub fn echo(args: &[String]) -> Result<(), String> {
    println!("{}", args.join(" "));
    Ok(())
}
