use std::collections::HashMap;
use std::string::String;

#[derive(Debug)]
struct Memory {
    pointer: usize,
    map: HashMap<usize, u8>
}

impl Memory {
    fn init() -> Memory {
        Memory { pointer: 0, map: HashMap::new() }
    }

    fn inc_ptr(&mut self) {
        self.pointer += 1;
    }

    fn dec_ptr(&mut self) {
        self.pointer -= 1;
    }

    fn get(&mut self) -> u8 {
        match self.map.get(&self.pointer) {
            Some(x) => *x as u8,
            None => 0
        }
    }

    fn set(&mut self, val : u8) {
        self.map.insert(self.pointer, val);
    }

    fn inc_val(&mut self) {
        let new_val = self.get() + 1;
        self.set(new_val);
    }

    fn dec_val(&mut self) {
        let new_val = self.get() - 1;
        self.set(new_val);
    }

    fn print(&mut self) {
        let ch = self.get() as char;
        print!("{}", ch);
    }
}


// >​ 	incrémente (augmente de 1) le pointeur.
// <​ 	décrémente (diminue de 1) le pointeur.
// +​ 	incrémente l'octet du tableau sur lequel est positionné le pointeur (l'octet pointé).
// -​ 	décrémente l'octet pointé.
// .​ 	sortie de l'octet pointé (valeur ASCII).
// ,​ 	entrée d'un octet dans le tableau à l'endroit où est positionné le pointeur (valeur ASCII).
// [​ 	saute à l'instruction après le ] correspondant si l'octet pointé est à 0.
// ]​ 	retourne à l'instruction après le [ si l'octet pointé est différent de 0.
fn main() {
    // let input = String::from("+++++++++++++++++++++++++++++++++++++++++++++++++.");
    // let input = String::from("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    let input = String::from("    +++++++++++ >+>>>>++++++++++++++++++++++++++++++++++++++++++++ >++++++++++++++++++++++++++++++++<<<<<<[>[>>>>>>+> +<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<[>++++++++++[- <-[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]>[<<[>>>+<<< -]>>[-]]<<]>>>[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]] >[<<+>>[-]]<<<<<<<]>>>>>[+++++++++++++++++++++++++ +++++++++++++++++++++++.[-]]++++++++++<[->-<]>++++ ++++++++++++++++++++++++++++++++++++++++++++.[-]<< <<<<<<<<<<[>>>+>+<<<<-]>>>>[<<<<+>>>>-]<-[>>.>.<<< [-]]<<[>>+>+<<<-]>>>[<<<+>>>-]<<[<+>-]>[<+>-]<<<-]");
    let mut memory = Memory::init();
    run(&input, &mut memory);
    println!("")
}

fn run(input : &String, memory : &mut Memory) {
    println!("Running with {:?} and {:?}", input, memory);

    if let Some(c) = input.chars().next() {
        let rest = &str_rest_at(input, 1);
        // println!("Handling char {:?}", c);
        // println!("Rest is {:?}", rest);
        match c {
            '[' => enter_loop(rest, memory),
            ']' => return,
            '+' => {
                memory.inc_val();
                run(rest, memory)
            },
            '-' => {
                memory.dec_val();
                run(rest, memory)
            },
            '>' => {
                memory.inc_ptr();
                run(rest, memory)
            },
            '<' => {
                memory.dec_ptr();
                run(rest, memory)
            },
            '.' => {
                memory.print();
                run(rest, memory)
            },
            _ => run(rest, memory)
        }
    }

    
}

fn enter_loop(input : &String, memory : &mut Memory) {
    if memory.get() == 0 {
        if let Some(end) = input.find("]") {
            let rest = &str_rest_at(input, end+1);
            run(rest, memory);
        }
    } else {
        run(input, memory);
        enter_loop(input, memory);
    }
}

fn str_rest_at(string : &String, at : usize) -> String {
    let (_, tmp) = string.split_at(at);
    String::from(tmp)
}
