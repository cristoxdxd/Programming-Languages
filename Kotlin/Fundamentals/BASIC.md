# Basic Kotlin program structure

## Variables

Variables in Kotlin could be declared using keyword `var` or `val`. The difference between them is that `var` is mutable and `val` is immutable.

> **Note:** It is recommended to use `val` instead of `var` whenever possible. Duck typing is used in Kotlin, so the compiler can infer the type of the variable.

```kotlin
var a: Int = 1
var b = 2
val c: Int = 3
val d = 4
```

### Data types

Kotlin is a statically typed language, which means that the type of the variable is known at compile time. The compiler can infer the type of the variable, so it is not necessary to specify it explicitly.  
Kotlin supports the following number types:

* `Double` - 64 bit
* `Float` - 32 bit
* `Long` - 64 bit
* `Int` - 32 bit
* `Short` - 16 bit
* `Byte` - 8 bit

Kotlin also supports the following types:

* `Boolean` - true or false
* `Char` - a single character
* `String` - a sequence of characters

### Type conversion

Kotlin does not support automatic type conversion. The following code will not compile:

```kotlin
val a: Int = 1
val b: Long = a
```

To convert a number to another type, the following functions can be used:

* `toByte()`
* `toShort()`
* `toInt()`
* `toLong()`
* `toFloat()`
* `toDouble()`
* `toChar()`
* `toString()`

```kotlin
val a: Int = 1
val b: Long = a.toLong()
```

## Functions

Functions in Kotlin are declared using keyword `fun`. The return type is specified after the function name. If the function does not return anything, the return type is `Unit`.

```kotlin
fun sum(a: Int, b: Int): Int {
    return a + b
}

fun printSum(a: Int, b: Int): Unit {
    println("sum of $a and $b is ${a + b}")
}
```

### Single-expression functions

If the function returns a single expression, the curly braces can be omitted and the body is specified after a `=` symbol.

```kotlin
fun sum(a: Int, b: Int): Int = a + b
```

### Default arguments

Function parameters can have default values, which are used when a corresponding argument is omitted.

```kotlin
fun sum(a: Int = 0, b: Int = 0): Int = a + b
```

## Control flow

### If expression

```kotlin
fun maxOf(a: Int, b: Int): Int {
    if (a > b) {
        return a
    } else {
        return b
    }
}
```

### When expression

```kotlin
fun describe(obj: Any): String =
    when (obj) {
        1 -> "One"
        "Hello" -> "Greeting"
        is Long -> "Long"
        !is String -> "Not a string"
        else -> "Unknown"
    }
```

### For loop

```kotlin
fun main() {
    val items = listOf("apple", "banana", "kiwifruit")
    for (item in items) {
        println(item)
    }
}
```

### For Each

```kotlin
fun main() {
    val items = listOf("apple", "banana", "kiwifruit")
    items.forEach { println(it) }
}
```

### While loop

```kotlin
fun main() {
    val items = listOf("apple", "banana", "kiwifruit")
    var index = 0
    while (index < items.size) {
        println("item at $index is ${items[index]}")
        index++
    }
}
```

## Classes

Classes in Kotlin are declared using keyword `class`. The constructor is specified after the class name. If the class has no body, the curly braces can be omitted.

```kotlin
class Person(name: String, surname: String)
```

### Properties

Classes can have properties. The properties can be mutable or immutable. The properties are declared in the class body.

```kotlin
class Person(val name: String, var surname: String)
```

### Constructors

Classes can have one primary constructor and one or more secondary constructors. The primary constructor is part of the class header and it is specified after the class name. The secondary constructor is specified in the class body.

```kotlin
class Person(val name: String, var surname: String) {
    constructor(name: String) : this(name, "")
}
```

### Inheritance

Classes can inherit from other classes. The parent class is specified after a colon.

```kotlin
open class Person(val name: String, var surname: String)

class Student(name: String, surname: String, val studentId: Int) : Person(name, surname)
```

### Overriding methods

Methods can be overridden in the child class. The `override` keyword is used to mark the method as overridden.

```kotlin
open class Person(val name: String, var surname: String) {
    open fun printInfo() {
        println("Name: $name, Surname: $surname")
    }
}

class Student(name: String, surname: String, val studentId: Int) : Person(name, surname) {
    override fun printInfo() {
        println("Name: $name, Surname: $surname, Student ID: $studentId")
    }
}
```

### Abstract classes

Classes can be marked as `abstract`. Abstract classes cannot be instantiated. Abstract methods do not have an implementation.

```kotlin
abstract class Person(val name: String, var surname: String) {
    abstract fun printInfo()
}

class Student(name: String, surname: String, val studentId: Int) : Person(name, surname) {
    override fun printInfo() {
        println("Name: $name, Surname: $surname, Student ID: $studentId")
    }
}
```

### Interfaces

Interfaces can be implemented by classes. The `override` keyword is used to mark the method as overridden.

```kotlin
interface Person {
    val name: String
    val surname: String
    fun printInfo()
}

class Student(override val name: String, override var surname: String, val studentId: Int) : Person {
    override fun printInfo() {
        println("Name: $name, Surname: $surname, Student ID: $studentId")
    }
}
```

### Data classes

Data classes are used to hold data. The `toString()`, `equals()` and `hashCode()` methods are automatically generated. The `copy()` method is also generated, which can be used to create a copy of the object.

```kotlin
data class Person(val name: String, var surname: String)

fun main() {
    val person = Person("John", "Doe")
    println(person) // Person(name=John, surname=Doe)
    println(person.copy(surname = "Smith")) // Person(name=John, surname=Smith)
}
```

### Object expressions

Object expressions are used to create anonymous objects. The object can extend a class or implement one or more interfaces.

```kotlin
fun main() {
    val person = object : Person("John", "Doe") {
        override fun printInfo() {
            println("Name: $name, Surname: $surname")
        }
    }
    person.printInfo() // Name: John, Surname: Doe
}
```

### Companion objects

Companion objects are used to store constants and utility functions. Companion objects are declared inside a class.

```kotlin
class Person(val name: String, var surname: String) {
    companion object {
        const val DEFAULT_NAME = "John"
        const val DEFAULT_SURNAME = "Doe"
        fun createDefaultPerson() = Person(DEFAULT_NAME, DEFAULT_SURNAME)
    }
}

fun main() {
    val person = Person.createDefaultPerson()
    println(person) // Person(name=John, surname=Doe)
}
```

## Packages

Packages are used to organize code. The package name is specified at the top of the file.

```kotlin
package com.example

class Person(val name: String, var surname: String)
```

## Imports

Imports are used to import classes from other packages. The import statement is specified at the top of the file.

```kotlin
import com.example.Person

fun main() {
    val person = Person("John", "Doe")
    println(person) // Person(name=John, surname=Doe)
}
```

## Visibility modifiers

Kotlin supports the following visibility modifiers:

* `public` - the default visibility, visible everywhere
* `internal` - visible inside the same module
* `protected` - visible inside the same class and subclasses
* `private` - visible inside the same class

## Nullable types

Kotlin supports nullable types. The `?` symbol is used to mark a type as nullable.

```kotlin
fun main() {
    val a: Int? = null
    val b: Int? = 1
    println(a) // null
    println(b) // 1
}
```

### Safe calls

The `?.` operator is used to safely call a method or access a property on a nullable object. If the object is null, the expression is skipped and null is returned.

```kotlin
fun main() {
    val a: Int? = null
    val b: Int? = 1
    println(a?.inc()) // null
    println(b?.inc()) // 2
}
```

### Elvis operator

The Elvis operator `?:` is used to specify a default value if the object is null.

```kotlin
fun main() {
    val a: Int? = null
    val b: Int? = 1
    println(a ?: 0) // 0
    println(b ?: 0) // 1
}
```

### Not-null assertion

The not-null assertion operator `!!` is used to tell the compiler that the object is not null. If the object is null, a `NullPointerException` is thrown.

```kotlin
fun main() {
    val a: Int? = null
    val b: Int? = 1
    println(a!!.inc()) // NullPointerException
    println(b!!.inc()) // 2
}
```

## Extensions

Extensions are used to add new functionality to existing classes. Extensions are declared outside of the class.

```kotlin
fun String.printInfo() {
    println("String: $this")
}

fun main() {
    "Hello".printInfo() // String: Hello
}
```

## Lambdas

Lambdas are used to pass a function as a parameter. The lambda is specified after the function call.

```kotlin
fun main() {
    val list = listOf(1, 2, 3)
    list.forEach { println(it) }
}
```

## Higher-order functions

Higher-order functions are functions that take other functions as parameters or return a function. The function type is specified using the `->` symbol.

```kotlin
fun main() {
    val list = listOf(1, 2, 3)
    list.filter { it > 1 }.forEach { println(it) }
}
```

## Collections

Kotlin supports the following collection types:

* `List` - an ordered collection
* `Set` - a collection of unique elements
* `Map` - a collection of key-value pairs

### List

```kotlin
fun main() {
    val list = listOf(1, 2, 3)
    println(list) // [1, 2, 3]
    println(list[0]) // 1
    println(list.get(0)) // 1
    println(list.size) // 3
    println(list.isEmpty()) // false
    println(list.contains(1)) // true
    println(list.indexOf(1)) // 0
    println(list.lastIndexOf(1)) // 0
    println(list.subList(0, 2)) // [1, 2]
    println(list + 4) // [1, 2, 3, 4]
    println(list + listOf(4, 5)) // [1, 2, 3, 4, 5]
    println(list - 1) // [2, 3]
    println(list - listOf(1, 2)) // [3]
    println(list.reversed()) // [3, 2, 1]
    println(list.sorted()) // [1, 2, 3]
    println(list.sortedDescending()) // [3, 2, 1]
    println(list.shuffled()) // [2, 3, 1]
    println(list.sum()) // 6
    println(list.average()) // 2.0
    println(list.min()) // 1
    println(list.max()) // 3
    println(list.first()) // 1
    println(list.last()) // 3
    println(list.joinToString()) // 1, 2, 3
    println(list.joinToString(prefix = "[", postfix = "]")) // [1, 2, 3]
    println(list.joinToString(separator = "; ")) // 1; 2; 3
    println(list.joinToString { "($it)" }) // (1), (2), (3)
    println(list.joinToString(transform = { "($it)" })) // (1), (2), (3)
```

### Set

```kotlin
fun main() {
    val set = setOf(1, 2, 3)
    println(set) // [1, 2, 3]
    println(set.size) // 3
    println(set.isEmpty()) // false
    println(set.contains(1)) // true
    println(set + 4) // [1, 2, 3, 4]
    println(set + listOf(4, 5)) // [1, 2, 3, 4, 5]
    println(set - 1) // [2, 3]
    println(set - listOf(1, 2)) // [3]
    println(set.reversed()) // [3, 2, 1]
    println(set.sorted()) // [1, 2, 3]
    println(set.sortedDescending()) // [3, 2, 1]
    println(set.shuffled()) // [2, 3, 1]
    println(set.sum()) // 6
    println(set.average()) // 2.0
    println(set.min()) // 1
    println(set.max()) // 3
    println(set.first()) // 1
    println(set.last()) // 3
    println(set.joinToString()) // 1, 2, 3
    println(set.joinToString(prefix = "[", postfix = "]")) // [1, 2, 3]
    println(set.joinToString(separator = "; ")) // 1; 2; 3
    println(set.joinToString { "($it)" }) // (1), (2), (3)
    println(set.joinToString(transform = { "($it)" })) // (1), (2), (3)
}
```

### Map

```kotlin
fun main() {
    val map = mapOf(1 to "one", 2 to "two", 3 to "three")
    println(map) // {1=one, 2=two, 3=three}
    println(map.size) // 3
    println(map.isEmpty()) // false
    println(map.containsKey(1)) // true
    println(map.containsValue("one")) // true
    println(map + Pair(4, "four")) // {1=one, 2=two, 3=three, 4=four}
    println(map + mapOf(4 to "four", 5 to "five")) // {1=one, 2=two, 3=three, 4=four, 5=five}
    println(map - 1) // {2=two, 3=three}
    println(map - listOf(1, 2)) // {3=three}
    println(map.reversed()) // {three=3, two=2, one=1}
    println(map.keys) // [1, 2, 3]
    println(map.values) // [one, two, three]
    println(map.entries) // [1=one, 2=two, 3=three]
    println(map.keys.joinToString()) // 1, 2, 3
    println(map.keys.joinToString(prefix = "[", postfix = "]")) // [1, 2, 3]
    println(map.keys.joinToString(separator = "; ")) // 1; 2; 3
    println(map.keys.joinToString { "($it)" }) // (1), (2), (3)
    println(map.keys.joinToString(transform = { "($it)" })) // (1), (2), (3)
    println(map.values.joinToString()) // one, two, three
    println(map.values.joinToString(prefix = "[", postfix = "]")) // [one, two, three]
    println(map.values.joinToString(separator = "; ")) // one; two; three
    println(map.values.joinToString { "($it)" }) // (one), (two), (three)
    println(map.values.joinToString(transform = { "($it)" })) // (one), (two), (three)
    println(map.entries.joinToString()) // 1=one, 2=two, 3=three
```

## Ranges

Ranges are used to represent a sequence of values. The `..` operator is used to create a range.

## Exceptions

Exceptions are used to handle errors. The `try-catch` block is used to catch exceptions.

```kotlin
fun main() {
    try {
        val a: Int? = null
        println(a!!.inc())
    } catch (e: Exception) {
        println(e) // java.lang.NullPointerException
    }
}
```
