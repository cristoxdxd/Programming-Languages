# Files Management in Python

In Python, a file operation takes place in the following order:

1. Open a file
2. Read or write (perform operation)
3. Close the file

## Opening a File

Python has a built-in function `open()` to open a file. This function returns a file object, also called a handle, as it is used to read or modify the file accordingly.

```python
file = open("filename", "mode")
```

The `open()` function takes two parameters; filename, and mode. Another option to open a file is to use `with open`.

```python
with open("filename", "mode") as file:
    # perform file operations
```

### Modes

| Mode | Description |
| --- | --- |
| r | Opens a file for reading only. The file pointer is placed at the beginning of the file. This is the default mode. |
| rb | Opens a file for reading only in binary format. The file pointer is placed at the beginning of the file. This is the default mode. |
| r+ | Opens a file for both reading and writing. The file pointer placed at the beginning of the file. |
| rb+ | Opens a file for both reading and writing in binary format. The file pointer placed at the beginning of the file. |
| w | Opens a file for writing only. Overwrites the file if the file exists. If the file does not exist, creates a new file for writing. |
| wb | Opens a file for writing only in binary format. Overwrites the file if the file exists. If the file does not exist, creates a new file for writing. |
| w+ | Opens a file for both writing and reading. Overwrites the existing file if the file exists. If the file does not exist, creates a new file for reading and writing. |
| wb+ | Opens a file for both writing and reading in binary format. Overwrites the existing file if the file exists. If the file does not exist, creates a new file for reading and writing. |
| a | Opens a file for appending. The file pointer is at the end of the file if the file exists. If the file does not exist, it creates a new file for writing. |
| ab | Opens a file for appending in binary format. The file pointer is at the end of the file if the file exists. If the file does not exist, it creates a new file for writing. |
| a+ | Opens a file for both appending and reading. The file pointer is at the end of the file if the file exists. If the file does not exist, it creates a new file for reading and writing. |
| ab+ | Opens a file for both appending and reading in binary format. The file pointer is at the end of the file if the file exists. If the file does not exist, it creates a new file for reading and writing. |

## Reading a File

The `read()` method reads a string from an open file. It is important to note that Python strings can have binary data and not just text.

```python
file.read([size])
```

The `size` parameter is optional and if omitted, the entire contents of the file will be read and returned. If the end of the file has been reached, `read()` will return an empty string (`""`).

```python
file.read()
```

### Seeking a File

The `seek()` method changes the current file position. The `whence` argument is optional and defaults to `0`, which means absolute file positioning, other values are `1` which means seek relative to the current position and `2` means seek relative to the file's end.

```python
file.seek(offset[, whence])
```

## Writing to a File

The `write()` method writes any string to an open file. It is important to note that Python strings can have binary data and not just text.

```python
file.write(str)
```

The `str` parameter is the string which needs to be written to the file.

```python
file.write("This is a test")
```

## Closing a File

When you are done with operations to the file, you need to properly close the file.

```python
file.close()
```

## File Object Methods

| Method | Description |
| --- | --- |
| file.close() | Closes the opened file. |
| file.flush() | Flushes the internal buffer. |
| file.fileno() | Returns the integer file descriptor that is used by the underlying implementation to request I/O operations from the operating system. |
| file.isatty() | Returns true if file is connected to a terminal device, false otherwise. |
| file.next() | Returns the next line from the file each time it is being called. |
| file.read([size]) | Reads at most `size` bytes from the file. |
| file.readline([size]) | Reads entire line from the file. |
| file.readlines([size]) | Reads entire file from the file. |
| file.seek(offset[, whence]) | Changes the file position. |
| file.tell() | Returns the current file position. |
| file.truncate([size]) | Resizes the file to `size` bytes. |
| file.write(str) | Writes the string `str` to the file. |
| file.writelines(sequence) | Writes the sequence to the file. |

## File Object Attributes

| Attribute | Description |
| --- | --- |
| file.closed | Returns true if file is closed, false otherwise. |
| file.mode | Returns access mode with which file was opened. |
| file.name | Returns name of the file. |
