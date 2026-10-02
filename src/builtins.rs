use std::time::{Duration, SystemTime};

use crate::{callable::Callable, error::CarlaeError, expr::LiteralValue};

#[derive(Debug)]
pub struct Clock {
    pub name: String,
}

impl Callable for Clock {
    fn call(
        &self,
        _args: Vec<crate::expr::LiteralValue>,
        _interpreter: &crate::interpreter::Interpreter,
    ) -> Result<crate::expr::LiteralValue, crate::error::CarlaeError> {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|t| LiteralValue::Number(Duration::as_secs_f64(&t)))
            .map_err(|e| CarlaeError::General(format!("Unable to determine system time: {e}")))
    }

    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> &str {
        &self.name
    }
}
