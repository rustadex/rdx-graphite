use stderr::Stderr;
use rdx::quants::*;         
use rdx::quant_error::QuantError;

fn main() {
    let mut logger = Stderr::new();

    match do_experiment() {
        Ok(_) => info("Experiment succeeded"),
        Err(e) => error(&format!("Experiment failed: {}", e)),
    }
}

fn do_experiment() -> Result<(), QuantError> {
    // Instantiate or mutate your quant system here
    // E.g., let q = Quant::new();
    Ok(())
}
