class Pets:
    def __init__(self, name, age, color):
        self.name = name
        self.age = age
        self.color = color

    def __str__(self):
        return "Name: " + self.name + "\nAge: " + str(self.age) + "\nColor: " + self.color
    
class Dogs(Pets):
    def __init__(self, name, age, color, breed):
        super().__init__(name, age, color)
        self.breed = breed

    def speak(self):
        print("Woof!")

    def eat(self):
        print(f"{self.name} is eating.")

    def sleep(self):
        print(f"{self.name} is sleeping.")

    def play(self):
        print(f"{self.name} is playing.")

    def __str__(self):
        return super().__str__() + "\nBreed: " + self.breed
    
class Cats(Pets):
    def __init__(self, name, age, color, breed):
        super().__init__(name, age, color)
        self.breed = breed

    def speak(self):
        print("Meow!")

    def eat(self):
        print(f"{self.name} is eating.")

    def sleep(self):
        print(f"{self.name} is sleeping.")

    def play(self):
        print(f"{self.name} is playing.")

    def __str__(self):
        return super().__str__() + "\nBreed: " + self.breed
