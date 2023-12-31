use std::{collections::HashMap, process::exit, str};

pub struct Argument {
    name: String,
    description: String,
    exit_statuses: HashMap<u16, String>,
    epilog: String,
    credits: String,
    args: (
        HashMap<String, (String, isize)>,
        HashMap<char, (String, isize, String)>,
    ),
    help_order: (Vec<String>, Vec<String>, Vec<u16>),
}

impl Argument {
    pub fn new(name: &str, description: &str, epilog: &str, credits: &str) -> Self {
        let mut args: (
            HashMap<String, (String, isize)>,
            HashMap<char, (String, isize, String)>,
        ) = (HashMap::new(), HashMap::new());
        let exit_statuses: HashMap<u16, String> = HashMap::new();
        let mut help_order: (Vec<String>, Vec<String>, Vec<u16>) = (vec![], vec![], vec![]);
        args.1.insert(
            'h',
            (
                "help".to_string(),
                0,
                "Use this to print this help message".to_string(),
            ),
        );
        help_order.1.push('h'.to_string());
        Self {
            name: name.to_string(),
            description: description.to_string(),
            exit_statuses,
            epilog: epilog.to_string(),
            credits: credits.to_string(),
            args,
            help_order,
        }
    }

    pub fn add_exit_status(&mut self, code: u16, help: &str) {
        self.help_order.2.push(code);
        self.exit_statuses.insert(code, help.to_string());
    }

    pub fn add_arg(&mut self, placeholder: &str, args: &str, help: Option<&str>) {
        let nargs = if args == "+" {
            -1
        } else {
            match args.to_string().parse::<usize>() {
                Ok(n) => n as isize,
                Err(_) => {
                    eprintln!(
                        "Error! \"args\" parameter must be either a positive integer, 0 or +"
                    );
                    exit(1);
                }
            }
        };
        self.help_order.0.push(placeholder.to_string());
        self.args.0.insert(
            placeholder.to_string(),
            (help.unwrap_or("").to_string(), nargs),
        );
    }

    pub fn add_option(
        &mut self,
        mut short: char,
        long: &str,
        parameters: &str,
        help: Option<&str>,
    ) {
        if short == ' ' {
            short = '-'
        };

        let nargs = if parameters == "+" {
            -1
        } else {
            match parameters.to_string().parse::<usize>() {
                Ok(n) => n as isize,
                Err(_) => {
                    eprintln!(
                        "Error! \"parameters\" parameter must be either a positive integer, 0 or +"
                    );
                    exit(1);
                }
            }
        };

        if short == '-' {
            self.help_order.1.push(long.to_string());
        } else {
            self.help_order.1.push(short.to_string());
        };

        self.args.1.insert(
            short,
            (long.to_string(), nargs, help.unwrap_or("").to_string()),
        );
    }

    pub fn print_help(&self) {
        let mut help_string = String::new();
        let options = &self.args.1;
        let pos_args = &self.args.0;
        let name = &self.name;
        let description = &self.description;
        let credits = &self.credits;
        let bottom_text = &self.epilog;
        let exit_statuses = &self.exit_statuses;
        let mut usage = format!("Usage: {}", name);
        let mut pos_args_help = String::new();
        let help_orders = &self.help_order;

        for argument in &help_orders.0 {
            let values = pos_args.get(argument).unwrap();
            let nargs = values.1;
            let help = &values.0;
            usage.push_str(format!(" {}", argument).as_str());
            if nargs > 1 {
                usage.push_str(format!("*{}", nargs).as_str());
            }
            pos_args_help.push_str(format!("\n    {argument}\t\t{help}").as_str());
        }

        usage.push_str(" [OPTIONS]\n");

        help_string.push_str(
            format!(
                "{}{}\n\nPositional Arguments:{}\n\nOptions:\n",
                usage, description, pos_args_help
            )
            .as_str(),
        );

        for option in &help_orders.1 {
            let key: char;
            let mut field = (&' ', &(String::new(), 0isize, String::new()));
            if option.len() > 1 {
                let mut found = false;
                for (tempkey, tempvalues) in &*options {
                    if tempvalues.0 == option.to_owned() {
                        field = (tempkey, tempvalues);
                        found = true;
                        break;
                    }
                }
                if found == false {
                    eprintln!("Exception, couldn't get order of value in help message");
                    exit(1);
                }
            } else {
                field = options
                    .get_key_value(&option.chars().nth(0).unwrap())
                    .unwrap()
            };
            if field.0.to_owned() == '-' {
                key = ' ';
            } else {
                key = field.0.to_owned();
            };
            let values = field.1;
            help_string.push_str(
                format!(
                    "    {}{}\t--{}\t\t{}\n",
                    if key == ' ' { "" } else { "-" },
                    key,
                    values.0,
                    values.2
                )
                .as_str(),
            );
        }

        if exit_statuses.len() > 1 {
            help_string.push_str("\n\nExit Statuses:");
            for key in &help_orders.2 {
                let value = exit_statuses.get(key).unwrap();
                help_string.push_str(format!("\n    {}\t{}", key, value).as_str());
            }
        };

        help_string.push_str(format!("\n\n{}\n{}", bottom_text, credits).as_str());

        println!("{}", help_string);
    }

    pub fn parse_args(&mut self) -> HashMap<String, (bool, Vec<String>)> {
        let raw_args = std::env::args();
        let mut collected_raw_args: Vec<String> = std::env::args().collect();
        collected_raw_args.remove(0);
        let positional_arguments = &self.args.0;
        let mut positional_arguments_length = 0;
        let positional_arguments_order = &self.help_order.0;
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

        for key in positional_arguments.iter() {
            return_map.insert(key.0.to_owned(), (true, vec![]));
            positional_arguments_length += key.1 .1 as usize;
        } 

        // handling optional arguments
        for (pos, argument) in raw_args.into_iter().enumerate() {
            if pos < positional_arguments_length + 1 && argument != "-h" && argument != "--help" {
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
        }; 
        if return_map.get("h").unwrap().0 == true {
            self.print_help();
            exit(0);
        };

        // handling positional_arguments
        let mut current_argument_position: usize = 0;
        for argument in positional_arguments_order {
            let argument_length = positional_arguments.get(argument).unwrap().1 as usize;
            if current_argument_position + argument_length > collected_raw_args.len() {
                eprintln!("Error! Too few arguments supplied to positional argument {}", argument);
                exit(1);
            }
            *return_map.get_mut(argument).unwrap() = (
                true,
                collected_raw_args
                    [current_argument_position..current_argument_position + argument_length]
                    .iter()
                    .cloned()
                    .collect(),
            );
            current_argument_position += argument_length;
        };

        return_map
    }
}
