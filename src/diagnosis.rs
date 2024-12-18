
/* Linear FSM */

use std::{
    thread,
    time::Duration,
    sync::mpsc,
};

use crate::{
    eeprom,
    EEPROM,
    DB,
    ShiftRegister,
    db::model::{Module, Matrix, TargetValue},
};



/* CONFIG */

// NOTE: Array, so that rendering states as a list is possible
pub const DIAGNOSIS_STATE_REPRS: [&str; STATE_COUNT+1] = [
    "Leerlauf",
    "Auslesen Seriennummer",
    "DB Lookup",
    "Schaltmatrix",
    "Signalerzeugung",
    "Messung",
    "Auswertung",
    "End"
];




pub const STATE_COUNT: usize = 7; // needed for rendering state machine

// NOTE: using numeric constants, because it makes rendering and incrementing state easier
// TODO: potential self test

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DiagnosisState {
    #[default] Idle = 0, // Start
    ReadSerial      = 1,
    DBLookup        = 2,
    SwitchMatrix    = 3,
    ApplySignals    = 4,
    Measurements    = 5,
    Evaluation      = 6,
    End             = 7, // NOTE: not included in `STATE_COUNT` (implementationdetail)
}



impl std::fmt::Display for DiagnosisState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = DIAGNOSIS_STATE_REPRS[self.clone() as usize];
        write!(f, "{}", repr)
    }
}

impl DiagnosisState {
    pub fn repr(&self) -> &str {
        DIAGNOSIS_STATE_REPRS[self.clone() as usize]
    }
}





#[derive(thiserror::Error, Debug)]
pub enum DiagnosisErrorInternal {
}


#[derive(thiserror::Error, Debug)]
pub enum DiagnosisError {
    #[error("Failed to transmit current state")]
    SendError(#[from] mpsc::SendError<DiagnosisState>),
    #[error("Database operation failed")]
    DatabaseError(#[from] sqlx::Error),
    #[error("EEPROM operation failed")]
    EepromError(#[from] eeprom::EepromError),

    // The current DUT is holding a serial number which was not found within the database
    // UnknownModule {
    //     serial: String,
    // },
}


pub type MeasuredValue = crate::db::model::TargetValue;


// TODO:
/* this holds the results of a successful diagnosis */
#[derive(Debug, Clone)]
pub struct DiagnosisReport(Vec<MeasuredValue>);

impl DiagnosisReport {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

pub type DiagnosisResult = Result<DiagnosisReport, DiagnosisError>;




#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum DiagnosisMode {
    #[default] Automatic,
    Manual,
}







#[derive(Debug)]
pub struct Diagnosis {
    state:        DiagnosisState,
    sender:       mpsc::Sender<DiagnosisState>, // informing the receiver about change of state
    pub mode:     DiagnosisMode,
    pub eeprom:   EEPROM,
    pub db:       DB,
    pub shiftreg: ShiftRegister,

    // Temporary values resulting from computations within the states
    temp_serial: Option<String>,
    temp_module: Option<Module>,
}

impl Diagnosis {

    pub fn new(eeprom: EEPROM, db: DB, shiftreg: ShiftRegister, sender: mpsc::Sender<DiagnosisState>) -> Self {
        Self {
            state: DiagnosisState::default(),
            sender,
            mode: DiagnosisMode::default(),
            eeprom,
            db,
            shiftreg,
            temp_serial: None,
            temp_module: None,
        }
    }

    fn next_state(&mut self) -> Result<(), DiagnosisError> {
        use DiagnosisState as DS;

        self.sender.send(self.state)?;

        println!("current state: {}", self.state.repr());
        self.state = match self.state {
            DS::Idle         => DS::ReadSerial,
            DS::ReadSerial   => DS::DBLookup,
            DS::DBLookup     => DS::SwitchMatrix,
            DS::SwitchMatrix => DS::ApplySignals,
            DS::ApplySignals => DS::Measurements,
            DS::Measurements => DS::Evaluation,
            DS::Evaluation   => DS::End,
            DS::End          => DS::Idle,
        };

        Ok(())

    }

    fn do_stuff() {
        thread::sleep(Duration::from_millis(500));
    }


    // + return diagnosiserror in case of error -> show error popup
    // TODO: diag error should contain information for showing error popup on ui


    /* Executes the current state, and transitions to the next state          */
    /* Returns Ok(None) if state execution was successful, else returns error */
    /* Returns a DiagnosisResult if the last state was executed successfully  */
    pub fn run_state(&mut self) -> Result<Option<DiagnosisReport>, DiagnosisError> {

        use DiagnosisState as S;
        match self.state {

            S::Idle => self.next_state()?,

            S::ReadSerial => {
                Self::do_stuff();
                let serial: String = self.eeprom.get_serial()?;
                self.temp_serial = Some(serial);
                self.next_state()?;
            }

            S::DBLookup => {
                Self::do_stuff();

                let serial = self.temp_serial.as_ref().unwrap();
                let module: Module = self.db.get_module_by_serial(&serial)?;
                dbg!(&module);
                self.temp_module = Some(module);

                self.next_state()?;
            }

            S::SwitchMatrix => {
                Self::do_stuff();

                let id = self.temp_module.as_ref().unwrap().id.unwrap();
                let matrix: Matrix = self.db.get_matrix_by_id(id)?;
                dbg!(&matrix);

                // TODO: shift reg

                self.next_state()?;
            }

            S::ApplySignals => {
                Self::do_stuff();
                self.next_state()?;
            }

            S::Measurements => {
                Self::do_stuff();
                self.next_state()?;
            }

            S::Evaluation => {
                use crate::db::model::TargetValue;

                Self::do_stuff();

                // // TODO: deal with unwrap()
                // // let id = self.temp_module.clone().unwrap().id.unwrap();
                // let id = self.temp_module.as_ref().unwrap().id.unwrap();
                // let targetvalues: Vec<TargetValue> = self.db.get_targetvalue_by_id(id)?;
                // // TODO: compare measured values with targetvalues

                self.next_state()?;
            }

            S::End => {
                Self::do_stuff();
                self.reset()?;
                return Ok(Some(DiagnosisReport::new()));
            }

        }

        Ok(None)

    }

    pub fn reset(&mut self) -> Result<(), DiagnosisError> {
        self.state = DiagnosisState::default();
        self.temp_serial = None;
        self.temp_module = None;
        self.sender.send(self.state)?;
        Ok(())
    }

    // TODO: implement manual stepping
    pub fn run_to_end(&mut self) -> DiagnosisResult {
        loop {

            match self.run_state() {
                Ok(result) => {
                    if let Some(result) = result {
                        break Ok(result);
                    }
                }
                Err(e) => {
                    self.reset()?;
                    return Err(e);
                }

            }

        }
    }

}
