# Object Oriented Programming with Python

## Class

### Class Definition

```python
class ClassName:
    # class body
```

### Class Instantiation

```python
class ClassName:
    # class body

# Instantiation
instance = ClassName()
```

### Class Attributes

```python
class ClassName:
    # class body

# Instantiation
instance = ClassName()

# Accessing attributes
instance.attribute
```

### Class Methods

```python
class ClassName:
    # class body

# Instantiation
instance = ClassName()

# Calling methods
instance.method()
```

## Methods

### Instance Methods

```python
class ClassName:
    # class body

    def method(self):
        # method body
```

### Class Methods

```python
class ClassName:
    # class body

    @classmethod
    def method(cls):
        # method body
```

### Static Methods

```python
class ClassName:
    # class body

    @staticmethod
    def method():
        # method body
```

## Inheritance

```python
class ParentClass:
    # class body

class ChildClass(ParentClass):
    # class body
```

## Multiple Inheritance

```python
class ParentClass1:
    # class body

class ParentClass2:
    # class body

class ChildClass(ParentClass1, ParentClass2):
    # class body
```

<details>

<summary> Special Methods </summary>

### `__init__`

This method is used to initialize the instance of a class.

```python
class ClassName:
    # class body

    def __init__(self, arg1, arg2):
        # method body
```

### `__str__`

This method is used to return a string representation of the instance of a class.

```python
class ClassName:
    # class body

    def __str__(self):
        # method body
```

### `__repr__`

This method is used to return a string representation of the instance of a class.

```python
class ClassName:
    # class body

    def __repr__(self):
        # method body
```

### `__len__`

This method is used to return the length of the instance of a class.

```python
class ClassName:
    # class body

    def __len__(self):
        # method body
```

### `__add__`

This method is used to add two instances of a class.

```python
class ClassName:
    # class body

    def __add__(self, other):
        # method body
```

### `__eq__`

This method is used to check if two instances of a class are equal.

```python
class ClassName:
    # class body

    def __eq__(self, other):
        # method body
```

</details>

## Decorators

### `@property`

This decorator is used to define a property.

```python
class ClassName:
    # class body

    @property
    def property(self):
        # method body
```

### `@classmethod`

This decorator is used to define a class method.

```python
class ClassName:
    # class body

    @classmethod
    def method(cls):
        # method body
```

### `@staticmethod`

This decorator is used to define a static method.

```python
class ClassName:
    # class body

    @staticmethod
    def method():
        # method body
```

## Polymorphism

### Duck Typing

```python
class ClassName:
    # class body

    def method(self):
        # method body

def function(instance):

    instance.method()
```

### Operator Overloading

```python
class ClassName:
    # class body

    def __add__(self, other):
        # method body
```

## Abstract Base Classes

```python
from abc import ABC, abstractmethod

class ClassName(ABC):
    # class body

    @abstractmethod
    def method(self):
        # method body
```

## Data Classes

```python
from dataclasses import dataclass

@dataclass
class ClassName:
    # class body
```

## Named Tuples

```python
from collections import namedtuple

ClassName = namedtuple('ClassName', ['attribute1', 'attribute2'])
```

## Type Hints

```python
from typing import List, Dict, Tuple, Set, Optional

def function(arg1: str, arg2: int) -> None:
    # function body
```

## Generators

```python
def function():
    # function body

    yield
```

## Iterators

```python
class ClassName:
    # class body

    def __iter__(self):
        # method body

    def __next__(self):
        # method body
```

## Context Managers

```python
class ClassName:
    # class body

    def __enter__(self):
        # method body

    def __exit__(self, exc_type, exc_value, traceback):
        # method body
```

## Metaclasses

```python
class ClassName(type):
    # class body
```
