# Chapter 3 

### mutability 
- variables are immutable by default
  - can be changed by adding mut to the declaration of the variable. 

### Constants 
- not allowed to be mut 
- used by prefecing with `const` keyword. 
- can only be set to a constant expression, not the result of a value from run time. 
- Constants are valid for the entire time a program runs, within the scope in which they were declared.

### Shadowing 

- you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

- Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. 

## Data Types
- Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.

### Scalar 
- A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 

#### Integer Types
An integer is a number without a fractional component.

![alt text](image.png)
 
the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.


#### Floating Point Types
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively.
  

### Compound
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

#### Tuples
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

- elements of a tuple can be accessed by using the name and index EX: `tup.index`

#### Array

- every element of an array must have the same type. 
- Unlike arrays in some other languages, arrays in Rust have a fixed length.
- define an array as so: `let a = [1, 2, 3, 4, 5];`
- if you need more flexability you should use a vector instead of an array. 
- acess a single element `arr[index]`


## Functions
the `main` function, which is the entry point of many programs. You’ve also seen the `fn` keyword, which allows you to declare new functions.

### Parameters
We can define functions to have parameters, which are special variables that are part of a function’s signature.
- When a function has parameters, you can provide it with concrete values for those parameters
- you must declare the type of each parameter

### Statements and Expressions 
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.

### Return Values
Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow `->`.
 - In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly.

## Comments
- use `//` for a single line comment 

## Control Flow
The ability to run some code depending on whether a condition is `true` and to run some code repeatedly while a condition is `true` are basic building blocks in most programming languages

### if statments 
An `if` expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”
- if statments always expect a bool 
- You can use multiple conditions by combining `if` and `else` in an `else if` expression.

### Loop
The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
- You might also need to pass the result of that operation out of the `loop` to the rest of your code. To do this, you can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so you can use it
- If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.

### While Loop 
A program will often need to evaluate a condition within a loop. While the condition is `true`, the loop runs. When the condition ceases to be true, the program calls `break`, stopping the loop.

- As a more concise alternative, you can use a `for` loop and execute some code for each item in a collection.