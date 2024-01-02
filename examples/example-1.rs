// First, import taap so we can use it
use taap;

fn main() {
    // Next, in the main function, create a MUTABLE variable with a fitting name, like arguments!
    // It is very important the variable is mutable because we will need to modify values in it
    // later!
    //
    // You create a new "Argument" instance using Argument::new. The "new" argument takes 4 values:
    // * The name of the program (the executable name)
    // * The description of the program
    // * The epilog, also known as the text at the bottom of the help
    // * The credits, usually your own name (or the owner's name) and the year
    let mut arguments = taap::Argument::new("example-1", "The first example program for TAAP!", "The text at the bottom of the help!", "SpamixOfficial 2023");


    // Now we will add our first positional argument!
    // First we add our letter we want to use, in this case 'f', a char
    // 
    // Next we add our long name we want to use, in this case "foo", a &str
    // 
    // After that we add the amount of arguments the option takes. The parameter can either be:
    // * "0", for 0 arguments,
    // * A positive integer, for another amount of arguments
    // * A "+" for an unspecified amount of arguments, aka infinite arguments
    // (NOTE: All this needs to be &str)
    //
    // Last we add our our help for the argument, which is contained in a "Some". If you don't want
    // to specify any help, simply set it to "None", as shown when adding the argument 'no-help'
    //
    // To add an argument without a "short name", set the short name to '-' or ' '
    arguments.add_option('f', "foo", "0", Some("Some help!"));
    arguments.add_option('-', "no-help", "2", None);


    // Here I'll add a positonal argument. Positional arguments takes almost the same parameters as
    // optional arguments, except that it doesn't take a "long name" and a "short name". Instead it
    // just takes a placeholder!
    //
    // First I'll add my placeholder name, which should be a &str 
    // 
    // Next I'll add the amount of arguments the option takes, which is the same as optional
    // arguments
    //
    // Lastly I'll add my help, which is the same as optional arguments
    arguments.add_arg("BAR", "1", None);

    // Now let's also add some exit statuses!
    // 
    // Adding exit statuses is very useful for the end user, since if something goes wrong the user
    // will know what the code means!
    //
    // First, we add our exit code, which should be a u16
    //
    // Next we add our help, which in this case is NOT optional!
    arguments.add_exit_status(0, "Everything went just fine");
    arguments.add_exit_status(1, "Something went a little wrong");
    arguments.add_exit_status(2, "Something went horribly wrong!");

    // Finally, let's parse the args and make use of them!
    //
    // We parse the args by calling parse_args(), which returns a HashMap<String, (bool, String)>
    // I'll explain what every part means in a second!
    //
    // When we have parsed our args, we also want to save the result! 
    // To do this we create a new variable, and contain our parsed args in that variable 
    let parsed_arguments = arguments.parse_args();

    // Now let's use our arguments!
    // First, let's grab our first positional argument, named "BAR"
    // 
    // To grab an argument, we simply just get it from our hashmap using the name we originally
    // defined for it. 
    // If it's an optional argument we use the short name, and if it's a positonal argument we use
    // the placeholder name
    // If it's an optional argument without a short name we instead use the long name, "no-help"
    // for example!
    //
    // the .get() returns a result, which we can't use. To actually get the values we use .unwrap().
    // This returns our values, which is a tuple that contains a:
    // * boolean, which tells us if the argument was used or not (positional arguments always
    // returns true)
    // * A vector of strings, which are the command line arguments that were passed
    //
    // Let's also print bar's value!
    let bar = parsed_arguments.get("BAR").unwrap();
    println!("BAR is: {}", bar.0);

    // Next, let's see if our optional argument foo was used
    //
    // The process here is the same as the previous lines
    //
    // Let's also pass our gathered info to a function.
    // The function needs to take &(bool, Vec<String>) as input since foo is a borrow and foo
    // consists of (bool, Vec<String>)
    //
    // If your optional argument take no arguments, then your Vec<String> will be an empty vector
    let foo = parsed_arguments.get("f").unwrap();
    was_foo_used(foo);

    // Finally, let's see how we would handle an output with values!
    //
    // First, we once again store the output of our parameter, in this case "no-help"
    //
    // Next, we check if "no-help" was used. If "no-help" was used, we print that it was used!
    //  
    // Now we print what values "no-help" was used with. We do that by looping over the vector,
    // which is the second item in the tuple (index 1)
    //
    // Why we use .iter() on no_help.1, is because the vector is behind a shared reference
    
    let no_help = parsed_arguments.get("no-help").unwrap();
    if no_help.0 {
        println!("--no-help was used with arguments:");
        for argument in no_help.1.iter() {
            println!("{}", argument);
        };
    } else {
        println!("--no--help was not used!");
    };
}

fn was_foo_used(info: &(bool, Vec<String>)) {
    // As you see here, we use info.0 to retreive if foo was used or not, which is a boolean value
    if info.0 {
        println!("Foo was used!");
    } else {
        println!("Foo was not used!");
    };
}
