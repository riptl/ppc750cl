use pyo3::prelude::*;
use pyo3::{PyIterProtocol, PyObjectProtocol};

#[pyclass]
struct Ins(ppc750cl::Ins);

macro_rules! ins_ufield {
    ($name:ident) => {
        #[pymethods]
        impl Ins {
            #[getter]
            fn $name(&self) -> PyResult<u32> {
                Ok(self.0.$name() as u32)
            }
        }
    };
}

macro_rules! ins_ifield {
    ($name:ident) => {
        #[pymethods]
        impl Ins {
            #[getter]
            fn $name(&self) -> PyResult<i32> {
                Ok(self.0.$name() as i32)
            }
        }
    };
}

#[pymethods]
impl Ins {
    #[new]
    fn new(code: u32, addr: u32) -> Self {
        Ins(ppc750cl::Ins::new(code, addr))
    }

    #[getter]
    fn code(&self) -> PyResult<u32> {
        Ok(self.0.code)
    }

    #[getter]
    fn addr(&self) -> PyResult<u32> {
        Ok(self.0.addr)
    }
}

ins_ufield!(rc);
ins_ufield!(aa);
ins_ufield!(lk);
ins_ufield!(l);
ins_ufield!(oe);
ins_ufield!(w);
ins_ufield!(s);
ins_ufield!(d);
ins_ufield!(a);
ins_ufield!(b);
ins_ufield!(c);
ins_ufield!(crb_d);
ins_ufield!(crb_a);
ins_ufield!(crb_b);
ins_ufield!(crm);
ins_ufield!(sr);
ins_ufield!(spr);
ins_ufield!(fm);
ins_ufield!(crf_d);
ins_ufield!(crf_s);
ins_ifield!(simm);
ins_ufield!(uimm);
ins_ufield!(bo);
ins_ufield!(bi);
ins_ufield!(sh);
ins_ufield!(mb);
ins_ufield!(me);
ins_ufield!(me_31sub);
ins_ifield!(bd);
ins_ifield!(li);
ins_ufield!(to);
ins_ufield!(ps_l);
ins_ifield!(ps_d);

impl From<ppc750cl::Ins> for Ins {
    fn from(ins: ppc750cl::Ins) -> Self {
        Self(ins)
    }
}

#[pyproto]
impl<'a> PyObjectProtocol<'a> for Ins {
    fn __str__(&self) -> String {
        self.0.to_string()
    }
}

#[pyclass]
struct DisasmIterator {
    bytes: Vec<u8>,
    addr: u32,
    offset: u32,
}

#[pyproto]
impl PyIterProtocol for DisasmIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<DisasmIterator> {
        slf
    }
    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<Ins>> {
        if (slf.bytes.len() as u32) - slf.offset < 4 {
            return Ok(None);
        }
        let code = ((slf.bytes[(slf.offset) as usize] as u32) << 24)
            | ((slf.bytes[(slf.offset + 1) as usize] as u32) << 16)
            | ((slf.bytes[(slf.offset + 2) as usize] as u32) << 8)
            | (slf.bytes[(slf.offset + 3) as usize] as u32);
        slf.offset += 4;
        let ins = Ins::new(code, slf.addr);
        slf.addr += 4;
        Ok(Some(ins))
    }
}

#[pyfunction]
fn disasm_iter(code: &[u8], addr: u32) -> PyResult<DisasmIterator> {
    Ok(DisasmIterator {
        bytes: code.to_vec(),
        addr,
        offset: 0,
    })
}

#[pymodule]
fn ppc750cl(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Ins>()?;
    m.add_wrapped(wrap_pyfunction!(disasm_iter))?;
    Ok(())
}
