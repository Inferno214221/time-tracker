use std::fmt::{self, Display, Formatter};

use typst::{diag::SourceDiagnostic, ecow::EcoVec};

pub struct DisplayErrors(pub EcoVec<SourceDiagnostic>);

impl Display for DisplayErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for error in &self.0 {
            write!(f, "\n{}", error.message)?;
            if let Some(trace_loc) = error.trace.last() {
                write!(f, "\n{}", trace_loc.v)?;
            }
        }
        Ok(())
    }
}