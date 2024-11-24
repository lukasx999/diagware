use crate::EEPROM;
use crate::{DB, Module};
use crate::AnyError;



pub enum DiagnosisState {
    Start,
    ReadSerial,
    DBLookup,
    Measurements,
    CheckValues,
    End,
}

pub struct Diagnosis {
    state: DiagnosisState,
    eeprom: EEPROM,
    db: DB,
}

impl Diagnosis {

    pub fn new(eeprom: EEPROM, db: DB) -> Self {
        Self {
            state: DiagnosisState::Start,
            eeprom,
            db,
        }
    }

    fn read_serial(&mut self) -> AnyError<String> {
        let serial: String = self.eeprom.get_serial()?;
        Ok(serial)
    }

    fn db_lookup(&self, serial: &str) -> AnyError<Module> {
        let module: Module = self.db.get_module_by_serial(serial)?;
        Ok(module)
    }

    pub fn diagnosis(&mut self) -> AnyError<()> {

        match self.state {
            DiagnosisState::Start        => {}
            DiagnosisState::ReadSerial   => {
                self.read_serial()?;
            }
            DiagnosisState::DBLookup     => {
            }
            DiagnosisState::Measurements => {}
            DiagnosisState::CheckValues  => {}
            DiagnosisState::End          => {}
        }

        Ok(())

    }

}
