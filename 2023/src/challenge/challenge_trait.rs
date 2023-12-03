pub trait Challenge {
    fn challenge() -> String;
    fn run(input: String) -> String;
}