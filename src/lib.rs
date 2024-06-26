#![doc = include_str!("../docs/MAIN.md")]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    process::exit,
    str,
};

#[cfg(test)]
mod tests {
    use crate::Argument;
    use std::collections::BTreeMap;
    
    // test of "new" function
    #[test]
    fn new() {
        let mut args: (
            BTreeMap<String, (String, isize)>,
            BTreeMap<char, (String, isize, String)>,
        ) = (BTreeMap::new(), BTreeMap::new());

        let exit_statuses: BTreeMap<u16, String> = BTreeMap::new();

        args.1.insert(
            'h',
            (
                "help".to_string(),
                0,
                "Use this to print this help message".to_string(),
            ),
        );

        let expected_test_obj = Argument {
            name: String::from("Hello"),
            description: String::from("World"),
            exit_statuses,
            epilog: String::from("From"),
            credits: String::from("TAAP"),
            args,
        };

        let result_test_obj = Argument::new("Hello", "World", "From", "TAAP");

        assert_eq!(expected_test_obj, result_test_obj);
    }

    // test of "add_exit_status" function
    #[test]
    fn exit_status() {
        let mut expected_test_obj: BTreeMap<String, (bool, Vec<String>)> = BTreeMap::new();
        
        expected_test_obj.insert("h".to_string(), (false, vec![]));

        let mut argument_test_obj = Argument::new("Hello", "World", "From", "TAAP");
        
        argument_test_obj.add_exit_status(0, "Everything went well!");
        let result_test_obj = argument_test_obj.parse_args(None);

        assert_eq!(expected_test_obj, result_test_obj);
    }

    // test of "add_option" function
    #[test]
    fn options() {
        let mut expected_test_obj: BTreeMap<String, (bool, Vec<String>)> = BTreeMap::new();
        
        expected_test_obj.insert("f".to_string(), (false, vec![]));
        expected_test_obj.insert("h".to_string(), (false, vec![]));

        let mut argument_test_obj = Argument::new("Hello", "World", "From", "TAAP");
        
        argument_test_obj.add_option('f', "foo", "0", None);
        let result_test_obj = argument_test_obj.parse_args(None);

        assert_eq!(expected_test_obj, result_test_obj);
    }
    
    // test of "add_arg" function
    #[test]
    fn args() {
        let mut expected_test_obj: BTreeMap<String, (bool, Vec<String>)> = BTreeMap::new();
        
        expected_test_obj.insert("GOOD BYE".to_string(), (true, vec![]));
        expected_test_obj.insert("HELLO WORLD".to_string(), (true, vec![]));
        expected_test_obj.insert("h".to_string(), (false, vec![]));

        let mut argument_test_obj = Argument::new("Hello", "World", "From", "TAAP");

        argument_test_obj.add_arg("HELLO WORLD", "0", None);
        argument_test_obj.add_arg("GOOD BYE", "+", Some("Some help!"));
        let result_test_obj = argument_test_obj.parse_args(None);

        assert_eq!(expected_test_obj, result_test_obj);
    }
}

/// The struct that actually contains all the info, and acts like the container for all commands
/// needed
///
/// The Argument struct contains all info, e.g. the name, the description, all arguments added,
/// etc.
/// The Argument implementations are also what we use to create and modify our args!
///
/// An example of the Argument struct in use:
/// ```no_run
/// fn main() {
///     let mut arguments = taap::Argument::new("Name", "Description", "Epilog, text at the bottom", "Credits and year");
///     // Add some arguments and options
///     // ...
///     let parsed_args = arguments.parse_args(None);
///     // Do something with the parsed args
///     // ...
/// }
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Argument {
    name: String,
    description: String,
    exit_statuses: BTreeMap<u16, String>,
    epilog: String,
    credits: String,
    args: (
        BTreeMap<String, (String, isize)>,
        BTreeMap<char, (String, isize, String)>,
    ),
}

impl Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{name: {}, description: {}, epilog: {}, credits: {}}}",
            self.name, self.description, self.epilog, self.credits
        )
    }
}

/// Implementation for Argument struct
///
/// Code example available in te top of the documentation, and at the home page
impl Argument {
    /// Returns a new Argument instance
    ///
    /// A functon called `new` which creates and returns an instance of the Argument struct, with
    /// the values you input.
    ///
    /// Code Example:
    /// ```no_run
    /// fn main () {
    /// let mut arguments = taap::Argument::new("Name", "Description", "Epilog, text at the bottom", "Credits");
    /// // do something with arguments
    /// 
    /// }
    /// ```
    ///
    /// | Parameter   | Type | Description                                                          |
    /// |-------------|------|----------------------------------------------------------------------|
    /// | name        | &str | The name of the program                                              |
    /// | description | &str | The description of the program                                       |
    /// | epilog      | &str | The text at the bottom of the help                                   |
    /// | credits     | &str | The credits at the bottom of the help (often your name and the year) |
    ///
    pub fn new(name: &str, description: &str, epilog: &str, credits: &str) -> Self {
        let mut args: (
            BTreeMap<String, (String, isize)>,
            BTreeMap<char, (String, isize, String)>,
        ) = (BTreeMap::new(), BTreeMap::new());
        let exit_statuses: BTreeMap<u16, String> = BTreeMap::new();
        args.1.insert(
            'h',
            (
                "help".to_string(),
                0,
                "Use this to print this help message".to_string(),
            ),
        );
        Self {
            name: name.to_string(),
            description: description.to_string(),
            exit_statuses,
            epilog: epilog.to_string(),
            credits: credits.to_string(),
            args,
        }
    }

    /// Add an exit status to the help page
    ///
    /// A function that takes an u16 and a &str as input and adds it to the help page as an exit
    /// status
    ///
    /// Code Example:
    /// ```no_run
    /// fn main() {
    /// // first initialize a new Argument instance using the "new" function
    /// let mut arguments = taap::Argument::new("Name", "Description", "Epilog, text at the bottom", "Credits");
    /// // Add our exit status, first the code, then the help text
    /// arguments.add_exit_status(0, "Everything went well!");
    /// // ...
    /// }
    /// ```
    ///
    /// | Parameter | Type | Description                                            |
    /// |-----------|------|--------------------------------------------------------|
    /// | code      | u16  | The exit code                                          |
    /// | help      | &str | The help message on the help page fot that exit status |
    ///

    pub fn add_exit_status(&mut self, code: u16, help: &str) {
        self.exit_statuses.insert(code, help.to_string());
    }

    /// Add a positional argument
    ///
    /// A function that takes a placeholder &str, the amount of arguments as a &str, and a help
    /// &str of the type Option<&str>
    ///
    /// The reason for the amount of args being a &str is because it doesn't only take positive
    /// integers, it can also take "+" as an amount of arguments.
    /// The "+" is equal to an unspecified amount of arguments.
    ///
    /// The last argument is an Option<&str> because it's optional, which means you can pass None
    /// if you don't want a help text for the argument
    ///
    /// Code Example:
    /// ```no_run
    /// fn main() {
    /// // first initialize a new Argument instance using the "new" function
    /// let mut arguments = taap::Argument::new("Name", "Description", "Epilog, text at the bottom", "Credits");
    /// // Add a positonal argument to the Argument instance
    /// arguments.add_arg("BAR", "1", Some("Some Help"));
    /// // Add another positional argument, but this time it's "infinite"
    /// arguments.add_arg("FOO", "+", None);
    /// // ...
    /// }
    /// ```
    ///
    /// | Parameter   | Type         | Description                                                         |
    /// |-------------|--------------|---------------------------------------------------------------------|
    /// | placeholder | &str         | The placeholder of the positional argument, meant for the help page |
    /// | args        | &str         | The amount of arguments, can either be a positive integer or a "+"  |
    /// | help        | Option<&str> | The help text, can either be None or Some(&str)                     |
    ///
    pub fn add_arg(&mut self, placeholder: &str, args: &str, help: Option<&str>) {
        let nargs = if args == "+" {
            -1
        } else {
            match args.to_string().parse::<usize>() {
                Ok(n) => n as isize,
                Err(_) => {
                    panic!("Error! \"args\" parameter must be either a positive integer, 0 or +");
                }
            }
        };
        self.args.0.insert(
            placeholder.to_string(),
            (help.unwrap_or("").to_string(), nargs),
        );
    }

    /// Add an optional argument
    ///
    /// A function that takes a short name as a char, a long name as a &str, the amount of arguments as a &str,
    /// and a help &str of the type Option<&str>
    ///
    /// The short name can be a space (' ') or a dash ('-') if you only want a long name
    ///
    /// The long name can be an empty str (""), a space (" ") or a single/double dash ("-"/"--")
    /// if you only want a short name
    ///
    /// The reason for the amount of args being a &str is because it doesn't only take positive
    /// integers, it can also take "+" as an amount of arguments.
    /// The "+" is equal to an unspecified amount of arguments.
    ///
    /// The last argument is an Option<&str> because it's optional, which means you can pass None
    /// if you don't want a help text for the argument
    ///
    /// Code Example:
    /// ```no_run
    /// fn main() {
    /// // first initialize a new Argument instance using the "new" function
    /// let mut arguments = taap::Argument::new("Name", "Description", "Epilog, text at the bottom", "Credits");
    /// // Add some optional arguments
    /// arguments.add_option('f', "foo", "0", Some("I have a short and a long name!"));
    /// arguments.add_option('-', "boo", "2", Some("I only have a long name"));
    /// arguments.add_option('a', "-", "0", Some("I only have a short name"));
    /// arguments.add_option('n', "no-help", "0", None);
    /// 
    ///
    /// // More code...
    /// // ...
    /// }
    /// ```
    ///
    /// | Parameter | Type         | Description                                                        |
    /// |-----------|--------------|--------------------------------------------------------------------|
    /// | short     | char         | The short name of the optional argument                            |
    /// | long      | &str         | The long name of the optional argument
    /// | args      | &str         | The amount of arguments, can either be a positive integer or a "+" |
    /// | help      | Option<&str> | The help text, can either be None or Some(&str)                    |
    ///
    pub fn add_option(
        &mut self,
        mut short: char,
        mut long: &str,
        parameters: &str,
        help: Option<&str>,
    ) {
        if short == ' ' {
            short = '-'
        };
        if long.is_empty() || long == " " || long == "-" || long == "--" {
            long = ""
        };
        let nargs = if parameters == "+" {
            -1
        } else {
            match parameters.to_string().parse::<usize>() {
                Ok(n) => n as isize,
                Err(_) => {
                    panic!(
                        "Error! \"parameters\" parameter must be either a positive integer, 0 or +"
                    );
                }
            }
        };

        self.args.1.insert(
            short,
            (long.to_string(), nargs, help.unwrap_or("").to_string()),
        );
    }

    /// Prints the help page for your program
    ///
    /// Call this function to print the help page for your program.
    ///
    /// The function takes no arguments
    ///
    /// Code Example:
    /// ```no_run
    /// fn main() {
    /// // first initialize a new Argument instance using the "new" function
    /// let mut arguments = taap::Argument::new("Name", "Description", "Epilog, text at the bottom", "Credits");
    /// // Add some optional arguments
    /// arguments.add_option('f', "foo", "0", Some("I have a short and a long name!"));
    /// arguments.add_option('-', "boo", "2", Some("I only have a long name"));
    /// arguments.add_option('a', "-", "0", Some("I only have a short name"));
    /// arguments.add_option('n', "no-help", "0", None);
    ///
    /// // print the help
    /// arguments.print_help();
    /// }
    /// ```
    ///
    /// Most of the time printing the help manually is unnecessesary since the program already
    /// adds the optional argument 'h' and "help" automatically
    ///

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
        for values in pos_args.iter() {
            let argument = values.0;
            let nargs = values.1 .1;
            let help = &values.1 .0;
            usage.push_str(format!(" {}", argument).as_str());
            if nargs != 1 {
                if nargs < 0 {
                    usage.push_str("*∞");
                    pos_args_help.push_str(format!("\n    {argument}*∞\t\t\t{help}").as_str());
                } else {
                    usage.push_str(format!("*{}", nargs).as_str());
                    let tabs_needed = 3 - (nargs.to_string().len() as f32 / 8.0).ceil() as usize;
                    pos_args_help.push_str(
                        format!("\n    {argument}*{nargs}{:\t<tabs_needed$}{help}", "").as_str(),
                    );
                };
            } else {
                pos_args_help.push_str(format!("\n    {argument}\t\t\t{help}").as_str());
            };
        }

        usage.push_str(" [OPTIONS]\n");

        help_string.push_str(
            format!(
                "{}{}\n\nPositional Arguments:{}\n\nOptions:",
                usage, description, pos_args_help
            )
            .as_str(),
        );

        for field in options.iter() {
            let key: char;
            if field.0 == &'-' {
                key = ' ';
            } else {
                key = field.0.to_owned();
            };
            let values = field.1;
            let tabs_needed = if values.1 > 0 {
                2 - (values.1.to_string().len() as f32 / 8.0).ceil() as usize
            } else if values.1 < 0 {
                1
            } else {
                2
            };
            help_string.push_str(
                format!(
                    "\n    {}{}\t{}{}{}{:\t<tabs_needed$}{}",
                    if key == ' ' { "" } else { "-" },
                    key,
                    if values.0 == "" { "" } else { "--" },
                    values.0,
                    if values.1 == 0 || values.1 == 1 || values.0.is_empty() {
                        "".to_string()
                    } else if values.1 < 0 {
                        "*∞".to_string()
                    } else {
                        format!("*{}", values.1)
                    },
                    "",
                    values.2
                )
                .as_str(),
            );
        }

        if exit_statuses.len() > 1 {
            help_string.push_str("\n\nExit Statuses:");
            exit_statuses.iter().for_each(|(key, value)| {
                help_string.push_str(format!("\n    {}\t{}", key, value).as_str())
            });
        };

        help_string.push_str(format!("\n\n{}\n{}", bottom_text, credits).as_str());

        println!("{}", help_string);
    }

    /// Returns a HashMap containing the parsed arguments
    ///
    /// A function that takes an Option<Vec<String>> value, parses arguments passed to the program and
    /// returns a HashMap<String, (bool, Vec\<String\>)> which contains the parsed arguments
    ///
    ///
    /// | Parameter      | Type                | Description                                                              |
    /// |----------------|---------------------|--------------------------------------------------------------------------|
    /// | custom_arglist | Option<Vec<String>> | A custom argument-list you can use instead of the command line arguments |
    ///
    /// Code Example:
    /// ```no_run
    /// fn main() {
    /// // first initialize a new Argument instance using the "new" function
    /// let mut arguments = taap::Argument::new("Name", "Description", "Epilog, text at the bottom", "Credits");
    /// // Add a positonal argument
    /// arguments.add_arg("BAR", "0", None);
    /// // Add some optional arguments
    /// arguments.add_option('f', "foo", "0", Some("I have a short and a long name!"));
    /// arguments.add_option('-', "boo", "2", Some("I only have a long name"));
    /// arguments.add_option('a', "-", "0", Some("I only have a short name"));
    /// arguments.add_option('n', "no-help", "0", None);
    ///
    /// // Now parse the arguments and save the result in a variable
    /// let parsed_arguments = arguments.parse_args(None);
    ///
    /// // Do something with the parsed arguments
    /// // ...
    /// }
    /// ```
    ///

    pub fn parse_args(
        &mut self,
        custom_arglist: Option<Vec<String>>,
    ) -> BTreeMap<String, (bool, Vec<String>)> {
        let mut collected_raw_args: Vec<String> = std::env::args().collect();
        match custom_arglist {
            Some(val) => collected_raw_args = val,
            None => {
                collected_raw_args.remove(0);
            }
        };
        let positional_arguments = &self.args.0;
        let options = &self.args.1;
        let mut return_map: BTreeMap<String, (bool, Vec<String>)> = BTreeMap::new();
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
        }

        // handling optional arguments
        for (pos, argument) in collected_raw_args.iter().enumerate() {
            // only parse if it's over 1 character, starts with - and 2nd character isn't -
            if argument.len() > 1
                && argument.starts_with("-")
                && argument.chars().nth(1).unwrap() != '-'
            {
                // trim out the - and get characters, since options are single characters
                for part in argument.get(1..).unwrap().chars() {
                    // if it's in the hashmap, we know it exists, else just skip
                    if options.contains_key(&part) {
                        let options_needed = options.get(&part).unwrap().1;
                        // infinite args part
                        if options_needed < 0 {
                            let mut temp_infinite_arglist: Vec<String> = vec![];
                            for argument2 in collected_raw_args[pos + 1..].iter() {
                                if argument2.starts_with("-") {
                                    break;
                                };
                                if argument2.starts_with(r"\") {
                                    temp_infinite_arglist.push(argument2[1..].to_string());
                                } else {
                                    temp_infinite_arglist.push(argument2.to_owned());
                                };
                            }
                            *return_map.get_mut(&part.to_string()).unwrap() =
                                (true, temp_infinite_arglist);
                        } else {
                            // Normal args go down here
                            if collected_raw_args.len() < pos + 1 + options_needed as usize {
                                eprintln!(
                                    "Error! -{} requires {} arguments",
                                    &part, options_needed
                                );
                                exit(1);
                            };
                            *return_map.get_mut(&part.to_string()).unwrap() = (
                                true,
                                collected_raw_args[pos + 1..(pos + 1 + options_needed as usize)]
                                    .iter()
                                    .cloned()
                                    .collect(),
                            );
                        };
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
                        // infinite args handling
                        if options_needed < 0 {
                            let mut temp_infinite_arglist: Vec<String> = vec![];
                            for argument2 in collected_raw_args[pos + 1..].iter() {
                                if argument2.starts_with("-") {
                                    break;
                                };
                                if argument2.starts_with(r"\") {
                                    temp_infinite_arglist.push(argument2[1..].to_string());
                                } else {
                                    temp_infinite_arglist.push(argument2.to_owned());
                                };
                            }
                            *return_map.get_mut(&part.to_string()).unwrap() =
                                (true, temp_infinite_arglist);
                        } else {
                            // Normal args are handled HERE
                            if collected_raw_args.len() < pos + 1 + options_needed as usize {
                                eprintln!(
                                    "Error! --{} requires {} arguments",
                                    &part, options_needed
                                );
                                exit(1);
                            };
                            *return_map.get_mut(&name).unwrap() = (
                                true,
                                collected_raw_args[pos + 1..(pos + 1 + options_needed as usize)]
                                    .iter()
                                    .cloned()
                                    .collect(),
                            );
                        };
                    }
                }
            }
        }
        if return_map.get("h").unwrap().0 == true {
            self.print_help();
            exit(0);
        };

        // handling positional_arguments
        let mut current_argument_position: usize = 0;
        for (pos, (key, value)) in positional_arguments.iter().enumerate() {
            let argument_length = value.1;
            if argument_length < 0 {
                let mut temp_infinite_arglist: Vec<String> = vec![];
                for argument in collected_raw_args[pos..].iter() {
                    if argument.starts_with("-") {
                        break;
                    };
                    if argument.starts_with(r"\") {
                        temp_infinite_arglist.push(argument[1..].to_string());
                    } else {
                        temp_infinite_arglist.push(argument.to_owned());
                    };
                }
                *return_map.get_mut(key).unwrap() = (true, temp_infinite_arglist);
            } else {
                if current_argument_position + argument_length as usize > collected_raw_args.len() {
                    eprintln!(
                        "Error! {} requires {} arguments",
                        key,
                        match positional_arguments.get(key) {
                            Some(val) => val.1,
                            None => panic!("Panic! Key \"{}\" non-existant!", key),
                        }
                    );
                    exit(1);
                };
                *return_map.get_mut(key).unwrap() = (
                    true,
                    collected_raw_args[current_argument_position
                        ..current_argument_position + argument_length as usize]
                        .iter()
                        .cloned()
                        .collect(),
                );
                current_argument_position += argument_length as usize;
            }
        }

        return_map
    }
}
