use std::thread;
use std::time::Duration;

use crate::EEPROM;
use crate::{DB, Module};
use crate::AnyError;


pub const STATE_COUNT: usize = 6; // needed for rendering state machine

// TODO: switch to enum numbers (=> incrementing in next_state())
#[derive(Debug, Clone, Default)]
pub enum DiagnosisState {
    #[default] Start,
    ReadSerial,
    DBLookup,
    Measurements,
    Evaluation,
    End,
}

#[derive(Debug)]
pub struct Diagnosis {
    pub state: DiagnosisState,
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
        self.state = match self.state {
            DiagnosisState::Start        => DiagnosisState::ReadSerial,
            DiagnosisState::ReadSerial   => DiagnosisState::DBLookup,
            DiagnosisState::DBLookup     => DiagnosisState::Measurements,
            DiagnosisState::Measurements => DiagnosisState::Evaluation,
            DiagnosisState::Evaluation   => DiagnosisState::End,
            DiagnosisState::End          => DiagnosisState::Start,
        }
    }


    fn do_stuff(&self) {
        thread::sleep(Duration::from_millis(500));
    }



    pub fn diagnosis(&mut self) -> AnyError<()> {

        loop {

            match self.state {
                DiagnosisState::Start        => {
                    self.do_stuff();
                    println!("Start!");
                    self.next_state();
                }
                DiagnosisState::ReadSerial   => {
                    self.do_stuff();
                    println!("Reading Serial...");
                    self.next_state();
                }
                DiagnosisState::DBLookup     => {
                    self.do_stuff();
                    println!("Looking up Data...");
                    self.next_state();
                }
                DiagnosisState::Measurements => {
                    self.do_stuff();
                    println!("Taking Measurements...");
                    self.next_state();
                }
                DiagnosisState::Evaluation   => {
                    self.do_stuff();
                    println!("Evaluating...");
                    self.next_state();
                }
                DiagnosisState::End => {
                    self.do_stuff();
                    println!("End!");
                    self.next_state();
                    break;
                }
            }
        }

        Ok(())

    }



}
