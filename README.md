# TAAP
> Argument parsing - made easy

![Crates.io Total Downloads](https://img.shields.io/crates/d/taap?style=plastic&label=Total%20Downloads)
![Crates.io Version](https://img.shields.io/crates/v/taap?style=plastic)
![docs.rs](https://img.shields.io/docsrs/taap?style=plastic&label=docs.rs%20status)
![Crates.io License](https://img.shields.io/crates/l/taap?style=plastic)

# What is TAAP?
TAAP is an argument parser made for rust, with ease of use in mind!

(TAAP is short for "totally acceptable argument parser")

This crate provides the Argument struct, which has a couple of implementations which you use to
create and parse args.
To get started, please take a look at the example down below, which uses this crate to create a
simple program with arguments!

(*PSST! All this, and more is available over at the [docs.rs page](https://docs.rs/taap/latest/taap/)*)

# Quick Start
## Adding to your project
To include the crate in your project, either add the following lines to your `Cargo.toml` file:
```toml
[dependencies]
taap = "0.1.4"
```

or run this command in your cargo project directory:
```
cargo add taap
```

When you've added that, you're ready to use TAAP!

## Example Usage
In the following codeblock, I'll cover how to add:
- a positional argument
- an optional argument
- some exit statuses
- how to parse the args (and make use of them)
(This example/codeblock is also available in the [examples
folder](https://github.com/SpamixOfficial/taap-rs/tree/master/examples) 
```rust
fn main() {
    // Next, in the main function, create a MUTABLE variable using Argument::new()
    let mut arguments = taap::Argument::new("example-1", "The first example program for TAAP!", "The text at the bottom of the help!", "SpamixOfficial 2023");


    // Now we will add our first positional argument!
    // First we add our letter we want to use, in this case 'f'
    // Next we add our long name we want to use, in this case "foo"
    // To add an argument without a "short name", set the short name to '-' or ' '
    arguments.add_option('f', "foo", "0", Some("Some help!"));
    arguments.add_option('-', "no-help", "2", None);


    // Here I'll add a positonal argument. Positional arguments takes almost the same parameters as
    // optional arguments, except that it doesn't take a "long name" and a "short name". Instead it
    // just takes a placeholder 
    arguments.add_arg("BAR", "1", None);

    // Now let's also add some exit statuses! 
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
    // If we want to use a custom argument-list we can pass it using Some(Vec<String>)
    // In this case we don't want to do that, so we pass None
    let parsed_arguments = arguments.parse_args(None);
    

    // Now let's use our arguments!
    // First, let's grab our first positional argument, named "BAR"
    let bar = parsed_arguments.get("BAR").unwrap();
    println!("BAR is: {}", bar.0);

    // Next, let's see if our optional argument foo was used
    //
    // The process here is the same as the previous lines
    // If your optional argument take no arguments, then your Vec<String> will be an empty vector
    let foo = parsed_arguments.get("f").unwrap();
    was_foo_used(foo);

    // Finally, let's see how we would handle an output with values!
    
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
```

If you want more "in-depth" explanations and comments, please see the first example in the example folder over in the github repository

## Using your software

Now let's run our program!
```text 
[user@the_machine taap-rs]$ ./example-1
Error! BAR requires 1 arguments,
```
We supplied no arguments, which resulted in this output!

Now let's actually supply the arguments.
To do this maybe we want to take a look at the help first!

```text
[user@the_machine taap-rs]$ ./example-1 -h
Usage: example-1 BAR [OPTIONS]
The first example program for TAAP!

Positional Arguments:
    BAR		

Options:
    -h	--help		Use this to print this help message
    -f	--foo		Some help!
     	--no-help*2		

Exit Statuses:
    0	Everything went just fine
    1	Something went a little wrong
    2	Something went horribly wrong!

The text at the bottom of the help!
SpamixOfficial 2023
```
Hmmm, we didn't define a help argument though? Well, as you see from the output above, TAAP got
that covered for us! 

When we add arguments, it also automatically adds it to the help!
If you also want to print the help yourself, you can call the print_help() function!

Now let's run the program, with the right arguments!

```text
[user@the_machine taap-rs]$ ./example-1 "I am BAR" -f Im_foo --no-help "I take" "Two arguments!"
BAR is: true
Foo was used!
--no-help was used with arguments:
I take
Two arguments! 
```

As you see, all the arguments got parsed and used correctly!

## Extra info

If one of the arguments would have had an unspecified amount of arguments
(an infinite amount), we would have had to terminate it using -.

That means, that if BAR had an infinite amount of arguments
it would have been terminated by the -f option, since the character - terminates
the infinite argument.

If we still wanted to use a -, we would have to escape it using \\ .

NOTE: Some shells actually uses \ as an escape character, which means you would
have to escape the escape character (\\\\).

## Final words

You should now be ready to use TAAP!

If you want to read more about TAAP, there's more documentation on docs.rs

If you want to look at more examples, take a look at the [examples
folder](https://github.com/SpamixOfficial/taap-rs/tree/master/examples) in this repository 


# Credits
See CREDITS.md for credits
