# Python syntax and semantics

<details>

<summary> Comments </summary>

Comments are used to explain code and are ignored by the Python interpreter. Comments can be written in two ways:

```python
# This is a comment
```

```python
"""
This is a 
multiple lines 
comment...
"""
```

</details>

<details>

<summary> Variables </summary>

Variables are used to store data. Variables can be declared and assigned values in the following way:

```python
x = 5
```

Variables can also be declared and assigned values in the following way:

```python
x: int = 5
```

</details>

<details>

<summary> Data types </summary>

All in Python is a class. You could use the `type()` function to look for the following data types in Python:

- `int` - integer
- `float` - floating-point number
- `complex` - complex number
- `str` - string
- `bool` - boolean
- `list` - list
- `tuple` - tuple
- `range` - range
- `dict` - dictionary
- `set` - set
- `frozenset` - frozen set
- `bytes` - bytes
- `bytearray` - byte array
- `memoryview` - memory view

```python
x = 5
print(type(x)) # <class 'int'>
```

</details>

<details>

<summary> Type casting </summary>

Type casting is used to convert a variable from one data type to another data type. Type casting can be done in the following way:

```python
x = int(5) # x will be 5
```

</details>

<details>

<summary> Operators </summary>

Python has the following operators:

- [Arithmetic operators](#arithmetic-operators)
- [Assignment operators](#assignment-operators)
- [Comparison operators](#comparison-operators)
- [Logical operators](#logical-operators)
- [Identity operators](#identity-operators)
- [Membership operators](#membership-operators)
- [Bitwise operators](#bitwise-operators)

### Arithmetic operators

Arithmetic operators are used to perform arithmetic operations on variables and values. Python has the following arithmetic operators:

| Operator | Description | Example |
| --- | --- | --- |
| `+` | Addition | `x + y` |
| `-` | Subtraction | `x - y` |
| `*` | Multiplication | `x * y` |
| `/` | Division | `x / y` |
| `%` | Modulus | `x % y` |
| `**` | Exponentiation | `x ** y` |
| `//` | Floor division | `x // y` |

### Assignment operators

Assignment operators are used to assign values to variables. Python has the following assignment operators:

| Operator | Example | Same as |
| --- | --- | --- |
| `=` | `x = 5` | `x = 5` |
| `+=` | `x += 5` | `x = x + 5` |
| `-=` | `x -= 5` | `x = x - 5` |
| `*=` | `x *= 5` | `x = x * 5` |
| `/=` | `x /= 5` | `x = x / 5` |
| `%=` | `x %= 5` | `x = x % 5` |
| `**=` | `x **= 5` | `x = x ** 5` |
| `//=` | `x //= 5` | `x = x // 5` |

### Comparison operators

Comparison operators are used to compare two values. Python has the following comparison operators:

| Operator | Description | Example |
| --- | --- | --- |
| `==` | Equal | `x == y` |
| `!=` | Not equal | `x != y` |
| `>` | Greater than | `x > y` |
| `<` | Less than | `x < y` |
| `>=` | Greater than or equal to | `x >= y` |
| `<=` | Less than or equal to | `x <= y` |

### Logical operators

Logical operators are used to combine conditional statements. Python has the following logical operators:

| Operator | Description | Example |
| --- | --- | --- |
| `and` | Returns `True` if both statements are true | `x < 5 and x < 10` |
| `or` | Returns `True` if one of the statements is true | `x < 5 or x < 4` |
| `not` | Reverse the result, returns `False` if the result is true | `not(x < 5 and x < 10)` |

### Identity operators

Identity operators are used to compare the objects, not if they are equal, but if they are actually the same object, with the same memory location. Python has the following identity operators:

| Operator | Description | Example |
| --- | --- | --- |
| `is` | Returns `True` if both variables are the same object | `x is y` |
| `is not` | Returns `True` if both variables are not the same object | `x is not y` |

### Membership operators

Membership operators are used to test if a sequence is presented in an object. Python has the following membership operators:

| Operator | Description | Example |
| --- | --- | --- |
| `in` | Returns `True` if a sequence with the specified value is present in the object | `x in y` |
| `not in` | Returns `True` if a sequence with the specified value is not present in the object | `x not in y` |

### Bitwise operators

Bitwise operators are used to compare (binary) numbers. Python has the following bitwise operators:

| Operator | Description | Example |
| --- | --- | --- |
| `&` | AND | `x & y` |
| `^` | XOR | `x ^ y` |
| `~` | NOT | `~x` |
| vertical bar | OR | `x` (symbol) `y` |
| `<<` | Zero fill left shift | `x << y` |
| `>>` | Signed right shift | `x >> y` |

</details>

<details>

<summary> Strings </summary>

Strings are used to store text. Strings can be declared in the following way:

```python
x = "Hello World"
```

Strings can also be declared in the following way:

```python
x: str = "Hello World"
```

### Multiline strings

Multiline strings can be declared in the following way:

```python
x = """Hello
World"""
```

Strings also allow you to use the following escape characters:

| Escape character | Description |
| --- | --- |
| `\'` | Single quote |
| `\"` | Double quote |
| `\t` | Tab |
| `\n` | Newline |
| `\r` | Carriage return |
| `\b` | Backspace |
| `\f` | Form feed |
| `\ooo` | Octal value |
| `\xhh` | Hex value |

### Format strings

Format strings allow you to format strings in the following way:

```python
x = "Hello, {}"
print(x.format("World"))
```

```python
country = "Norway"
x = "I live in {country}"
print(x.format(country = country))
# Or
print(f"I live in {country}")
```

Format strings also allow you to use the following format specifiers:

| Format specifier | Description |
| --- | --- |
| `:<` | Left align |
| `:>` | Right align |
| `:^` | Center align |
| `:=` | Places the sign to the left most position |
| `:+` | Use a plus sign to indicate if the result is positive or negative |
| `:-` | Use a minus sign for negative values only |
| `:` (space) | Use a leading space for positive numbers |
| `:,` | Use a comma as a thousand separator |
| `:_` | Use a underscore as a thousand separator |
| `:b` | Binary format |
| `:c` | Converts the value into the corresponding unicode character |
| `:d` | Decimal format |
| `:e` | Scientific format, with a lower case e |
| `:E` | Scientific format, with an upper case E |
| `:f` | Fix point number format |
| `:F` | Fix point number format, in uppercase format |
| `:g` | General format |
| `:G` | General format, in uppercase format |
| `:o` | Octal format |
| `:x` | Hex format, lower case |
| `:X` | Hex format, upper case |
| `:%` | Percentage format |

### Indexing Strings

You can access characters in a string by referring to its index number. Python uses zero-based indexing, so the first character has index 0. You can access a character in a string by using the square brackets notation:

```python
x = "Hello World"
print(x[0]) # H
```

You can also use negative indexing to access characters from the end of the string:

```python
x = "Hello World"
print(x[-1]) # d
```

Also allow you to access a range of characters by using the slice syntax:

```python
x = "Hello World"
print(x[2:5]) # llo
```

You could include a third parameter to specify the step:

```python
x = "Lorem ipsum dolor sit amet"
print(x[2:10:2]) # rmip
```

### String methods

Python has a set of built-in methods that you can use on strings. The following table lists the most commonly used methods:

| Method | Description |
| --- | --- |
| `upper()` | Converts a string into upper case |
| `lower()` | Converts a string into lower case |
| `capitalize()` | Converts the first character to upper case |
| `title()` | Converts the first character of each word to upper case |
| `strip()` | Returns a trimmed version of the string |
| `find()` | Searches the string for a specified value and returns the position of where it was found |
| `startswith()` | Returns `True` if the string starts with the specified value |
| `endswith()` | Returns `True` if the string ends with the specified value |
| `replace()` | Returns a string where a specified value is replaced with a specified value |
| `split()` | Splits the string at the specified separator, and returns a list |
| `join()` | Joins the elements of an iterable to the end of the string |

> Important: Strings are immutable, which means you cannot change a string once it has been created. But you can create a new string that is a variation of the original string.

</details>

<details>

<summary> Input & Output </summary>

The `input()` function is used to get user input. The `input()` function can be used in the following way:

```python
x = input("Enter your name: ")
```

The `print()` function is used to print text. The `print()` function can be used in the following way:

```python
print("Hello World")
```

The `print()` function also allows you to use the following arguments:

| Argument | Description |
| --- | --- |
| `sep` | Specify how to separate the objects, if there is more than one. Default is a single space character. |
| `end` | Specify what to print at the end. Default is a newline. |
| `file` | Specify an object to write the text to. Default is the screen. |
| `flush` | Specify if the output is flushed (True) or buffered (False). Default is False. |

</details>

<details>

<summary> Assignment statements </summary>

The simplest assignment statement is to assign a value to a variable. The assignment has the following form:

```python
variable = expression
```

The expression on the right-hand side is evaluated, and the resulting value is stored in the variable on the left-hand side.

Python allows you to assign a single value to several variables simultaneously. For example:

```python
x = y = z = 0
```

And allows you to assign multiple values to multiple variables simultaneously. For example:

```python
x, y, z = 0, 1, 2
```

You may be tempted to use it to swap two variables as follows:

```python
x, y = y, x
```

It works in Python, but it is not a good idea to use it in real programs. It is better to use a temporary variable for this purpose.

</details>

<details>

<summary> Lists </summary>

A list is a collection which is ordered and changeable. Allows different data types. Lists are written with square brackets.

```python
x = ["apple", "banana", "cherry"]
```

You can access items in a list by referring to the index number. Python uses zero-based indexing, so the first item has index 0. You can access an item in a list by using the square brackets notation:

```python
x = ["apple", "banana", "cherry"]
print(x[0]) # apple
```

You can also use negative indexing to access items from the end of the list:

```python
x = ["apple", "banana", "cherry"]
print(x[-1]) # cherry
```

### List methods

Python has a set of built-in methods that you can use on lists. The following table lists the most commonly used methods:

| Method | Description |
| --- | --- |
| `append()` | Adds an element at the end of the list |
| `clear()` | Removes all the elements from the list |
| `copy()` | Returns a copy of the list |
| `count()` | Returns the number of elements with the specified value |
| `extend()` | Add the elements of a list (or any iterable), to the end of the current list |
| `index()` | Returns the index of the first element with the specified value |
| `insert()` | Adds an element at the specified position |
| `pop()` | Removes the element at the specified position |
| `remove()` | Removes the item with the specified value |
| `reverse()` | Reverses the order of the list |
| `sort()` | Sorts the list |
| `split()` | Splits a string into a list |

</details>

<details>

<summary> Tuples </summary>

A tuple is a collection which is ordered and unchangeable. Allows different data types. Tuples are written with round brackets.

```python
x = ("apple", "banana", "cherry")
```

> Important: Tuples are unchangeable, meaning that you cannot change, add, or remove items once the tuple is created.

### Tuple methods

Python has two built-in methods that you can use on tuples. The following table lists the most commonly used methods:

| Method | Description |
| --- | --- |
| `count()` | Returns the number of times a specified value occurs in a tuple |
| `index()` | Searches the tuple for a specified value and returns the position of where it was found |

</details>

<details>

<summary> Sets </summary>

A set is a collection which is unordered and unindexed. No duplicate members. Sets are written with curly brackets.

```python
x = {"apple", "banana", "cherry"}
```

> Important: Sets are unordered, so you cannot be sure in which order the items will appear.

### Set methods

Python has a set of built-in methods that you can use on sets. The following table lists the most commonly used methods:

| Method | Description |
| --- | --- |
| `add()` | Adds an element to the set |
| `clear()` | Removes all the elements from the set |
| `copy()` | Returns a copy of the set |
| `difference()` | Returns a set containing the difference between two or more sets |
| `difference_update()` | Removes the items in this set that are also included in another, specified set |
| `discard()` | Remove the specified item |
| `intersection()` | Returns a set, that is the intersection of two other sets |
| `intersection_update()` | Removes the items in this set that are not present in other, specified set(s) |
| `isdisjoint()` | Returns whether two sets have a intersection or not |
| `issubset()` | Returns whether another set contains this set or not |
| `issuperset()` | Returns whether this set contains another set or not |
| `pop()` | Removes an element from the set |
| `remove()` | Removes the specified element |
| `symmetric_difference()` | Returns a set with the symmetric differences of two sets |
| `symmetric_difference_update()` | inserts the symmetric differences from this set and another |
| `union()` | Return a set containing the union of sets |
| `update()` | Update the set with the union of this set and others |

</details>

<details>

<summary> Dictionaries </summary>

A dictionary is a collection which is unordered, changeable and indexed. No duplicate members. Dictionaries are written with curly brackets, and they have keys and values.

```python
x = {"name": "John", "age": 36}
```

You can access the items of a dictionary by referring to its key name, inside square brackets:

```python
x = {"name": "John", "age": 36}
print(x["age"]) # 36
```

You can also use the `get()` method to access the items of a dictionary:

```python
x = {"name": "John", "age": 36}
print(x.get("age")) # 36
```

### Dictionary methods

Python has a set of built-in methods that you can use on dictionaries. The following table lists the most commonly used methods:

| Method | Description |
| --- | --- |
| `clear()` | Removes all the elements from the dictionary |
| `copy()` | Returns a copy of the dictionary |
| `fromkeys()` | Returns a dictionary with the specified keys and value |
| `get()` | Returns the value of the specified key |
| `items()` | Returns a list containing a tuple for each key value pair |
| `keys()` | Returns a list containing the dictionary's keys |
| `pop()` | Removes the element with the specified key |
| `popitem()` | Removes the last inserted key-value pair |
| `setdefault()` | Returns the value of the specified key. If the key does not exist: insert the key, with the specified value |
| `update()` | Updates the dictionary with the specified key-value pairs |
| `values()` | Returns a list of all the values in the dictionary |

</details>

<details>

<summary> Conditional statements </summary>

Conditional statements are used to perform different actions based on different conditions. Python has the following conditional statements:

| Statement | Description |
| --- | --- |
| `if` | Executes a statement if a specified condition is `True` |
| `elif` | Executes a statement if the previous condition is not `True` |
| `else` | Executes a statement if no previous conditions are `True` |

The `if` statement can be used in the following way:

```python
if condition:
    statement
```

The `elif` statement can be used in the following way:

```python
if condition:
    statement
elif condition:
    statement
```

The `else` statement can be used in the following way:

```python
if condition:
    statement
elif condition:
    statement
else:
    statement
```

</details>

<details>

<summary> Loops </summary>

Loops are used to execute a set of statements repeatedly until a particular condition is satisfied. Python has the following loop statements:

| Statement | Description |
| --- | --- |
| `while` | Executes a set of statements as long as a specified condition is `True` |
| `for` | Executes a set of statements for each item in a collection |

The `while` loop can be used in the following way:

```python
while condition:
    statement
```

The `for` loop can be used in the following way:

```python
for item in collection:
    statement
```

The `for` loop could be used with the `range()` function to loop through a set of code a specified number of times. For example:

```python
for x in range(5):
    statement
```

The `range()` function could be used with three parameters to specify the start, stop, and step. For example:

```python
for x in range(2, 10, 2):
    statement
```

And allows you to use the `else` statement to execute a block of code once, when the condition no longer is `True`. For example:

```python
while condition:
    statement
else:
    statement
```

Also you could use the `break` statement to stop the loop even if the condition is `True`. For example:

```python
while condition:
    statement
    if condition:
        break
```

</details>

<details>

<summary> Functions </summary>

A function is a block of code which only runs when it is called. You can pass data, known as parameters, into a function. A function can return data as a result. To create a function, use the `def` keyword, followed by a name, parentheses `()`, and a colon `:`. For example:

```python
def my_function():
    statement
```
