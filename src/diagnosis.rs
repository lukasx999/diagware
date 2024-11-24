pub enum DiagnosisStates {
    Start,
    ReadSerial,
    DBLookup,
    Measurements,
    CheckValues,
    End,
}


pub struct Diagnosis {
    state: DiagnosisStates,
    // TODO: db, eeprom, io
}

impl Diagnosis {

    pub fn new() -> Self {
        Self { state: DiagnosisStates::Start }
    }

    fn read_serial(&self) {
    }

    pub fn diagnosis(&mut self) {

        match self.state {
            DiagnosisStates::Start        => {}
            DiagnosisStates::ReadSerial   => {}
            DiagnosisStates::DBLookup     => {}
            DiagnosisStates::Measurements => {}
            DiagnosisStates::CheckValues  => {}
            DiagnosisStates::End          => {}
        }

    }

}
