use std::thread;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use crate::EEPROM;
use crate::{DB, Module};
use crate::AnyError;


pub const STATE_COUNT: usize = 6; // needed for rendering state machine

// TODO: Error state

// NOTE: using numeric constants, because it makes rendering and incrementing state easier

#[derive(Debug, Clone, Default, PartialEq)]
pub enum DiagnosisState {
    #[default] Idle = 0, // Start
    ReadSerial      = 1,
    DBLookup        = 2,
    Measurements    = 3,
    Evaluation      = 4,
    End             = 5,
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
            state: DiagnosisState::default(),
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
        use DiagnosisState as DS;
        self.state = match self.state {
            DS::Idle        => DS::ReadSerial,
            DS::ReadSerial   => DS::DBLookup,
            DS::DBLookup     => DS::Measurements,
            DS::Measurements => DS::Evaluation,
            DS::Evaluation   => DS::End,
            DS::End          => DS::Idle,
        }
    }

    fn next(mutex: &Mutex<Self>) {
        let mut s = mutex.lock().unwrap();
        println!("{:?}", s.state);
        s.next_state();
    }

    fn do_stuff() {
        thread::sleep(Duration::from_millis(500));
    }

    pub fn is_running(&self) -> bool {
        self.state != DiagnosisState::Idle
    }

    // TODO: switch to method syntax
    pub fn diagnosis(mutex: &Mutex<Self>) -> AnyError<()> {

        loop {

            let state: DiagnosisState = mutex.lock().unwrap().state.clone();

            // TODO: implement first 2 states
            match state {
                DiagnosisState::Idle => {
                    Self::next(mutex);
                }
                DiagnosisState::ReadSerial => {
                    Self::do_stuff();
                    Self::next(mutex);
                }
                DiagnosisState::DBLookup => {
                    Self::do_stuff();
                    Self::next(mutex);
                }
                DiagnosisState::Measurements => {
                    Self::do_stuff();
                    Self::next(mutex);
                }
                DiagnosisState::Evaluation => {
                    Self::do_stuff();
                    Self::next(mutex);
                }
                DiagnosisState::End => {
                    Self::do_stuff();
                    Self::next(mutex);
                    break Ok(());
                }
            }
        }


    }

}
