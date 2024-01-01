//! TAAP is an argument parser made for rust, with ease of use in mind!
//!
//!(TAAP is short for "totally acceptable argument parser")
//!
//! This crate provides the Argument struct, which has a couple of implementations which you use to
//! create and parse args.
//! To get started, please take a look at the example down below, which uses this crate to create a
//! simple program with arguments!
//!
//! # Adding to your project
//! To include the crate in your project, add the following lines to your `Cargo.toml` file:
//! ```
//![dependencies]
//!taap = "0.1.0"
//! ```
//! When you've added that, you're ready to use TAAP!
//!
//! # Example Usage
//! In the following codeblock, I'll cover how to add:
//! - a positional argument
//! - an optional argument
//! - some exit statuses
//! - how to parse the args (and make use of them)
//! (This example/codeblock is also available in the [examples
//! folder](https://github.com/SpamixOfficial/taap-rs/examples) in the [github
//! repository](https://github.com/SpamixOfficial/taap-rs))
//! ```rust
//! // First, import taap so we can use it
//! use taap;
//!
//! fn main() {
//!     // Next, in the main function, create a MUTABLE variable with a fitting name, like arguments!
//!     // It is very important the variable is mutable because we will need to modify values in it
//!     // later!
//!     //
//!     // You create a new "Argument" instance using Argument::new. The "new" argument takes 4 values:
//!     // * The name of the program (the executable name)
//!     // * The description of the program
//!     // * The epilog, also known as the text at the bottom of the help
//!     // * The credits, usually your own name (or the owner's name) and the year
//!     let mut arguments = taap::Argument::new("example-1", "The first example program for TAAP!", "The text at the bottom of the help!", "SpamixOfficial 2023");
//!
//!
//!     // Now we will add our first positional argument!
//!     // First we add our letter we want to use, in this case 'f', a char
//!     //
//!     // Next we add our long name we want to use, in this case "foo", a &str
//!     //
//!     // After that we add the amount of arguments the option takes. The parameter can either be:
//!     // * "0", for 0 arguments,
//!     // * A positive integer, for another amount of arguments
//!     // * A "+" for an unspecified amount of arguments
//!     // (NOTE: All this needs to be &str)
//!     //
//!     // Last we add our our help for the argument, which is contained in a "Some". If you don't want
//!     // to specify any help, simply set it to "None", as shown when adding the argument 'no-help'
//!     //
//!     // To add an argument without a "short name", set the short name to '-' or ' '
//!     arguments.add_option('f', "foo", "0", Some("Some help!"));
//!     arguments.add_option('-', "no-help", "2", None);
//!
//!
//!     // Here I'll add a positonal argument. Positional arguments takes almost the same parameters as
//!     // optional arguments, except that it doesn't take a "long name" and a "short name". Instead it
//!     // just takes a placeholder!
//!     //
//!     // First I'll add my placeholder name, which should be a &str
//!     //
//!     // Next I'll add the amount of arguments the option takes, which is the same as optional
//!     // arguments
//!     //
//!     // Lastly I'll add my help, which is the same as optional arguments
//!     arguments.add_arg("BAR", "1", None);
//!
//!     // Now let's also add some exit statuses!
//!     //
//!     // Adding exit statuses is very useful for the end user, since if something goes wrong the user
//!     // will know what the code means!
//!     //
//!     // First, we add our exit code, which should be a u16
//!     //
//!     // Next we add our help, which in this case is NOT optional!
//!     arguments.add_exit_status(0, "Everything went just fine");
//!     arguments.add_exit_status(1, "Something went a little wrong");
//!     arguments.add_exit_status(2, "Something went horribly wrong!");
//!
//!     // Finally, let's parse the args and make use of them!
//!     //
//!     // We parse the args by calling parse_args(), which returns a HashMap
//!     // I'll explain what every part means in a second!
//!     //
//!     // When we have parsed our args, we also want to save the result!
//!     // To do this we create a new variable, and contain our parsed args in that variable
//!     let parsed_arguments = arguments.parse_args();
//!
//!     // Now let's use our arguments!
//!     // First, let's grab our first positional argument, named "BAR"
//!     //
//!     // To grab an argument, we simply just get it from our hashmap using the name we originally
//!     // defined for it.
//!     // If it's an optional argument we use the short name, and if it's a positonal argument we use
//!     // the placeholder name
//!     // If it's an optional argument without a short name we instead use the long name, "no-help"
//!     // for example!
//!     //
//!     // the .get() returns a result, which we can't use. To actually get the values we use .unwrap().
//!     // This returns our values, which is a tuple that contains a:
//!     // * boolean, which tells us if the argument was used or not (positional arguments always
//!     // returns true)
//!     // * A vector of strings, which are the command line arguments that were passed
//!     //
//!     // Let's also print bar's value!
//!     let bar = parsed_arguments.get("BAR").unwrap();
//!     println!("BAR is: {}", bar.0);
//!
//!     // Next, let's see if our optional argument foo was used
//!     //
//!     // The process here is the same as the previous lines
//!     //
//!     // Let's also pass our gathered info to a function.
//!     // The function needs to take &(bool, Vec) as input since foo is a borrow and foo
//!     // consists of (bool, Vec)
//!     //
//!     // If your optional argument take no arguments, then your Vec will be an empty vector
//!     let foo = parsed_arguments.get("f").unwrap();
//!     was_foo_used(foo);
//!
//!     // Finally, let's see how we would handle an output with values!
//!     //
//!     // First, we once again store the output of our parameter, in this case "no-help"
//!     //
//!     // Next, we check if "no-help" was used. If "no-help" was used, we print that it was used!
//!     //  
//!     // Now we print what values "no-help" was used with. We do that by looping over the vector,
//!     // which is the second item in the tuple (index 1)
//!     //
//!     // Why we use .iter() on no_help.1, is because the vector is behind a shared reference
//!     
//!     let no_help = parsed_arguments.get("no-help").unwrap();
//!     if no_help.0 {
//!         println!("--no-help was used with arguments:");
//!         for argument in no_help.1.iter() {
//!             println!("{}", argument);
//!         };
//!     } else {
//!         println!("--no--help was not used!");
//!     };
//! }
//!
//! fn was_foo_used(info: &(bool, Vec)) {
//!     // As you see here, we use info.0 to retreive if foo was used or not, which is a boolean value
//!     if info.0 {
//!         println!("Foo was used!");
//!     } else {
//!         println!("Foo was not used!");
//!     };
//! }
//! ```
//!
//! Now let's run our program!
//! ```text 
//! [user@the_machine taap-rs]$ ./example-1
//! Error! Too few arguments supplied to positional argument BAR 
//! ```
//! We supplied no arguments, which resulted in this output!
//!
//! Now let's actually supply the arguments.
//! To do this maybe we want to take a look at the help first!
//!
//! ```text
//! [user@the_machine taap-rs]$ ./example-1 -h
//! Usage: example-1 BAR [OPTIONS]
//! The first example program for TAAP!
//! 
//! Positional Arguments:
//!     BAR		
//! 
//! Options:
//!     -h	--help		Use this to print this help message
//!     -f	--foo		Some help!
//!      	--no-help*2		
//! 
//! Exit Statuses:
//!     0	Everything went just fine
//!     1	Something went a little wrong
//!     2	Something went horribly wrong!
//! 
//! The text at the bottom of the help!
//! SpamixOfficial 2023
//! ```
//! Hmmm, we didn't define a help argument though? Well, as you see from the output above, TAAP got
//! that covered for us! 
//!
//! When we add arguments, it also automatically adds it to the help!
//! If you also want to print the help yourself, you can call the print_help() function!
//!
//! Now let's run the program, with the right arguments!
//! 
//! ```text
//! [alexander@endeavouros-9470m taap-rs]$ ./example-1 "I am BAR" -f Im_foo --no-help "I take" "Two arguments!"
//! BAR is: true
//! Foo was used!
//! --no-help was used with arguments:
//! I take
//! Two arguments! 
//! ```
//!
//! As you see, all the arguments got parsed and used correctly!
//!
//! You should now be ready to use TAAP!
//!
//! If you want to read more about TAAP, there's more documentation on this page
//!
//! If you want to look at more examples, take a look at the [examples
//! folder](https://github.com/SpamixOfficial/taap-rs/examples) in the [github
//! repository](https://github.com/SpamixOfficial/taap-rs) 

use std::{collections::HashMap, process::exit, str};

/// Doc
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
                "{}{}\n\nPositional Arguments:{}\n\nOptions:",
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
                    "\n    {}{}{}\t--{}{}\t\t{}",
                    if key == ' ' { "" } else { "-" },
                    key,
                    if values.1 <= 1 || key == ' ' { "".to_string() } else { format!("*{}", values.1) },
                    values.0,
                    if values.1 <= 1 || values.0.is_empty() { "".to_string() } else { format!("*{}", values.1) }, 
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
        }
        if return_map.get("h").unwrap().0 == true {
            self.print_help();
            exit(0);
        };

        // handling positional_arguments
        let mut current_argument_position: usize = 0;
        for argument in positional_arguments_order {
            let argument_length = positional_arguments.get(argument).unwrap().1 as usize;
            if current_argument_position + argument_length > collected_raw_args.len() {
                eprintln!(
                    "Error! Too few arguments supplied to positional argument {}",
                    argument
                );
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
        }

        return_map
    }
}
