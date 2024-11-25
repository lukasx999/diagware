use crate::EEPROM;
use crate::{DB, Module};
use crate::AnyError;



#[derive(Debug)]
pub enum DiagnosisState {
    Start,
    ReadSerial,
    DBLookup,
    Measurements,
    Evaluation,
    End,
}

#[derive(Debug)]
pub struct Diagnosis {
    state: DiagnosisState,
    // eeprom: EEPROM,
    // db: DB,
}

impl Diagnosis {

    pub fn new() -> Self {
        Self {
            state: DiagnosisState::Start,
            // eeprom,
            // db,
        }
    }

    /*
    fn read_serial(&mut self) -> AnyError<String> {
        let serial: String = self.eeprom.get_serial()?;
        Ok(serial)
    }

    fn db_lookup(&self, serial: &str) -> AnyError<Module> {
        let module: Module = self.db.get_module_by_serial(serial)?;
        Ok(module)
    }
    */


    fn next_state(&mut self) {
        match self.state {
            DiagnosisState::Start        => {
                self.state = DiagnosisState::ReadSerial;
            }
            DiagnosisState::ReadSerial   => {
                self.state = DiagnosisState::DBLookup;
            }
            DiagnosisState::DBLookup     => {
                self.state = DiagnosisState::Measurements;
            }
            DiagnosisState::Measurements => {
                self.state = DiagnosisState::Evaluation;
            }
            DiagnosisState::Evaluation   => {
                self.state = DiagnosisState::End;
            }
            DiagnosisState::End => {}
        }
    }


    pub fn diagnosis(&mut self) -> AnyError<()> {

        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            match self.state {
                DiagnosisState::Start        => {
                    println!("Start!");
                    self.next_state();
                }
                DiagnosisState::ReadSerial   => {
                    println!("Reading Serial...");
                    self.next_state();
                }
                DiagnosisState::DBLookup     => {
                    println!("Looking up Data...");
                    self.next_state();
                }
                DiagnosisState::Measurements => {
                    println!("Taking Measurements...");
                    self.next_state();
                }
                DiagnosisState::Evaluation   => {
                    println!("Evaluating...");
                    self.next_state();
                }
                DiagnosisState::End => {
                    println!("End!");
                    break;
                }
            }
        }

        Ok(())

    }

}
