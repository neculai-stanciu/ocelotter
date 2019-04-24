use std::fmt;

use crate::constant_pool::CpAttr;

#[derive(Debug, Clone)]
pub struct OtField {
    klass_name: String,
    flags: u16,
    name_idx: u16,
    desc_idx: u16,
    name: String,
    desc: String,
    attrs: Vec<CpAttr>,
}

impl OtField {
    pub fn of(
        klass_name: String,
        field_name: String,
        field_desc: String,
        field_flags: u16,
        name: u16,
        desc: u16,
    ) -> OtField {
        OtField {
            klass_name: klass_name.to_string(),
            // FIXME
            flags: field_flags,
            name_idx: name,
            desc_idx: desc,
            name: field_name,
            desc: field_desc,
            attrs: Vec::new(),
        }
    }

    pub fn set_attr(&self, _index: u16, _attr: CpAttr) -> () {}

    pub fn get_name(&self) -> String {
        String::from("")
    }

    pub fn get_klass_name(&self) -> String {
        self.klass_name.clone()
    }

    pub fn get_fq_name_desc(&self) -> String {
        self.klass_name.clone() + "." + &self.name + ":" + &self.desc
    }
}

impl fmt::Display for OtField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}:{}", self.klass_name, self.name, self.desc_idx)
    }
}