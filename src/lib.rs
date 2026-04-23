use std::{env::args, io, process::exit};
use terminal_size::terminal_size;

pub enum Query {
    ShowTable,
    ListAll,
    ListPeriod(usize),
    ListGroup(usize),
    ListClass(Class),
    ShowElement(String),
    PrintHelp,
}

#[derive(PartialEq)]
pub enum Class { AlkaliMetal, AlkilineEarthMetal, PostTransitionMetal, TransitionMetal, Lanthanide, Metalloid, NonMetal, Halogen, InertGas, Actinide }

impl Class {
    fn to_string(&self) -> String {
        match self {
            Class::Actinide => "Actinide".to_string(),
            Class::AlkaliMetal => "Alkali metal".to_string(),
            Class::AlkilineEarthMetal => "Alkiline earth metal".to_string(),
            Class::Halogen => "Halogen".to_string(),
            Class::InertGas => "Inert gas".to_string(),
            Class::Lanthanide => "Lanthanide".to_string(),
            Class::Metalloid => "Metalloid".to_string(),
            Class::NonMetal => "Non-metal".to_string(),
            Class::TransitionMetal => "Transition metal".to_string(),
            Class::PostTransitionMetal => "Post-transition metal".to_string(),
        }
    }
}

pub fn parse_args() -> Query {
    let mut args = args(); args.next();
    let command = match args.next() {
        Some(s) => {s}
        None => {return Query::PrintHelp}
    };
    match command.as_str() {
        "-h" | "--help" => {return Query::PrintHelp;}
        "-a" | "--all" => {return Query::ListAll;}
        "-t" | "--table" => {return Query::ShowTable;}
        "-f" | "--find" => {
            match args.next() {
                Some(s) => {return Query::ShowElement(s.to_lowercase());}
                None => {eprintln!("Expected element identifier. Use '-h/--help' for more info."); exit(1)}
            }
        }
        "-p" | "--period" => {
            match args.next() {
                Some(s) => {
                    let num = match s.parse::<usize>() {
                        Ok(n) => {n}
                        Err(e) => {eprintln!("Error reading number: {}", e); exit(1)}
                    };
                    if !(1..=7).contains(&num) {eprintln!("Period number must be between 1 and 7."); exit(1)}
                    return Query::ListPeriod(num);
                }
                None => {eprintln!("Expected period number. Use '-h/--help' for more info."); exit(1)}
            }
        }
        "-g" | "--group" => {
            match args.next() {
                Some(s) => {
                    let num = match s.parse::<usize>() {
                        Ok(n) => {n}
                        Err(e) => {eprintln!("Error reading number: {}", e); exit(1)}
                    };
                    if !(1..=18).contains(&num) {eprintln!("Group number must be between 1 and 18."); exit(1)}
                    return Query::ListGroup(num);
                }
                None => {eprintln!("Expected group number. Use '-h/--help' for more info."); exit(1)}
            }
        }
        "-c" | "--class" => {
            println!("Select a class:");
            println!("[1] Alkali metal");
            println!("[2] Alkiline earth metal");
            println!("[3] Transition metal");
            println!("[4] Post-transition metal");
            println!("[5] Metalloid");
            println!("[6] Non-metal");
            println!("[7] Halogen");
            println!("[8] Inert gas");
            println!("[9] Lanthanide");
            println!("[10] Actinide");
            loop {
                let mut uinput = "".to_string();
                if let Err(e) = io::stdin().read_line(&mut uinput) {eprintln!("Error while reading user input: {}", e); exit(1)}
                uinput = uinput.trim().to_string();
                let number = match uinput.parse::<usize>() {
                    Ok(n) => {n}
                    Err(_) => {println!("Try again."); continue;}
                };
                match number {
                    1 => {println!(); return Query::ListClass(Class::AlkaliMetal);}
                    2 => {println!(); return Query::ListClass(Class::AlkilineEarthMetal);}
                    3 => {println!(); return Query::ListClass(Class::TransitionMetal);}
                    4 => {println!(); return Query::ListClass(Class::PostTransitionMetal);}
                    5 => {println!(); return Query::ListClass(Class::Metalloid);}
                    6 => {println!(); return Query::ListClass(Class::NonMetal);}
                    7 => {println!(); return Query::ListClass(Class::Halogen);}
                    8 => {println!(); return Query::ListClass(Class::InertGas);}
                    9 => {println!(); return Query::ListClass(Class::Lanthanide);}
                    10 => {println!(); return Query::ListClass(Class::Actinide);}
                    _ => {println!("Try again."); continue;}
                }
            }
        }
        _ => {eprintln!("Unrecognized command. Use '-h/--help' for more info."); exit(1)}
    }
}

fn up(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let first_upper = chars[0].to_ascii_uppercase();
    chars[0] = first_upper;
    chars.iter().collect()
}

enum Cell<'a> {
    Single(Element<'a>),
    Bundle(&'a [Element<'a>]),
    Void,
}

struct Element<'a> {
    symbol: &'a str,
    name: &'a str,
    number: u8,
    mass: f32,
    class: Class,
}
impl<'a> Element<'a> {
    
    const fn new(symbol: &'a str, name: &'a str, number: u8, mass: f32, class: Class) -> Self {
        Element {symbol, name, number, mass, class}
    }

    fn print(&self, period: usize, group: usize) {
        println!("[{}] {}", up(self.symbol), up(self.name));
        println!("Atomic number:   {}", self.number);
        if self.mass.fract() == 0.0000 {println!("Atomic mass:     ({})", self.mass as u64);}
        else {println!("Atomic mass:     {}", self.mass);}
        println!("Period (shells): {}", period);
        println!("Group:           {}", group);
        println!("Element class:   {}", self.class.to_string());
    }
}

const TABLE: [&[Cell; 18]; 7] = [
    &[
        Cell::Single(Element::new("h", "hydrogen", 1, 1.0079, Class::NonMetal)),
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Single(Element::new("he", "helium", 2, 4.0026, Class::InertGas)),
    ],
    &[
        Cell::Single(Element::new("li", "lithium", 3, 6.94,  Class::AlkaliMetal)),
        Cell::Single(Element::new("be", "beryllium", 4, 9.0122,  Class::AlkilineEarthMetal)),
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Single(Element::new("b", "boron", 5, 10.81, Class::Metalloid)),
        Cell::Single(Element::new("c", "carbon", 6, 12.011, Class::NonMetal)),
        Cell::Single(Element::new("n", "nitrogen", 7, 14.007, Class::NonMetal)),
        Cell::Single(Element::new("o", "oxygen", 8, 15.999, Class::NonMetal)),
        Cell::Single(Element::new("f", "fluorine", 9, 18.998, Class::Halogen)),
        Cell::Single(Element::new("ne", "neon", 10, 20.180, Class::InertGas)),
    ],
    &[
        Cell::Single(Element::new("na", "sodium", 11, 22.990, Class::AlkaliMetal)),
        Cell::Single(Element::new("mg", "magnesium", 12, 24.305, Class::AlkilineEarthMetal)),
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Void,
        Cell::Single(Element::new("al", "aluminium", 13, 26.982, Class::PostTransitionMetal)),
        Cell::Single(Element::new("si", "silicon", 14, 28.085, Class::Metalloid)),
        Cell::Single(Element::new("p", "phosphorus", 15, 30.974, Class::NonMetal)),
        Cell::Single(Element::new("s", "sulfur", 16, 32.06, Class::NonMetal)),
        Cell::Single(Element::new("cl", "chlorine", 17, 35.45, Class::Halogen)),
        Cell::Single(Element::new("ar", "argon", 18, 39.948, Class::InertGas)),
    ],
    &[
        Cell::Single(Element::new("k", "potassium", 19, 39.098, Class::AlkaliMetal)),
        Cell::Single(Element::new("ca", "calcium", 20, 40.078, Class::AlkilineEarthMetal)),
        Cell::Single(Element::new("sc", "scandium", 21, 44.956, Class::TransitionMetal)),
        Cell::Single(Element::new("ti", "titanium", 22, 47.867, Class::TransitionMetal)),
        Cell::Single(Element::new("v", "vanadium", 23, 50.942, Class::TransitionMetal)),
        Cell::Single(Element::new("cr", "chromium", 24, 51.996, Class::TransitionMetal)),
        Cell::Single(Element::new("mn", "manganese", 25, 54.938, Class::TransitionMetal)),
        Cell::Single(Element::new("fe", "iron", 26, 55.845, Class::TransitionMetal)),
        Cell::Single(Element::new("co", "cobalt", 27, 58.933, Class::TransitionMetal)),
        Cell::Single(Element::new("ni", "nickel", 28, 58.693, Class::TransitionMetal)),
        Cell::Single(Element::new("cu", "copper", 29, 63.546, Class::TransitionMetal)),
        Cell::Single(Element::new("zn", "zinc", 30, 65.38, Class::TransitionMetal)),
        Cell::Single(Element::new("ga", "gallium", 31, 69.723, Class::PostTransitionMetal)),
        Cell::Single(Element::new("ge", "germanium", 32, 72.63, Class::Metalloid)),
        Cell::Single(Element::new("as", "arsenic", 33, 74.922, Class::Metalloid)),
        Cell::Single(Element::new("se", "selenium", 34, 78.96, Class::NonMetal)),
        Cell::Single(Element::new("br", "bromine", 35, 79.904, Class::Halogen)),
        Cell::Single(Element::new("kr", "krypton", 36, 83.798, Class::InertGas)),
    ],
    &[
        Cell::Single(Element::new("rb", "rubidium", 37, 85.468, Class::AlkaliMetal)),
        Cell::Single(Element::new("sr", "strontium", 38, 87.62, Class::AlkilineEarthMetal)),
        Cell::Single(Element::new("y", "yttrium", 39, 88.906, Class::TransitionMetal)),
        Cell::Single(Element::new("zr", "zirconium", 40, 91.224, Class::TransitionMetal)),
        Cell::Single(Element::new("nb", "niobium", 41, 92.906, Class::TransitionMetal)),
        Cell::Single(Element::new("mo", "molybdenum", 42, 95.96, Class::TransitionMetal)),
        Cell::Single(Element::new("tc", "technetium", 43, 97.91, Class::TransitionMetal)),
        Cell::Single(Element::new("ru", "ruthenium", 44, 101.07, Class::TransitionMetal)),
        Cell::Single(Element::new("rh", "rhodium", 45, 102.91, Class::TransitionMetal)),
        Cell::Single(Element::new("pd", "palladium", 46, 106.42, Class::TransitionMetal)),
        Cell::Single(Element::new("ag", "silver", 47, 107.87, Class::TransitionMetal)),
        Cell::Single(Element::new("cd", "cadmium", 48, 112.41, Class::TransitionMetal)),
        Cell::Single(Element::new("in", "indium", 49, 114.82, Class::PostTransitionMetal)),
        Cell::Single(Element::new("sn", "tin", 50, 118.71, Class::PostTransitionMetal)),
        Cell::Single(Element::new("sb", "antimony", 51, 121.76, Class::Metalloid)),
        Cell::Single(Element::new("te", "tellurium", 52, 127.60, Class::Metalloid)),
        Cell::Single(Element::new("i", "iodine", 53, 126.90, Class::Halogen)),
        Cell::Single(Element::new("xe", "xenon", 54, 131.29, Class::InertGas)),
    ],
    &[
        Cell::Single(Element::new("cs", "caesium", 55, 132.91, Class::AlkaliMetal)),
        Cell::Single(Element::new("ba", "barium", 56, 137.33, Class::AlkilineEarthMetal)),
        Cell::Bundle(
            &[
                Element::new("la", "lanthanum", 57, 138.91, Class::Lanthanide),
                Element::new("ce", "cerium", 58, 140.12, Class::Lanthanide),
                Element::new("pr", "praseodymium", 59, 140.91, Class::Lanthanide),
                Element::new("nd", "neodymium", 60, 144.24, Class::Lanthanide),
                Element::new("pm", "promethium", 61, 144.91, Class::Lanthanide),
                Element::new("sm", "samarium", 62, 150.36, Class::Lanthanide),
                Element::new("eu", "europium", 63, 151.96, Class::Lanthanide),
                Element::new("gd", "gadolinium", 64, 157.25, Class::Lanthanide),
                Element::new("tb", "terbium", 65, 158.93, Class::Lanthanide),
                Element::new("dy", "dysprosium", 66, 162.50, Class::Lanthanide),
                Element::new("ho", "holmium", 67, 164.93, Class::Lanthanide),
                Element::new("er", "erbium", 68, 167.26, Class::Lanthanide),
                Element::new("tm", "thulium", 69, 168.93, Class::Lanthanide),
                Element::new("yb", "ytterbium", 70, 173.05, Class::Lanthanide),
                Element::new("lu", "lutetium", 71, 174.97, Class::Lanthanide),
            ]
        ),
        Cell::Single(Element::new("hf", "hafnium", 72, 178.49, Class::TransitionMetal)),
        Cell::Single(Element::new("ta", "tantalum", 73, 180.95, Class::TransitionMetal)),
        Cell::Single(Element::new("w", "tungsten", 74, 183.84, Class::TransitionMetal)),
        Cell::Single(Element::new("re", "rhenium", 75, 186.21, Class::TransitionMetal)),
        Cell::Single(Element::new("os", "osmium", 76, 190.23, Class::TransitionMetal)),
        Cell::Single(Element::new("ir", "iridium", 77, 192.22, Class::TransitionMetal)),
        Cell::Single(Element::new("pt", "platinum", 78, 195.08, Class::TransitionMetal)),
        Cell::Single(Element::new("au", "gold", 79, 196.97, Class::TransitionMetal)),
        Cell::Single(Element::new("hg", "mercury", 80, 200.59, Class::TransitionMetal)),
        Cell::Single(Element::new("ti", "thallium", 81, 204.38, Class::PostTransitionMetal)),
        Cell::Single(Element::new("pb", "lead", 82, 207.2, Class::PostTransitionMetal)),
        Cell::Single(Element::new("bi", "bismuth", 83, 208.98, Class::PostTransitionMetal)),
        Cell::Single(Element::new("po", "polonium", 84, 209.0, Class::Metalloid)),
        Cell::Single(Element::new("at", "astatine", 85, 210.0, Class::Halogen)),
        Cell::Single(Element::new("rn", "radon", 86, 222.0, Class::InertGas)),
    ],
    &[
        Cell::Single(Element::new("fr", "francium", 87, 223.0, Class::AlkaliMetal)),
        Cell::Single(Element::new("ra", "radium", 88, 226.0, Class::AlkilineEarthMetal)),
        Cell::Bundle(&[
            Element::new("ac", "actinium", 89, 227.0, Class::Actinide),
            Element::new("th", "thorium", 90, 232.04, Class::Actinide),
            Element::new("pa", "protactinium", 91, 231.04, Class::Actinide),
            Element::new("u", "uranium", 92, 238.03, Class::Actinide),
            Element::new("np", "neptunium", 93, 237.0, Class::Actinide),
            Element::new("pu", "plutonium", 94, 244.0, Class::Actinide),
            Element::new("am", "americium", 95, 243.0, Class::Actinide),
            Element::new("cm", "curium", 96, 247.0, Class::Actinide),
            Element::new("bk", "berkelium", 97, 247.0, Class::Actinide),
            Element::new("cf", "californium", 98, 251.0, Class::Actinide),
            Element::new("es", "einsteinium", 99, 252.0, Class::Actinide),
            Element::new("fm", "fermium", 100, 257.0, Class::Actinide),
            Element::new("md", "mendelevium", 101, 258.0, Class::Actinide),
            Element::new("no", "nobelium", 102, 259.0, Class::Actinide),
            Element::new("lr", "lawrencium", 103, 262.0, Class::Actinide),
        ]),
        Cell::Single(Element::new("rf", "rutherfordium", 104, 267.0, Class::TransitionMetal)),
        Cell::Single(Element::new("db", "dubnium", 105, 268.0, Class::TransitionMetal)),
        Cell::Single(Element::new("sg", "seaborgium", 106, 271.0, Class::TransitionMetal)),
        Cell::Single(Element::new("bh", "bohrium", 107, 270.0, Class::TransitionMetal)),
        Cell::Single(Element::new("hs", "hassium", 108, 277.0, Class::TransitionMetal)),
        Cell::Single(Element::new("mt", "meitnerium", 109, 276.0, Class::TransitionMetal)),
        Cell::Single(Element::new("ds", "darmstadtium", 110, 281.0, Class::TransitionMetal)),
        Cell::Single(Element::new("rg", "roentgenium", 111, 280.0, Class::TransitionMetal)),
        Cell::Single(Element::new("cn", "copernicium", 112, 285.0, Class::TransitionMetal)),
        Cell::Single(Element::new("nh", "nihonium", 113, 286.0, Class::PostTransitionMetal)),
        Cell::Single(Element::new("fl", "flerovium", 114, 289.0, Class::PostTransitionMetal)),
        Cell::Single(Element::new("mc", "moscovium", 115, 288.0, Class::PostTransitionMetal)),
        Cell::Single(Element::new("lv", "livermorium", 116, 293.0, Class::PostTransitionMetal)),
        Cell::Single(Element::new("ts", "tennessine", 117, 294.0, Class::Halogen)),
        Cell::Single(Element::new("og", "oganesson", 118, 294.0, Class::InertGas)),
    ],
];

pub fn run(query: Query) {
    match query {
        Query::PrintHelp => {print_help();}
        Query::ShowTable => {
            let (term_width, _) = match terminal_size() {
                Some(x) => {x}
                None => {eprintln!("Error detecting terminal size."); exit(1)}
            };
            if term_width.0 < 75 {eprintln!("This terminal is too small."); exit(1)}
            println!("P\\G 1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18 ");
            let mut bundles = vec![];
            for (period_number, period) in TABLE.iter().enumerate() {
                print!(" {} ", period_number+1);
                for (_, cell) in period.iter().enumerate() {
                    match cell {
                        Cell::Void => {print!("    ");}
                        Cell::Single(element) => {
                            if element.symbol.len() == 1 {print!("[{} ]", up(element.symbol))}
                            else {print!("[{}]", up(element.symbol))}
                        }
                        Cell::Bundle(bundle) => {
                            print!("[..]");
                            bundles.push(bundle);
                        }
                    }
                }
                println!();
            }
            println!("            ||");
            for bundle in bundles {
                print!("           ");
                for element in *bundle {
                    if element.symbol.len() == 1 {print!("[{} ]", up(element.symbol))}
                    else {print!("[{}]", up(element.symbol))}
                }
                println!();
            }
        }
        Query::ListAll => {
            for (period_number, period) in TABLE.iter().enumerate() {
                for (group_number, cell) in period.iter().enumerate() {
                    match cell {
                        Cell::Void => {continue;}
                        Cell::Single(element) => {element.print(period_number+1, group_number+1); println!()}
                        Cell::Bundle(bundle) => {
                            for element in *bundle {element.print(period_number+1, group_number+1); println!()}
                        }
                    }
                }
            }
        }
        Query::ShowElement(id) => {
            for (period_number, period) in TABLE.iter().enumerate() {
               for (group_number, cell) in period.iter().enumerate() {
                    match cell {
                        Cell::Void => {continue;}
                        Cell::Single(element) => {
                            if element.symbol == &id || element.name == &id {element.print(period_number+1, group_number+1); exit(0)}
                        }
                        Cell::Bundle(bundle) => {
                            for element in *bundle {
                                if element.symbol == &id || element.name == &id {element.print(period_number+1, group_number+1); exit(0)}
                            }
                        }
                    }
                }
            }
            eprintln!("No element found with symbol or name '{id}'."); exit(1);
        }
        Query::ListPeriod(requested_period_number) => {
            for (period_number, period) in TABLE.iter().enumerate() {
                if period_number+1 != requested_period_number {continue;}
                for (group_number, cell) in period.iter().enumerate() {
                    match cell {
                        Cell::Void => {continue;}
                        Cell::Single(element) => {element.print(period_number+1, group_number+1); println!()}
                        Cell::Bundle(bundle) => {
                            for element in *bundle {element.print(period_number+1, group_number+1); println!()}
                        }
                    }
                }
            }
        }
        Query::ListGroup(requested_group_number) => {
            for (period_number, period) in TABLE.iter().enumerate() {
                for (group_number, cell) in period.iter().enumerate() {
                    if group_number+1 != requested_group_number {continue;}
                    match cell {
                        Cell::Void => {continue;}
                        Cell::Single(element) => {element.print(period_number+1, group_number+1); println!()}
                        Cell::Bundle(bundle) => {
                            for element in *bundle {element.print(period_number+1, group_number+1); println!()}
                        }
                    }
                }
            }
        }
        Query::ListClass(class) => {
            for (period_number, period) in TABLE.iter().enumerate() {
                for (group_number, cell) in period.iter().enumerate() {
                    match cell {
                        Cell::Void => {continue;}
                        Cell::Single(element) => {
                            if element.class == class {element.print(period_number+1, group_number+1); println!()}
                        }
                        Cell::Bundle(bundle) => {
                            for element in *bundle {
                                if element.class == class {element.print(period_number+1, group_number+1); println!()}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn print_help() {
    println!("Pt is a tiny program that lets you browse the periodic table of elements from your terminal.");
    println!("Commands:");
    println!("    $ pt -t/--table             : displays element symbols as a table.");
    println!("    $ pt -a/--all               : lists info on all known elements.");
    println!("    $ pt -f/--find [IDENTIFIER] : looks up a specific element by it's symbol or name.");
    println!("    $ pt -p/--period [NUMBER]   : lists info on all elements from one period.");
    println!("    $ pt -g/--group [NUMBER]    : lists info on all elements from one group.");
    println!("    $ pt -c/--class             : lists info on all elements from one class.");
    println!("    $ pt -h/--help              : prints this page.");
}