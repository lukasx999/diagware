use std::{
    thread,
    time::Duration,
    sync::{Arc, Mutex, MutexGuard, mpsc},
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
    End             = 6,
}


pub const STATE_LABELS: [&str; 7] = [
    "Leerlauf",
    "Auslesen Seriennummer (via EEPROM)",
    "DB Lookup",
    "Selbsttest",
    "Messung",
    "Auswertung",
    "End",
];




#[derive(Debug)]
pub enum DiagnosisError {
}

// impl std::error::Error for DiagnosisError {
// }




#[derive(Debug)]
pub struct Diagnosis {
    pub state:  DiagnosisState,
    pub eeprom: EEPROM,
    pub db:     DB,
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
            DS::Evaluation   => DS::End,
            DS::End          => DS::Idle,
        }

    }

    fn send_state(&self, sender: &mpsc::Sender<DiagnosisState>) {
        sender.send(self.state.clone()).unwrap();
    }

    fn do_stuff() {
        thread::sleep(Duration::from_millis(500));
    }

    pub fn is_running(&self) -> bool {
        self.state != DiagnosisState::Idle
    }

    pub fn next(&mut self) {
        todo!();
    }

    // TODO: step manually through measurements via next() + selector in ui
    // + access fields of diagnosis like db, eeprom in ui
    pub fn diagnosis(
        &mut self,
        sender: mpsc::Sender<DiagnosisState>
    ) -> Result<(), Box<dyn std::error::Error>> {

        // NOTE: `sender` for informing UI about the change of state

        let mut serial = String::from("");

        loop {

            match self.state {

                DiagnosisState::Idle => {
                    self.send_state(&sender);
                    self.next_state();
                }

                DiagnosisState::ReadSerial => {
                    self.send_state(&sender);
                    self.read_serial()?;
                    self.next_state();
                }

                DiagnosisState::DBLookup => {
                    self.send_state(&sender);
                    Self::do_stuff();
                    self.next_state();
                }

                DiagnosisState::SelfTest => {
                    self.send_state(&sender);
                    Self::do_stuff();
                    self.next_state();
                }

                DiagnosisState::Measurements => {
                    self.send_state(&sender);
                    Self::do_stuff();
                    self.next_state();
                }

                DiagnosisState::Evaluation => {
                    self.send_state(&sender);
                    Self::do_stuff();
                    self.next_state();
                }

                DiagnosisState::End => {
                    self.next_state();
                    self.send_state(&sender);
                    break Ok(());
                }

            }
        }


    }

}
