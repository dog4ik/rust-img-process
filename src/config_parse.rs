pub struct Config {
    pub first_path: String,
    pub second_path: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let first_path = args[1].clone();
        let second_path = args[2].clone();

        Ok({
            Config {
                first_path,
                second_path,
            }
        })
    }
}
