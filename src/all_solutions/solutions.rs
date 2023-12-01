pub trait Solvable {
    
    fn first_solution(&self) -> String;

    fn second_solution(&self) -> String{
        return String::new(); 
    }

}

