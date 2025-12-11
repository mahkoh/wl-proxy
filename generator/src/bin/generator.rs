use {
    error_reporter::Report,
    generator::{generate, generate::GeneratorError},
};

fn main() -> Result<(), Report<GeneratorError>> {
    generate::main().map_err(Report::new)
}
