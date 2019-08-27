use crate::entities::CodeChar; 
use crate::entities::Token; 

pub trait TokenRecognizer {
    fn recognize_token(&self, chars: &Vec<CodeChar>, position: usize, ) -> (Option<Token>, usize); 
}