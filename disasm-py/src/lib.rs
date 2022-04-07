use pyo3::prelude::*;
use pyo3::types::PyBytes;

use ppc750cl::formatter::FormattedIns;

#[pyclass]
struct Ins(ppc750cl::Ins);

#[pymethods]
impl Ins {
    #[new]
    fn new(code: u32, addr: u32) -> Self {
        Ins(ppc750cl::Ins::new(code, addr))
    }

    #[getter]
    fn code(&self) -> u32 {
        self.0.code
    }

    #[getter]
    fn addr(&self) -> u32 {
        self.0.addr
    }

    #[getter]
    fn opcode(&self) -> &'static str {
        self.0.op.mnemonic()
    }

    fn __str__(&self) -> String {
        FormattedIns(self.0.clone()).to_string()
    }
}

impl From<ppc750cl::Ins> for Ins {
    fn from(ins: ppc750cl::Ins) -> Self {
        Self(ins)
    }
}

#[pyclass]
struct DisasmIterator {
    bytes: Py<PyBytes>,
    addr: u32,
    offset: u32,
    left: usize,
}

#[pymethods]
impl DisasmIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<DisasmIterator> {
        slf
    }
    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<Ins>> {
        if slf.left < 4 {
            return Ok(None);
        }
        let bytes = slf.bytes.as_ref(slf.py());
        let code = ((bytes[(slf.offset) as usize] as u32) << 24)
            | ((bytes[(slf.offset + 1) as usize] as u32) << 16)
            | ((bytes[(slf.offset + 2) as usize] as u32) << 8)
            | (bytes[(slf.offset + 3) as usize] as u32);
        slf.offset += 4;
        slf.left -= 4;
        let ins = Ins::new(code, slf.addr);
        slf.addr += 4;
        Ok(Some(ins))
    }
}

#[pyfunction(code, addr, offset = "0", size = "None")]
fn disasm_iter(
    code: &PyBytes,
    addr: u32,
    offset: u32,
    size: Option<u32>,
) -> PyResult<DisasmIterator> {
    let left = match size {
        None => code.as_bytes().len().saturating_sub(offset as usize),
        Some(v) => v as usize,
    };
    Ok(DisasmIterator {
        bytes: code.into(),
        addr,
        offset,
        left,
    })
}

#[pymodule]
fn ppc750cl(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Ins>()?;
    m.add_wrapped(wrap_pyfunction!(disasm_iter))?;
    Ok(())
}
