# Chapter 3 

## mutability 
- variables are immutable by default
  - can be changed by adding mut to the declaration of the variable. 

## Constants 
- not allowed to be mut 
- used by prefecing with `const` keyword. 
- can only be set to a constant expression, not the result of a value from run time. 
- Constants are valid for the entire time a program runs, within the scope in which they were declared.

## Shadowing 

- you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

- Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. 
- 