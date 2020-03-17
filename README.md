# Blinmaker 2.1.0

Rust-based library for calculating amount of blins(pencakes) you can make using given amount of materials(eggs, milk, flour)

# Getting Started

This instructions will get you through steps, needed to start including and using this library in your projects and will get you through features and functionality of this library

## Prequisities

For using this library you'll need configured Rust development environment, which includes Rust compiler(rustc) and project management system(cargo)

### Integrating                                                                                                                                                                                                                                         
Place required dependency record(which can be found below) in your Cargo.toml file in the directory with your project.
    
```
blinmaker = {git="https://github.com/ilchub/blinmaker2.git"}
```
Next thing you'll required to do, it's to include it in your code. For this, use this syntax:
```
#include "blinmaker.h"
```
### Testing

To check if everything is working correctly, please run
```
cargo build
```
## Usage

So, from now, let's cycle through basic functionality of the framework. The main idea of the library defines itself in getting pre-defined param values, and then, based on collected data, give you result, passed through math formula. Each function of the library will require some input parameters. All of them is described below.

### Measurment Requirements

Flour is defined in grams

Milk is defined in mililiters

### Code Requirements

Library operates with two types of variables: i32 and f32. Due to Rust strict basic data types standarts, compiler will report an error when you trying to pass an argument to the function without exact match of data types which function require and which you are trying to pass, so please consider converting all integer types to i32 and all float types to f32. Author thought that you won't require such big mantissa that f64 offers, because you won't be able to measure flour and milk so precicely by the lack of so precicive scales and measurment storage, thats why f32 is being used for storing flour and milk.
### Function name formation

General idea of library syntax, is based on some keywords. Different combinations may form different functionality. List of all keywords is given below:
```
Actions: {
get(coming soon)
set(coming soon)
find
}
Objects: {
flour(coming soon)
milk(coming soon)
eggs(coming soon)
blin
materials
}
Result {
amount
}
```
Let me define some examples:
```
set_flour_amount()
```
will define the amount of flour that you posess
```
find_blin_amount()
```
will find amount of blins you can make, based on materials quantity that you have set

| WARNING: Not all words can be combined. Learn more below |
| --- |
### Functions

#### find_blin_amount(f32 flour_amount, f32 milk_amount, i32 eggs_amount)
Returns the amount of blins you can make, from materials you have. Accepts three parameters: flour amount, milk amount, eggs amount.

| WARNING: Make sure you pass function arguments in the right order. Missplacing them may result in a wrong function result|
| --- |

#### find_materials_amount(f32 flour_amount, f32 milk_amount, i32 eggs_Amount)
Returns exact amount of materials that you used to make those number of blins you got using FindBlinAmount(). Useful in those situations when you have too many amount of one or more of the ingredients, and you sure that you won't use all of it due to the lack of other ingredients. Function will show you, how much of redundant ingredient you will use for blins preparation.
Pay attention to one moment. Function returns giant amount of arguments(3). Thats why, they should be stored in tuple. Make sure that you store them in tuple but not in array, because value types that function returns, differs from each other. Examples below:
```
let eggs_amount: i32;
let milk_amount: f32;
let flour_amount: f32;
let requirements: (f32,f32,i32) = find_materials_amount(flour_amount, milk_amount, eggs_amount);
println!("You will need {} eggs", requirements.3);
println!("You will need {} grams of flour", requirements.1);
println!("You will need {} mililiters of milk", requirements.2);
```
### Constants

Library includes 3 constants which were primarily designed to perform checks of materials amount you posess. They define minimal amount of materials required to be capable of baking one portion of them(6 blins)

```
EGGS_MIN: i32 = 1;                                                                                                                                                                                     
FLOUR_MIN: f32 = 100.0;
MILK_MIN: f32 = 200.0;
```
See "Examples" to find out how to use them
### Examples

Please consider looking inside [blinmaker-cli]() repository. This is ready to use program which was built with usage of this library. To compile and run this program, please run:
```
cargo build
cargo run
```
