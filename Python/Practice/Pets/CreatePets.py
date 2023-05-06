import Pets

def create():
    print("\n\tWelcome to the pet creator!")
    print("1. Create a dog")
    print("2. Create a cat")
    print("3. Exit")
    choice = int(input("Enter your choice: "))
    while choice != 3:
        if choice == 1:
            name = input("\nEnter the dog's name: ")
            age = int(input("Enter the dog's age: "))
            color = input("Enter the dog's color: ")
            breed = input("Enter the dog's breed: ")
            dog = Pets.Dogs(name, age, color, breed)
            return dog
        elif choice == 2:
            name = input("\nEnter the cat's name: ")
            age = int(input("Enter the cat's age: "))
            color = input("Enter the cat's color: ")
            breed = input("Enter the cat's breed: ")
            cat = Pets.Cats(name, age, color, breed)
            return cat
        else:
            print("\n\tInvalid choice.")
