# Python Most used Libraries

<details>

<summary> Math </summary>

### `math`

Used for mathematical operations.

```python
import math

math.ceil(3.2) # 4
math.floor(3.2) # 3
math.factorial(5) # 120
math.gcd(12, 8) # 4
math.log(2, 10) # 0.3010299956639812
math.log10(2) # 0.3010299956639812
math.log2(2) # 1.0
math.pow(2, 3) # 8.0
math.sqrt(4) # 2.0
```

</details>

<details>

<summary> Random </summary>

### `random`

Used for generating random numbers.

```python
import random

random.random() # 0.37444887175646646
random.randint(1, 10) # 7
random.randrange(1, 10) # 7
random.choice([1, 2, 3, 4, 5]) # 3
random.choices([1, 2, 3, 4, 5], k=2) # [2, 5]
random.sample([1, 2, 3, 4, 5], k=2) # [2, 5]
random.shuffle([1, 2, 3, 4, 5]) # [3, 2, 5, 1, 4]
```

</details>

<details>

<summary> PDF's </summary>

Used for creating and editing PDF's.

```python
import PyPDF2 as pdf

pdf_file = open("file.pdf", "rb")

reader = pdf.PdfFileReader(pdf_file)
reader.numPages # 2

page = reader.getPage(0)
page.extractText() # "Hello World"

writer = pdf.PdfFileWriter()
writer.addPage(page)
writer.addBlankPage()
writer.write(open("new_file.pdf", "wb"))
```

</details>

<details>

<summary> Text-to-speech </summary>

Used for converting text to speech.

```python
import pyttsx3

engine = pyttsx3.init()
text = "Hello World"
engine.say(text)
engine.runAndWait()
```

</details>

<details>

<summary> Translator </summary>

Used for translating text.

```python
from googletrans import Translator

translator = Translator()
text = "Hello World"
translated = translator.translate(text, dest="es")
```

</details>

<details>

<summary> Web Scraping </summary>

Used for scraping data from websites.

```python
import requests

url = "https://www.google.com"
response = requests.get(url)
response.status_code # 200
response.content # b'<!doctype html>...'
```

</details>

<details>

<summary> Games </summary>

Used for creating games.

```python
import pygame

pygame.init()
screen = pygame.display.set_mode((400, 300))
done = False

while not done:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            done = True

    pygame.display.flip()
```

</details>

<details>

<summary> GUI </summary>

Used for creating GUI's.

```python
import tkinter as tk

window = tk.Tk()
window.title("Hello World")
window.geometry("400x300")
window.mainloop()
```
