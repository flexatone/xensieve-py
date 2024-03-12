use pyo3::prelude::*;
use ::xensieve::Sieve as SieveRS;


#[pyclass(frozen)]
struct Sieve {
    pub(crate) s: SieveRS,
}

#[pymethods]
impl Sieve {
    #[new]
    fn new(expr: String) -> Self {
        Self{s: SieveRS::new(&expr)}
    }

    fn __repr__(&self) -> String {
        self.s.to_string()
    }

    fn __contains__(&self, v: i64) -> bool {
        self.s.contains(v as i128)
    }

    // fn __neg__(&self) -> Self {
    //     let new: SieveRS = !self.s;
    //     Self{s: new}
    // }

    // fn __xor__(&self, other: &Self) -> Self {
    //     let new: SieveRS = self.s ^ other.s;
    //     Self{s: new}
    // }

    // fn __or__(&self, other: &Self) -> Self {
    //     let new: SieveRS = self.s | other.s;
    //     Self{s: new}
    // }

    // NOTE: this works but can be implemented on xensieve_rs
    // fn __and__(&self, other: &Self) -> Self {
    //     let new: SieveRS = self.s.clone() & other.s.clone();
    //     Self{s: new}
    // }

}

/// A Python module implemented in Rust.
#[pymodule]
fn xensieve(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Sieve>()?;
    Ok(())
}
