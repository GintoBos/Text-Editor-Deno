#![warn(
    clippy::all, 
    clippy::pedantic, 
    clippy::print_stdout, 
    clippy::arithmetic_side_effects, 
    clippy::as_conversions, 
    clippy::integer_division
)]
 //makes the module editor known to this one (it will look for editor.rs)
mod editor;
use editor::Editor;

fn main(){
    // calling a method on editor
    Editor::new().unwrap().run(); 
}



    
