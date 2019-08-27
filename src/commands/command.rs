use crate::commands::CircleCommand;
use crate::commands::ElementCommand;
use crate::commands::ElementEntryCommand;
use crate::commands::ImportCommand;
use crate::commands::PackageCommand;
use crate::commands::FunctionCommand; 

pub enum Command {
    None, 
    Circle(CircleCommand), 
    Element(ElementCommand),
    ElementEntry(ElementEntryCommand),
    Import(ImportCommand),
    Package(PackageCommand),
    Function(FunctionCommand),
}