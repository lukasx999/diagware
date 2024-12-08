use std::{
    thread,
    time::Duration,
    sync::mpsc,
};

use crate::{
    EEPROM,
    DB,
};



/* CONFIG */

// NOTE: Array, so that rendering states as a list is possible
pub const DIAGNOSIS_STATE_REPRS: [&str; STATE_COUNT+1] = [
    "Leerlauf",
    "Auslesen Seriennummer (via EEPROM)",
    "DB Lookup",
    "Selbsttest",
    "Messung",
    "Auswertung",
    "End"
];


pub const STATE_COUNT: usize = 6; // needed for rendering state machine

// NOTE: using numeric constants, because it makes rendering and incrementing state easier

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DiagnosisState {
    #[default] Idle = 0, // Start
    ReadSerial      = 1,
    DBLookup        = 2,
    SelfTest        = 3,
    Measurements    = 4,
    Evaluation      = 5,
    End             = 6, // NOTE: not included in `STATE_COUNT` (implementationdetail)
}


impl DiagnosisState {
    pub fn repr(&self) -> &'static str {
        DIAGNOSIS_STATE_REPRS[self.clone() as usize]
    }
}



// TODO: Error state
/*
In case of error: switch to error state
show popup if user wants to restart diagnosis or continue
OR:
In case of error: Return diagnosis result error and show error information as popup
*/



/*

// TODO: this
#[derive(Debug)]
pub enum DiagnosisError {
}

impl std::fmt::Display for DiagnosisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}

impl std::error::Error for DiagnosisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

*/



#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DiagnosisMode {
    Manual,
    Automatic,
}





#[derive(Debug)]
pub struct Diagnosis {
    pub state:  DiagnosisState,
    pub mode:   DiagnosisMode,

    pub eeprom: EEPROM,
    pub db:     DB,
}

impl Diagnosis {

    pub fn new(eeprom: EEPROM, db: DB) -> Self {
        Self {
            state: DiagnosisState::default(),
            mode:  DiagnosisMode::Automatic,
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

        println!("{}", self.state.repr());
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

    pub fn next(&mut self) {
        todo!();
    }

    // TODO: step manually through measurements via next() + selector in ui
    // + access fields of diagnosis like db, eeprom in ui
    // + return diagnosiserror in case of error -> show error popup

    pub fn diagnosis(
        &mut self,
        sender: mpsc::Sender<DiagnosisState>
    ) -> Result<(), Box<dyn std::error::Error>> {

        // NOTE: `sender` for informing UI about the change of state

        let mut serial = String::from("");

        loop {

            use DiagnosisState as State;

            match self.state {

                State::Idle => {
                    self.send_state(&sender);
                    self.next_state();
                }

                State::ReadSerial => {
                    self.send_state(&sender);
                    self.read_serial()?;
                    self.next_state();
                }

                State::DBLookup => {
                    self.send_state(&sender);
                    Self::do_stuff();
                    self.next_state();
                }

                State::SelfTest => {
                    self.send_state(&sender);
                    Self::do_stuff();
                    self.next_state();
                }

                State::Measurements => {
                    self.send_state(&sender);
                    Self::do_stuff();
                    self.next_state();
                }

                State::Evaluation => {
                    self.send_state(&sender);
                    Self::do_stuff();
                    self.next_state();
                }

                State::End => {
                    self.next_state();
                    self.send_state(&sender);
                    break Ok(());
                }

            }
        }


    }

}
