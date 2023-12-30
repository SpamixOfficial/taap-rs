use std::{collections::HashMap, process::exit, str};


pub struct Argument {
    name: String,
    description: String,
    exit_statuses: HashMap<u8, String>,
    epilog: String,
    credits: String,
    args: (HashMap<String, (String, isize)>, HashMap<char, (String, isize, String)>),
    help_order: (Vec<String>, Vec<String>)
}

impl Argument {
    fn new(name: &str, description: &str, epilog: &str, credits: &str) -> Self {
        let mut args: (HashMap<String, (String, isize)>, HashMap<char, (String, isize, String)>) =
            (HashMap::new(), HashMap::new());
        let exit_statuses: HashMap<u8, String> = HashMap::new();
        let mut help_order: (Vec<String>, Vec<String>) = (vec![], vec![]);
        args.1.insert('h', ("help".to_string(), 0, "Use this to print this help message".to_string()));
        help_order.1.push('h'.to_string());
        Self {
            name: name.to_string(),
            description: description.to_string(),
            exit_statuses,
            epilog: epilog.to_string(),
            credits: credits.to_string(),
            args,
            help_order
        }
    }

    fn add_arg(&mut self, placeholder: &str, args: &str, help: Option<&str>) {
        let nargs = if args == "+" {
            -1
        } else {
            match args.to_string().parse::<usize>() {
                Ok(n) => n as isize,
                Err(_) => {
                    eprintln!("Error! \"args\" parameter must be either a positive integer, 0 or +");
                    exit(1);
                }
            }
        };
        self.help_order.0.push(placeholder.to_string());
        self.args.0.insert(placeholder.to_string(), (help.unwrap_or("").to_string(), nargs));
    }

    fn add_option(&mut self, mut short: char, long: &str, parameters: &str, help: Option<&str>) {
        if short == ' ' {
            short = '-'
        }; 

        let nargs = if parameters == "+" {
            -1
        } else {
            match parameters.to_string().parse::<usize>() {
                Ok(n) => n as isize,
                Err(_) => {
                    eprintln!("Error! \"parameters\" parameter must be either a positive integer, 0 or +");
                    exit(1);
                }
            }
        };
        
        if short == '-' {
            self.help_order.1.push(long.to_string());
        } else {
            self.help_order.1.push(short.to_string());
        };

        self.args.1.insert(short, (long.to_string(), nargs, help.unwrap_or("").to_string()));
    }

    fn print_help(&self) {
        let mut help_string = String::new();
        let options = &self.args.1;
        let pos_args = &self.args.0;
        let name = &self.name;
        let description = &self.description;
        let credits = &self.credits;
        let bottom_text = &self.epilog;
        let exit_statuses = &self.exit_statuses;
        let usage = format!("usage: {}", name);
        let help_orders = &self.help_order;
        help_string.push_str(format!("{}\n{}\n\nOptions:\n", description, usage).as_str());

        for option in &help_orders.1 {
            let key: char;
            let mut field = (&' ', &(String::new(), 0isize, String::new())); 
            if option.len() > 1 {
                let mut found = false;
                for (tempkey, tempvalues) in &*options {
                    if tempvalues.0 == option.to_owned() {
                        field = (tempkey, tempvalues);
                        found = true;
                        break
                    }
                }
                if found == false {
                    eprintln!("Exception, couldn't get order of value in help message");
                    exit(1);
                }
            } else {
                field = options.get_key_value(&option.chars().nth(0).unwrap()).unwrap()
            };
            if field.0.to_owned() == '-' {
                key = ' ';
            } else {
                key = field.0.to_owned();
            };
            let values = field.1;
            help_string.push_str(format!("{}{}\t--{}\t\t{}\n", if key == ' '{ "" } else { "-" }, key, values.0, values.2).as_str());
        }

        if exit_statuses.len() > 1 {
            for (key, value) in &*exit_statuses {
                help_string.push_str(format!("Exit Statuses:\n\t{}\t{}\n", key, value).as_str());
            }
        };
        help_string.push_str(format!("\n{}\n{}\n", bottom_text, credits).as_str());

        println!("{}", help_string);
    }

    fn parse_args(&mut self) -> HashMap<String, (bool, Vec<String>)> {
        let raw_args = std::env::args();
        let mut collected_raw_args: Vec<String> = std::env::args().collect();
        collected_raw_args.remove(0);
        let options = &self.args.1;
        let mut return_map: HashMap<String, (bool, Vec<String>)> = HashMap::new();
        for (key, val) in options.iter() {
            let name: String;
            if key.to_owned() == '-' {
                name = val.0.to_owned();
            } else {
                name = key.to_string();
            };
            return_map.insert(name, (false, vec![]));
        }
        for (pos, argument) in raw_args.into_iter().enumerate() {
            if pos == 0 {
                continue;
            };
            if argument.len() > 1
                && argument.starts_with("-")
                && argument.chars().nth(1).unwrap() != '-'
            {
                for part in argument.get(1..).unwrap().chars() {
                    if options.contains_key(&part) {
                        let options_needed = options.get(&part).unwrap().1;
                        if collected_raw_args.len() < pos + options_needed as usize {
                            eprintln!("Too few options passed to -{}", &part);
                            exit(1);
                        }
                        *return_map.get_mut(&part.to_string()).unwrap() = (
                            true,
                            collected_raw_args[pos..(pos + options_needed as usize)]
                                .iter()
                                .cloned()
                                .collect(),
                        );
                    };
                }
            } else if argument.len() > 2 && argument.get(..2).unwrap() == "--" {
                let part = argument.get(2..).unwrap();
                for (key, values) in &*options {
                    if part == values.0 {
                        let name: String;
                        if key.to_owned() != '-' {
                            name = key.to_string();
                        } else {
                            name = part.to_string();
                        };
                        let options_needed = values.1;
                        *return_map.get_mut(&name).unwrap() = (
                            true,
                            collected_raw_args[pos..(pos + options_needed as usize)]
                                .iter()
                                .cloned()
                                .collect(),
                        );
                    }
                }
            }
        }
        if return_map.get("h").unwrap().0 == true {
            self.print_help();
            exit(0);
        };
        return_map
    }
}
