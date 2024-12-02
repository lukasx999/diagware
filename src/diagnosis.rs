use std::{
    thread,
    time::Duration,
    sync::{Arc, Mutex, MutexGuard},
    rc::Rc,
};

use crate::{
    EEPROM,
    DB,
    Module,
};


pub const STATE_COUNT: usize = 5; // needed for rendering state machine

// TODO: Error state

// NOTE: using numeric constants, because it makes rendering and incrementing state easier

#[derive(Debug, Clone, Default, PartialEq)]
pub enum DiagnosisState {
    #[default] Idle = 0, // Start
    ReadSerial      = 1,
    DBLookup        = 2,
    Measurements    = 3,
    Evaluation      = 4,
}


pub const STATE_LABELS: [&str; 5] = [
    "Leerlauf",
    "Auslesen Seriennummer (via EEPROM)",
    "DB Lookup",
    "Messung",
    "Auswertung",
];



pub struct DiagnosisError;



#[derive(Debug)]
pub struct Diagnosis {
    pub state: DiagnosisState,
    eeprom: Arc<Mutex<EEPROM>>,
    db: Arc<Mutex<DB>>,
}

impl Diagnosis {

    pub fn new(eeprom: Arc<Mutex<EEPROM>>, db: Arc<Mutex<DB>>) -> Self {
        Self {
            state: DiagnosisState::default(),
            eeprom,
            db,
        }
    }

    // TODO: Diag error struct

    fn read_serial(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let eeprom = self.eeprom.lock().unwrap();
        let serial: String = eeprom.get_serial()?;
        Ok(serial)
    }

    fn db_lookup(&self, serial: &str) -> Result<Module, Box<dyn std::error::Error>> {
        let db = self.db.lock().unwrap();
        let module: Module = db.get_module_by_serial(serial)?;
        Ok(module)
    }


    fn next_state(&mut self) {
        use DiagnosisState as DS;
        self.state = match self.state {
            DS::Idle         => DS::ReadSerial,
            DS::ReadSerial   => DS::DBLookup,
            DS::DBLookup     => DS::Measurements,
            DS::Measurements => DS::Evaluation,
            DS::Evaluation   => DS::Idle,
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
    // instead of spawning a new thread for the whole loop,
    // spawn a new thread for each task (thread spawning overhead?)

    // TODO: manual step through measurements
    // next() method: executes current state in new thread
    pub fn diagnosis(mutex: &Mutex<Self>) -> Result<(), Box<dyn std::error::Error>> {

        let mut serial = String::from("");

        loop {

            let state: DiagnosisState = mutex.lock().unwrap().state.clone();

            match state {

                DiagnosisState::Idle => {
                    Self::next(mutex);

                }

                DiagnosisState::ReadSerial => {
                    // NOTE: we're probably doing_stuff() WITH the mutex
                    // TODO: access eeprom without holding mutex for &mut self
                    Self::do_stuff();
                    serial = mutex
                        .lock()
                        .unwrap()
                        .read_serial()?;
                    Self::next(mutex);
                }

                DiagnosisState::DBLookup => {
                    Self::do_stuff();
                    let module: Module = mutex
                        .lock()
                        .unwrap()
                        .db_lookup(serial.as_str())?;
                    dbg!(&module);
                    Self::next(mutex);
                }

                DiagnosisState::Measurements => {
                    Self::do_stuff();
                    Self::next(mutex);
                }

                DiagnosisState::Evaluation => {
                    Self::do_stuff();
                    Self::next(mutex);
                    break Ok(());
                }

            }
        }


    }

}
