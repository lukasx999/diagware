use std::{
    thread,
    time::Duration,
    sync::{Arc, Mutex, MutexGuard},
    rc::Rc,
};

use crate::{
    EEPROM,
    DB,
};


pub const STATE_COUNT: usize = 6; // needed for rendering state machine

// TODO: Error state

// NOTE: using numeric constants, because it makes rendering and incrementing state easier

#[derive(Debug, Clone, Default, PartialEq)]
pub enum DiagnosisState {
    #[default] Idle = 0, // Start
    ReadSerial      = 1,
    DBLookup        = 2,
    SelfTest        = 3,
    Measurements    = 4,
    Evaluation      = 5,
}


pub const STATE_LABELS: [&str; 6] = [
    "Leerlauf",
    "Auslesen Seriennummer (via EEPROM)",
    "DB Lookup",
    "Selbsttest",
    "Messung",
    "Auswertung",
];



pub struct DiagnosisError;



#[derive(Debug)]
pub struct Diagnosis {
    pub state: DiagnosisState,
    pub eeprom:    EEPROM,
    pub db:        DB,
}

impl Diagnosis {

    pub fn new(eeprom: EEPROM, db: DB) -> Self {
        Self {
            state: DiagnosisState::default(),
            eeprom,
            db,
        }
    }

    // TODO: Diag error struct

    fn read_serial(&self) -> Result<String, Box<dyn std::error::Error>> {
        Self::do_stuff();
        Ok(self.eeprom.get_serial()?)
    }

    // fn db_lookup(&self, serial: &str) -> Result<Module, Box<dyn std::error::Error>> {
    //     Self::do_stuff();
    //     let module: Module = self.db.get_module_by_serial(serial)?;
    //     Ok(module)
    // }

    fn next_state(&mut self) {
        use DiagnosisState as DS;

        println!("{}", STATE_LABELS[self.state.clone() as usize]);
        self.state = match self.state {
            DS::Idle         => DS::ReadSerial,
            DS::ReadSerial   => DS::DBLookup,
            DS::DBLookup     => DS::SelfTest,
            DS::SelfTest     => DS::Measurements,
            DS::Measurements => DS::Evaluation,
            DS::Evaluation   => DS::Idle,
        }

    }

    fn do_stuff() {
        thread::sleep(Duration::from_millis(500));
    }

    pub fn is_running(&self) -> bool {
        self.state != DiagnosisState::Idle
    }

    // TODO: switch to method syntax
    // instead of spawning a new thread for the whole loop,
    // spawn a new thread for each task (thread spawning overhead?)

    // TODO: manual step through measurements
    // next() method: executes current state in new thread
    pub fn diagnosis(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let mut serial = String::from("");

        loop {

            match self.state {

                DiagnosisState::Idle => {
                    self.next_state();
                }

                DiagnosisState::ReadSerial => {
                    self.read_serial()?;
                    self.next_state();
                }

                DiagnosisState::DBLookup => {
                    self.next_state();
                }

                DiagnosisState::SelfTest => {
                    Self::do_stuff();
                    self.next_state();
                }

                DiagnosisState::Measurements => {
                    Self::do_stuff();
                    self.next_state();
                }

                DiagnosisState::Evaluation => {
                    Self::do_stuff();
                    self.next_state();
                    break Ok(());
                }

            }
        }


    }

}
