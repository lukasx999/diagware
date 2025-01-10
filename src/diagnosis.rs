use std::{
    thread,
    time::Duration,
    sync::mpsc,
};

use crate::{
    EEPROM,
    DDS,
    ShiftRegister,
    DB,
    db::model::{Module, Matrix, TargetValue},
};




pub const STATE_COUNT: u32 = 7; // needed for rendering state machine

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum State {
    #[default] Idle = 0, // Start
    ReadSerial      = 1,
    SwitchMatrix    = 2,
    ApplySignals    = 3,
    Measurements    = 4,
    Evaluation      = 5,
    End             = 6, // not included in `STATE_COUNT` (implementationdetail)
}



impl State {
    pub fn from_u32(num: u32) -> Self {
        match num {
            0 => Self::Idle,
            1 => Self::ReadSerial,
            2 => Self::SwitchMatrix,
            3 => Self::ApplySignals,
            4 => Self::Measurements,
            5 => Self::Evaluation,
            6 => Self::End,
            _ => panic!("Invalid integer"),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use State as S;
        let repr = match self {
            S::Idle          => "Idle",
            S::ReadSerial    => "Read Serial",
            S::SwitchMatrix  => "Switching Matrix",
            S::ApplySignals  => "Applying Signals",
            S::Measurements  => "Measurements",
            S::Evaluation    => "Evaluation",
            S::End           => "End"
        };
        write!(f, "{}", repr)
    }
}





#[derive(thiserror::Error, Debug)]
pub enum Failure {
    #[error("Failed to transmit current state")]
    SendError(#[from] mpsc::SendError<State>),
    #[error("Database operation failed")]
    DatabaseError(#[from] sqlx::Error),
    #[error("IO operation failed")]
    IoError(#[from] crate::io::IoError),
}



// Holds the results of a completed state
#[derive(Debug, Clone, Copy)]
pub enum Report {
    Pending,
    Completed { is_functional: bool, },
}


pub type DiagnosisResult = Result<Report, Failure>;



#[derive(Debug)]
pub struct Diagnosis {
    state:        State,
    sender:       mpsc::Sender<State>, // informing the receiver about change of state
    pub eeprom:   EEPROM,
    pub db:       DB,
    pub shiftreg: ShiftRegister,

    // Temporary values resulting from computations within the states
    temp_module: Option<Module>,
}

impl Diagnosis {

    pub fn new(eeprom: EEPROM,
        db: DB,
        shiftreg: ShiftRegister,
        sender: mpsc::Sender<State>,
    ) -> Self {
        Self {
            state: State::default(),
            sender,
            eeprom,
            db,
            shiftreg,
            temp_module: None,
        }
    }

    fn delay() {
        thread::sleep(Duration::from_millis(500));
    }

    fn reset_internal_state(&mut self) {
        self.temp_module = None;
    }

    // Transition to the next state
    pub fn next_state(&mut self) {
        use State as S;
        self.state = match self.state {
            S::Idle         => S::ReadSerial,
            S::ReadSerial   => S::SwitchMatrix,
            S::SwitchMatrix => S::ApplySignals,
            S::ApplySignals => S::Measurements,
            S::Measurements => S::Evaluation,
            S::Evaluation   => S::End,
            S::End          => S::Idle,
        };
        self.sender.send(self.state).unwrap();
    }

    // Execute the current state
    pub fn run_state(&mut self) -> DiagnosisResult {
        println!("{}: {}", self.state as u32, self.state);

        use State as S;
        match self.state {

            S::Idle => {
                Self::delay();
            }

            S::ReadSerial => {
                Self::delay();
                let serial: String = self.eeprom.get_serial()?;
                let module: Module = self.db.get_module_by_serial(&serial)?;
                self.temp_module   = Some(module);
            }

            S::SwitchMatrix => {
                Self::delay();
                let id = self.temp_module.as_ref().unwrap().id;
                let matrix: Matrix = self.db.get_matrix_by_id(id)?;
                self.shiftreg.switch(&matrix)?;
            }

            S::ApplySignals => {
                Self::delay();
            }

            S::Measurements => {
                Self::delay();
            }

            S::Evaluation => {
                // use crate::db::model::TargetValue;

                Self::delay();

                // // TODO: deal with unwrap()
                // // let id = self.temp_module.clone().unwrap().id.unwrap();
                // let id = self.temp_module.as_ref().unwrap().id.unwrap();
                // let targetvalues: Vec<TargetValue> = self.db.get_targetvalue_by_id(id)?;
                // // TODO: compare measured values with targetvalues
            }

            S::End => {
                Self::delay();
                self.reset_internal_state();
                return Ok(Report::Completed { is_functional: true });
            }

        }

        Ok(Report::Pending)

    }

    // Reset the statemachine
    pub fn reset_state(&mut self) {
        self.reset_internal_state();
        self.state = State::default();
        self.sender.send(self.state).unwrap();
    }

    // Execute the current state, and transition to the next state
    pub fn run_and_next(&mut self) -> DiagnosisResult {
        let report = self.run_state();
        self.next_state();
        report
    }

    // Run all states until the end has been reached (=> Automatic diagnosis)
    pub fn run_to_end(&mut self) -> DiagnosisResult {
        loop {
            match self.run_and_next() {
                Ok(result) => {
                    if let Report::Completed { .. } = result {
                        break Ok(result);
                    }
                }
                Err(e) => {
                    // self.reset();
                    break Err(e);
                }
            }
        }
    }

}
