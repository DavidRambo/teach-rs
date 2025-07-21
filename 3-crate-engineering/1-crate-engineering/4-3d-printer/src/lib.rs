use std::marker::PhantomData;

use rand::Rng;

pub struct Printer3D<S> {
    _marker: PhantomData<S>,
}

/* States */

/// The 3D printer encountered an error and needs resetting
pub enum ErrorState {}
/// The 3D printer is waiting for a job
pub enum IdleState {}
/// The 3D printer is currently printing
pub enum PrintingState {}
/// The 3D printed product is ready
pub enum ProductReadyState {}

/// Check if we're out of filament
fn out_of_filament() -> bool {
    let rand: usize = rand::thread_rng().gen_range(0..100);
    rand > 95
}

impl<S> Printer3D<S> {
    /// Generic Typestate changing method. The calling method informs the generic type T.
    fn change_state<T>(self) -> Printer3D<T> {
        Printer3D {
            _marker: PhantomData,
        }
    }
}

impl Default for Printer3D<IdleState> {
    fn default() -> Self {
        Self::new()
    }
}

impl Printer3D<IdleState> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn start(self) -> Printer3D<PrintingState> {
        // The method signature informs the generic type T in the change_state() method.
        self.change_state()
    }
}

impl Printer3D<PrintingState> {
    pub fn print(self) -> Result<Printer3D<ProductReadyState>, Printer3D<ErrorState>> {
        if out_of_filament() {
            Err(self.change_state())
        } else {
            Ok(self.change_state())
        }
    }
}

impl Printer3D<ProductReadyState> {
    pub fn retrieve_product(self) -> Printer3D<IdleState> {
        self.change_state()
    }
}

impl Printer3D<ErrorState> {
    pub fn reset(self) -> Printer3D<IdleState> {
        self.change_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let printer = Printer3D::new(); // Idle
        let printer = printer.start(); // Printing
        let printer = match printer.print() {
            Err(p) => p.reset(),
            Ok(p) => p.retrieve_product(),
        };
        let _ = printer.start();
    }
}
