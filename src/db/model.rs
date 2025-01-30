#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub id:     i64,
    pub name:   String,
    pub serial: String,
}

#[derive(Debug, Clone)]
pub struct TargetValue {
    pub id:         i64,
    pub module_id:  i64,
    pub identifier: String,
    pub descriptor: Option<String>,
    pub value:      f64,
    pub unit:       Option<String>,
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub id:       i64,
    pub gnd:      u16,
    pub v_plus:   u16,
    pub v_minus:  u16,
    pub dds_out1: u16,
    pub dds_out2: u16,
    pub dds_out3: u16,
    pub adc_in1:  u16,
    pub adc_in2:  u16,
}

impl Matrix {
    pub fn new(
        id:       i64,
        gnd:      u16,
        v_plus:   u16,
        v_minus:  u16,
        dds_out1: u16,
        dds_out2: u16,
        dds_out3: u16,
        adc_in1:  u16,
        adc_in2:  u16,
    ) -> Self {

        Self {
            id,
            gnd,
            v_plus,
            v_minus,
            dds_out1,
            dds_out2,
            dds_out3,
            adc_in1,
            adc_in2,
        }

    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id:         i64,
    pub module_id:  i64,
    pub document:   Vec<u8>,
    pub descriptor: String,
}
