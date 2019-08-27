use crate::entities::Statement; 
use crate::commands::Command; 
use crate::entities::ErrorEntry;

pub trait CommandRecognizer {
    fn from_statement(&self, statement: &Statement, errs_acc: &mut Vec<ErrorEntry>) -> Command; 
}