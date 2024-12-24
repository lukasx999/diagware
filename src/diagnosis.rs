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




pub const STATE_COUNT: u32 = 8; // needed for rendering state machine

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum State {
    #[default] Idle = 0, // Start
    ReadSerial      = 1,
    DBLookup        = 2,
    SwitchMatrix    = 3,
    ApplySignals    = 4,
    Measurements    = 5,
    Evaluation      = 6,
    End             = 7, // NOTE: not included in `STATE_COUNT` (implementationdetail)
}

impl State {
    pub fn from_u32(num: u32) -> Self {
        match num {
            0 => Self::Idle,
            1 => Self::ReadSerial,
            2 => Self::DBLookup,
            3 => Self::SwitchMatrix,
            4 => Self::ApplySignals,
            5 => Self::Measurements,
            6 => Self::Evaluation,
            7 => Self::End,
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
            S::DBLookup      => "DB Lookup",
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
    #[error("EEPROM operation failed")]
    EepromError(#[from] eeprom::EepromError),

    // The current DUT is holding a serial number which was not found within the database
    // UnknownModule {
    //     serial: String,
    // },
}



// Holds the results of a completed state
#[derive(Debug, Clone, Copy)]
pub enum Report {
    Pending,
    Completed {
        is_functional: bool,
    },
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
    temp_serial: Option<String>,
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
            temp_serial: None,
            temp_module: None,
        }
    }

    fn next_state(&mut self) -> Result<(), Failure> {

        println!("current state: {}", self.state);

        use State as DS;
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

        // TODO: think about this
        self.sender.send(self.state)?;

        Ok(())

    }

    fn do_stuff() {
        thread::sleep(Duration::from_millis(500));
    }


    /* Executes the current state, and transitions to the next state                  */
    /* Returns Ok(None) if state execution was successful, else returns error         */
    /* Returns Ok(Some(DiagnosisResult)) if the last state was executed successfully  */
    pub fn run_state(&mut self) -> DiagnosisResult {

        use State as S;
        match self.state {

            S::Idle => {
                Self::do_stuff();
                self.next_state()?;
            }

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
                return Ok(Report::Completed { is_functional: true });
            }

        }

        Ok(Report::Pending)

    }

    pub fn reset(&mut self) -> Result<(), Failure> {
        self.state = State::default();
        self.temp_serial = None;
        self.temp_module = None;
        self.sender.send(self.state)?;
        Ok(())
    }

    pub fn run_to_end(&mut self) -> DiagnosisResult {
        loop {
            match self.run_state() {
                Ok(result) => {
                    if let Report::Completed { .. } = result {
                        break Ok(result);
                    }
                }
                Err(e) => {
                    self.reset()?;
                    break Err(e);
                }

            }
        }
    }

}
