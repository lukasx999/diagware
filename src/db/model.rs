#[derive(Debug, Clone)]
pub struct Module {
    pub id:     Option<i64>,
    pub name:   String,
    pub serial: String,
}

impl Module {
    pub fn new(
        id: Option<i64>,
        name: &str,
        serial: &str
    ) -> Self {

        Self {
            id,
            name: name.to_owned(),
            serial: serial.to_owned()
        }

    }
}


#[derive(Debug, Clone)]
pub struct TargetValue {
    pub id:         Option<i64>,
    pub module_id:  Option<i64>,
    pub identifier: String,
    pub descriptor: Option<String>,
    pub value:      f64,
    pub unit:       Option<String>,
}

impl TargetValue {
    pub fn new(
        id:         Option<i64>,
        module_id:  Option<i64>,
        identifier: String,
        descriptor: Option<String>,
        value:      f64,
        unit:       Option<String>
    ) -> Self {

        Self {
            id,
            module_id,
            identifier,
            descriptor,
            value,
            unit,
        }

    }
}
