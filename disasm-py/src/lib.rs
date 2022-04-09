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

#[allow(non_snake_case)]
#[pymethods]
impl Ins {
    #[getter]
    fn simm(&self) -> i64 {
        self.0.field_simm() as i64
    }
    #[getter]
    fn uimm(&self) -> i64 {
        self.0.field_uimm() as i64
    }
    #[getter]
    fn offset(&self) -> i64 {
        self.0.field_offset() as i64
    }
    #[getter]
    fn ps_offset(&self) -> i64 {
        self.0.field_ps_offset() as i64
    }
    #[getter]
    fn BO(&self) -> i64 {
        self.0.field_BO() as i64
    }
    #[getter]
    fn BI(&self) -> i64 {
        self.0.field_BI() as i64
    }
    #[getter]
    fn BD(&self) -> i64 {
        self.0.field_BD() as i64
    }
    #[getter]
    fn LI(&self) -> i64 {
        self.0.field_LI() as i64
    }
    #[getter]
    fn SH(&self) -> i64 {
        self.0.field_SH() as i64
    }
    #[getter]
    fn MB(&self) -> i64 {
        self.0.field_SH() as i64
    }
    #[getter]
    fn ME(&self) -> i64 {
        self.0.field_SH() as i64
    }
    #[getter]
    fn rS(&self) -> i64 {
        self.0.field_rS() as i64
    }
    #[getter]
    fn rD(&self) -> i64 {
        self.0.field_rD() as i64
    }
    #[getter]
    fn rA(&self) -> i64 {
        self.0.field_rA() as i64
    }
    #[getter]
    fn rB(&self) -> i64 {
        self.0.field_rB() as i64
    }
    #[getter]
    fn rC(&self) -> i64 {
        self.0.field_rC() as i64
    }
    #[getter]
    fn sr(&self) -> i64 {
        self.0.field_sr() as i64
    }
    #[getter]
    fn spr(&self) -> i64 {
        self.0.field_spr() as i64
    }
    #[getter]
    fn frS(&self) -> i64 {
        self.0.field_frS() as i64
    }
    #[getter]
    fn frD(&self) -> i64 {
        self.0.field_frD() as i64
    }
    #[getter]
    fn frA(&self) -> i64 {
        self.0.field_frA() as i64
    }
    #[getter]
    fn frB(&self) -> i64 {
        self.0.field_frB() as i64
    }
    #[getter]
    fn frC(&self) -> i64 {
        self.0.field_frC() as i64
    }
    #[getter]
    fn crbD(&self) -> i64 {
        self.0.field_crbD() as i64
    }
    #[getter]
    fn crbA(&self) -> i64 {
        self.0.field_crbA() as i64
    }
    #[getter]
    fn crbB(&self) -> i64 {
        self.0.field_crbB() as i64
    }
    #[getter]
    fn crfD(&self) -> i64 {
        self.0.field_crfD() as i64
    }
    #[getter]
    fn crfS(&self) -> i64 {
        self.0.field_crfS() as i64
    }
    #[getter]
    fn crm(&self) -> i64 {
        self.0.field_crm() as i64
    }
    #[getter]
    fn ps_l(&self) -> i64 {
        self.0.field_ps_l() as i64
    }
    #[getter]
    fn ps_W(&self) -> i64 {
        self.0.field_ps_W() as i64
    }
    #[getter]
    fn ps_NB(&self) -> i64 {
        self.0.field_NB() as i64
    }
    #[getter]
    fn tbr(&self) -> i64 {
        self.0.field_tbr() as i64
    }
    #[getter]
    fn mtfsf_FM(&self) -> i64 {
        self.0.field_mtfsf_FM() as i64
    }
    #[getter]
    fn mtfsf_IMM(&self) -> i64 {
        self.0.field_mtfsf_IMM() as i64
    }
    #[getter]
    fn TO(&self) -> i64 {
        self.0.field_TO() as i64
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
