
/* Linear FSM */

use std::{
    thread,
    time::Duration,
    sync::mpsc,
    error::Error,
};

use crate::{
    EEPROM,
    DB,
    db::model::Module,
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



// TODO: repr trait vs Display trait

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



// TODO: Error state
/*
In case of error: switch to error state
show popup if user wants to restart diagnosis or continue
OR:
In case of error: Return diagnosis result error and show error information as popup
*/




// TODO: replace with actual error type
pub type DiagnosisError = Box<dyn Error>;


#[derive(Debug)]
pub enum ActualDiagnosisError {
    SendError(mpsc::SendError<DiagnosisState>),
    DatabaseError(sqlx::Error),
}

impl From<mpsc::SendError<DiagnosisState>> for ActualDiagnosisError {
    fn from(value: mpsc::SendError<DiagnosisState>) -> Self {
        Self::SendError(value)
    }
}

impl From<sqlx::Error> for ActualDiagnosisError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value)
    }
}

impl std::fmt::Display for ActualDiagnosisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}

impl Error for ActualDiagnosisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}




#[derive(Debug, Clone, Copy)]
pub struct DiagnosisResult {
}

impl DiagnosisResult {
    pub fn new() -> Self {
        Self {}
    }
}




#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum DiagnosisMode {
    #[default] Automatic,
    Manual,
}






// TODO: this

/*
pub trait Repr {
    fn repr(&self) -> &str;
}

impl DiagnosisMode {
    fn repr(&self) -> &str {
        use DiagnosisMode as Mode;
        match self {
            Mode::Manual    => "Manuell",
            Mode::Automatic => "Automatisch",
        }
    }
}
*/





#[derive(Debug)]
pub struct Diagnosis {
    state:      DiagnosisState,
    sender:     mpsc::Sender<DiagnosisState>, // informing the receiver about change of state
    pub mode:   DiagnosisMode,
    pub eeprom: EEPROM,
    pub db:     DB,

    // Temporary values resulting from computations within the states
    temp_serial: Option<String>,
    temp_module: Option<Module>,
}

impl Diagnosis {

    pub fn new(eeprom: EEPROM, db: DB, sender: mpsc::Sender<DiagnosisState>) -> Self {
        Self {
            state: DiagnosisState::default(),
            sender,
            mode: DiagnosisMode::default(),
            eeprom,
            db,
            temp_serial: None,
            temp_module: None,
        }
    }

    fn next_state(&mut self) -> Result<(), DiagnosisError> {
        use DiagnosisState as DS;

        self.sender.send(self.state)?; // error enum: ui to diag connection failed

        println!("{}", self.state.repr());
        self.state = match self.state {
            DS::Idle         => DS::ReadSerial,
            DS::ReadSerial   => DS::DBLookup,
            DS::DBLookup     => DS::SelfTest,
            DS::SelfTest     => DS::Measurements,
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
    pub fn run_state(&mut self) -> Result<Option<DiagnosisResult>, DiagnosisError> {

        use DiagnosisState as State;
        match self.state {

            State::Idle => {
                self.next_state()?;
            }

            State::ReadSerial => {
                Self::do_stuff();
                let serial: String = self.eeprom.get_serial()?;
                self.temp_serial = Some(serial);
                self.next_state()?;
            }

            State::DBLookup => {
                Self::do_stuff();

                assert!(self.temp_serial != None);
                let serial = match &self.temp_serial {
                    Some(value) => value,
                    None => panic!(), // In this case its okay to panic, because this error should
                                      // not occur in a functional program
                };

                let module: Module = self.db.get_module_by_serial(&serial)?;
                self.temp_module = Some(module);

                self.next_state()?;
            }

            State::SelfTest => {
                Self::do_stuff();
                self.next_state()?;
            }

            State::Measurements => {
                Self::do_stuff();
                self.next_state()?;
            }

            State::Evaluation => {
                Self::do_stuff();
                self.next_state()?;
            }

            State::End => {
                self.reset()?;
                return Ok(Some(DiagnosisResult::new()));
            }

        }

        Ok(None)

    }

    pub fn reset(&mut self) -> Result<(), DiagnosisError> {
        self.state = DiagnosisState::default();
        self.sender.send(self.state)?;
        Ok(())
    }

    pub fn run_to_end(&mut self) -> Result<DiagnosisResult, DiagnosisError> {
        loop {
            if let Some(result) = self.run_state()? {
                break Ok(result);
            }
        }
    }

}
