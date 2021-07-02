## Traits


- It's all about telling the compiler how to 'restrict' our code to logical soundness throughout the code base. Traits are blueprints for a given set of method signatures that can be "shared". In a house if you have a struct called 'livingRoomFloor' you can bind it to a trait called 'InstallFloors' which will provide it a few fn signatures that will inform the compiler to assure that our code fallows a particular pattern/structure of types. 
```rs
trait InstallFloors {
	fn install(&self) -> Birch;
	fn clean_up(&self) -> bool;
	
}
#[derive(Debug)]
struct Birch {
	wood: String,
	width: u8, 
	color: String, 
}
#[derive(Debug)]
struct LivingRoomFloor {
	area: u32,
	completed: bool, 
}
impl LivingRoomFloor {
    fn new() -> Self {
        Self {
            area: 100,
            completed: false, 
        }
    }
}

impl InstallFloors for LivingRoomFloor {
	fn install(&self) -> Birch {
		Birch{
			wood: String::from("PINE"),
			width: 10,
			color: String::from("red"), 
		}
		
	}
	fn clean_up(&self) -> bool {
	    true
	}
}


fn main() {
let job = LivingRoomFloor::new();
let job_done = job.install();
println!("We installed the {:?} floor and painted it {:?}", job_done.wood, job_done.color);
 
}
```

		- "A trait in Rust is a **group** ("grouping") of methods that are defined for a particular type ("associated or bond or scoped to/with a type that you defined or has already been defined"). Traits are an abstract definition of shared behavior amongst different types."

- What is the point? I think if we want a particular 'type' to have a restricted  method signatures. A trait is a way to define which types are allowed to interface with the methods. I can have a trait that has methods that only deal with 'String' or 'u8'. By saying `impl exampleTrait for myStruct`... I am telling the compiler that 'myStruct' will adopt the method signatures of 'exampleTrait' so keep track of when my code violates these type restrictions. This all promotes 'soundness' and 'consistency' throughout a program, reducing bugs caused by diff types or misused types. 
	- When I implement a Trait on a struct, I am saying I want the type constraints applied which will ensure that my code or the types I define will fallow the same logic as the trait. 

### wikipedia
- The type system supports a mechanism similar to type classes, called "traits", inspired directly by the Haskell language. This is a facility for ad hoc polymorphism, **achieved by adding constraints to type variable declarations**.
- The object system within Rust is based around implementations, traits and structured types. Implementations fulfill a role similar to that of classes within other languages and are defined with the keyword impl. Inheritance and polymorphism are provided by traits; they allow methods to be defined and mixed in to implementations. Structured types are used to define fields. Implementations and traits cannot define fields themselves, and only traits can provide inheritance.
