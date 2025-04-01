#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub id:     i64,
    pub name:   String,
    pub serial: String,
}

// Rustc complains about unused fields, even if they are used by
// SQLx's macro system, hence allowing dead code
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TargetValue {
    pub id:         i64,
    pub module_id:  i64,
    pub identifier: String,
    pub descriptor: Option<String>,
    pub value:      f64,
    pub unit:       Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Matrix {
    pub id:            i64,
    pub module_id:     i64,
    pub gnd:           u16,
    pub v_plus:        u16,
    pub v_minus:       u16,
    pub dds_out_plus:  u16,
    pub dds_out_minus: u16,
    pub v3_3:          u16,
    pub adc_in1:       u16,
    pub adc_in2:       u16,
}

impl Matrix {
    pub fn new(
        id:            i64,
        module_id:     i64,
        gnd:           u16,
        v_plus:        u16,
        v_minus:       u16,
        dds_out_plus:  u16,
        dds_out_minus: u16,
        v3_3:          u16,
        adc_in1:       u16,
        adc_in2:       u16,
    ) -> Self {
        Self {
            id,
            module_id,
            gnd,
            v_plus,
            v_minus,
            dds_out_plus,
            dds_out_minus,
            v3_3,
            adc_in1,
            adc_in2
        }
    }
}

pub type Blob = Vec<u8>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Document {
    pub id:         i64,
    pub module_id:  i64,
    pub document:   Blob,
    pub descriptor: String,
}
