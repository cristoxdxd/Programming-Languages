# Functional Programming in Python

## What is Functional Programming?

Functional programming is a programming paradigm where programs are constructed by applying and composing functions. It is a declarative programming paradigm in which function definitions are trees of expressions that each return a value, rather than a sequence of imperative statements which change the state of the program.

## Why Functional Programming?

Functional programming is a programming paradigm that treats computation as the evaluation of mathematical functions and avoids changing-state and mutable data. It is a declarative programming paradigm, which means programming is done with expressions or declarations instead of statements. In functional code, the output value of a function depends only on the arguments that are input to the function, so calling a function f twice with the same value for an argument x will produce the same result f(x) each time. Eliminating side effects, i.e. changes in state that do not depend on the function inputs, can make it much easier to understand and predict the behavior of a program, which is one of the key motivations for the development of functional programming.

Functional programming has its roots in lambda calculus, a formal system developed in the 1930s to investigate function definition, function application, and recursion. Many functional programming languages can be viewed as elaborations on the lambda calculus. Another well-known declarative programming paradigm, logic programming, is based on relations.

In contrast, imperative programming changes state with statements in the source code, the simplest example being assignment. Imperative programming languages include languages in the imperative family, such as C, C++, C#, Java, JavaScript, Perl, Python, Ruby, and PHP, as well as earlier languages such as Fortran, ALGOL 60, ALGOL 68, Pascal, PL/I, COBOL, and BASIC.

## Basic Functions

### Lambda Functions

Lambda functions are small functions usually not more than a line. It can have any number of arguments just like a normal function. The body of lambda functions is very small and consists of only one expression. The result of the expression is the value when the lambda is applied to an argument. Lambda functions are syntactically restricted to single expressions. Semantically, they are just syntactic sugar for a normal function definition. Like nested function definitions, lambda functions can reference variables from the containing scope.

```python
# Lambda function to add two numbers
add = lambda x, y: x + y
print(add(3, 5))
```

### Map Function

Map function is used to apply a function to all the elements of a sequence(list, tuple, etc.) and return a new list. It takes two arguments, first the function and second the sequence(list, tuple, etc.).

```python
# Map function to add 2 to all the elements of a list
list1 = [1, 2, 3, 4, 5]
list2 = list(map(lambda x: x + 2, list1))
print(list2)
```

### Filter Function

Filter function is used to filter out all the elements of a sequence(list, tuple, etc.) for which the function returns True. It takes two arguments, first the function and second the sequence(list, tuple, etc.).

```python
# Filter function to filter out all the even numbers from a list
list1 = [1, 2, 3, 4, 5]
list2 = list(filter(lambda x: x % 2 == 0, list1))
print(list2)
```

### Reduce Function

Reduce function is used to apply a function to all the elements of a sequence(list, tuple, etc.) and reduce it to a single value. It takes two arguments, first the function and second the sequence(list, tuple, etc.).

```python
# Reduce function to add all the elements of a list
from functools import reduce
list1 = [1, 2, 3, 4, 5]
sum = reduce(lambda x, y: x + y, list1)
print(sum)
```

### Zip Function

Zip function is used to map the similar index of multiple containers(list, tuple, etc.) so that they can be used just using as single entity. It takes two or more sequences(list, tuple, etc.) as arguments and returns a list of tuples. The first tuple contains the first element from each of the sequences, the second tuple contains the second element from each of the sequences and so on.

```python
# Zip function to map the similar index of multiple containers
list1 = [1, 2, 3, 4, 5]
list2 = [6, 7, 8, 9, 10]
list3 = list(zip(list1, list2))
print(list3)
```

### Enumerate Function

Enumerate function is used to return a list of tuples containing indices and values of a sequence(list, tuple, etc.). It takes a sequence(list, tuple, etc.) as argument and returns a list of tuples.

```python
# Enumerate function to return a list of tuples containing indices and values of a list
list1 = [1, 2, 3, 4, 5]
list2 = list(enumerate(list1))
print(list2)
```

### Sorted Function

Sorted function is used to sort the elements of a sequence(list, tuple, etc.) in a specific order(ascending or descending) and return a new list. It takes two arguments, first the sequence(list, tuple, etc.) and second the order(ascending or descending).

```python
# Sorted function to sort the elements of a list in ascending order
list1 = [5, 4, 3, 2, 1]
list2 = sorted(list1)
print(list2)
```

### Any Function

Any function is used to check if any of the elements of a sequence(list, tuple, etc.) is True. It takes a sequence(list, tuple, etc.) as argument and returns True if any of the elements of the sequence is True.

```python
# Any function to check if any of the elements of a list is True
list1 = [True, False, False]
print(any(list1))
```

### All Function

All function is used to check if all the elements of a sequence(list, tuple, etc.) are True. It takes a sequence(list, tuple, etc.) as argument and returns True if all the elements of the sequence are True.

```python
# All function to check if all the elements of a list are True
list1 = [True, True, True]
print(all(list1))
```

### Max Function

Max function is used to return the maximum element of a sequence(list, tuple, etc.). It takes a sequence(list, tuple, etc.) as argument and returns the maximum element of the sequence.

```python
# Max function to return the maximum element of a list
list1 = [1, 2, 3, 4, 5]
print(max(list1))
```

### Min Function

Min function is used to return the minimum element of a sequence(list, tuple, etc.). It takes a sequence(list, tuple, etc.) as argument and returns the minimum element of the sequence.

```python
# Min function to return the minimum element of a list
list1 = [1, 2, 3, 4, 5]
print(min(list1))
```

### Sum Function

Sum function is used to return the sum of all the elements of a sequence(list, tuple, etc.). It takes a sequence(list, tuple, etc.) as argument and returns the sum of all the elements of the sequence.

```python
# Sum function to return the sum of all the elements of a list
list1 = [1, 2, 3, 4, 5]
print(sum(list1))
```

### Range Function

Range function is used to return a sequence of numbers. It takes three arguments, first the starting point, second the ending point and third the step size. It returns a sequence of numbers starting from the starting point, ending at the ending point and incrementing by the step size.

```python
# Range function to return a sequence of numbers
list1 = list(range(1, 6, 1))
print(list1)
```

### Reversed Function

Reversed function is used to return a reversed iterator. It takes a sequence(list, tuple, etc.) as argument and returns a reversed iterator.

```python
# Reversed function to return a reversed iterator
list1 = [1, 2, 3, 4, 5]
list2 = list(reversed(list1))
print(list2)
```

### Exec Function

Exec function is used to execute a dynamically created program, which can be created using a string. It takes a string as argument and executes it.

```python
# Exec function to execute a dynamically created program
string = "print('Hello World')"
exec(string)
```

### Eval Function

Eval function is used to evaluate a dynamically created program, which can be created using a string. It takes a string as argument and evaluates it.

```python
# Eval function to evaluate a dynamically created program
string = "print('Hello World')"
eval(string)
```

## Higher Order Functions

Higher order functions are functions that take other functions as arguments or return functions as their results. In other words, we can say that a higher order function is a “function that operates on other functions by taking them as arguments or returning them”.

```python
# Higher order function to add 2 to all the elements of a list
def add2(x):
    return x + 2
def my_map(func, list1):
    list2 = []
    for item in list1:
        list2.append(func(item))
    return list2
list1 = [1, 2, 3, 4, 5]
list2 = my_map(add2, list1)
print(list2)
```

## Decorators

Decorators are functions that take another function as an argument, add some functionality and return another function, without altering the source code of the original function that we passed in. In other words, we can say that a decorator is a “function that takes a function and returns a closure”.

```python
# Decorator to add 2 to all the elements of a list
def add2(func):
    def wrapper(list1):
        list2 = []
        for item in list1:
            list2.append(func(item))
        return list2
    return wrapper
@add2
def add(x):
    return x + 2
list1 = [1, 2, 3, 4, 5]
list2 = add(list1)
print(list2)
```

## Closures

A closure is a function that remembers the values from the enclosing lexical scope even when the program flow is no longer in that scope. In other words, we can say that a closure is a “function that remembers the values from the enclosing lexical scope even when the program flow is no longer in that scope”.

```python
# Closure to add 2 to all the elements of a list
def add2():
    def wrapper(list1):
        list2 = []
        for item in list1:
            list2.append(item + 2)
        return list2
    return wrapper
list1 = [1, 2, 3, 4, 5]
add = add2()
list2 = add(list1)
print(list2)
```

## Currying

Currying is the process of transforming a function that takes multiple arguments into a function that takes a single argument and returns another function. In other words, we can say that currying is a “technique of translating the evaluation of a function that takes multiple arguments into evaluating a sequence of functions, each with a single argument”.

```python
# Currying to add 2 to all the elements of a list
def add2(x):
    def wrapper(y):
        return x + y
    return wrapper
list1 = [1, 2, 3, 4, 5]
list2 = []
for item in list1:
    add = add2(item)
    list2.append(add(2))
print(list2)
```

## Partial Functions

Partial functions are functions that derive from other functions with some arguments already supplied. In other words, we can say that a partial function is a “function that derives from another function with some arguments already supplied”.

```python
# Partial function to add 2 to all the elements of a list
from functools import partial
def add(x, y):
    return x + y
add2 = partial(add, 2)
list1 = [1, 2, 3, 4, 5]
list2 = []
for item in list1:
    list2.append(add2(item))
print(list2)
```

## Memoization

Memoization is an optimization technique used primarily to speed up computer programs by storing the results of expensive function calls and returning the cached result when the same inputs occur again. In other words, we can say that memoization is a “technique of storing the results of expensive function calls and returning the cached result when the same inputs occur again”.

```python
# Memoization to add 2 to all the elements of a list
def memoize(func):
    cache = {}
    def wrapper(x):
        if x not in cache:
            cache[x] = func(x)
        return cache[x]
    return wrapper
@memoize
def add(x):
    return x + 2
list1 = [1, 2, 3, 4, 5]
list2 = []
for item in list1:
    list2.append(add(item))
print(list2)
```

## Generators

Generators are functions that return an iterable generator object. In other words, we can say that a generator is a “function that returns an iterable generator object”.

```python
# Generator to add 2 to all the elements of a list
def add2(list1):
    for item in list1:
        yield item + 2
list1 = [1, 2, 3, 4, 5]
list2 = list(add2(list1))
print(list2)
```

## Iterators

Iterators are objects that can be iterated upon. In other words, we can say that an iterator is a “object that can be iterated upon”.

```python
# Iterator to add 2 to all the elements of a list
class Add2:
    def __init__(self, list1):
        self.list1 = list1
        self.index = 0
    def __iter__(self):
        return self
    def __next__(self):
        if self.index < len(self.list1):
            item = self.list1[self.index]
            self.index += 1
            return item + 2
        else:
            raise StopIteration
list1 = [1, 2, 3, 4, 5]
list2 = list(Add2(list1))
print(list2)
```

## Coroutines

Coroutines are computer program components that generalize subroutines for non-preemptive multitasking, by allowing multiple entry points for suspending and resuming execution at certain locations. In other words, we can say that a coroutine is a “computer program component that generalize subroutines for non-preemptive multitasking, by allowing multiple entry points for suspending and resuming execution at certain locations”.

```python
# Coroutine to add 2 to all the elements of a list
def add2():
    list1 = yield
    list2 = []
    for item in list1:
        list2.append(item + 2)
    return list2
list1 = [1, 2, 3, 4, 5]
add = add2()
next(add)
list2 = add.send(list1)
print(list2)
```

## Asynchronous Functions

Asynchronous functions are functions that are executed asynchronously via the event loop. In other words, we can say that an asynchronous function is a “function that is executed asynchronously via the event loop”.

```python
# Asynchronous function to add 2 to all the elements of a list
import asyncio
async def add2(list1):
    list2 = []
    for item in list1:
        list2.append(item + 2)
    return list2
async def main():
    list1 = [1, 2, 3, 4, 5]
    list2 = await add2(list1)
    print(list2)
asyncio.run(main())
```

## Asynchronous Generators

Asynchronous generators are generators that are executed asynchronously via the event loop. In other words, we can say that an asynchronous generator is a “generator that is executed asynchronously via the event loop”.

```python
# Asynchronous generator to add 2 to all the elements of a list
import asyncio
async def add2(list1):
    for item in list1:
        yield item + 2
async def main():
    list1 = [1, 2, 3, 4, 5]
    list2 = []
    async for item in add2(list1):
        list2.append(item)
    print(list2)
asyncio.run(main())
```

## Asynchronous Iterators

Asynchronous iterators are iterators that are executed asynchronously via the event loop. In other words, we can say that an asynchronous iterator is a “iterator that is executed asynchronously via the event loop”.

```python
# Asynchronous iterator to add 2 to all the elements of a list
import asyncio
class Add2:
    def __init__(self, list1):
        self.list1 = list1
        self.index = 0
    def __aiter__(self):
        return self
    async def __anext__(self):
        if self.index < len(self.list1):
            item = self.list1[self.index]
            self.index += 1
            return item + 2
        else:
            raise StopAsyncIteration
async def main():
    list1 = [1, 2, 3, 4, 5]
    list2 = []
    async for item in Add2(list1):
        list2.append(item)
    print(list2)
asyncio.run(main())
```
