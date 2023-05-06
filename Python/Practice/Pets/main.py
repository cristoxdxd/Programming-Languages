import random
import CreatePets

def main():
    userPets = []
    petActivities = ["speak", "eat", "sleep", "play"]
    print("\tPlay with pets!")
    print("1. Create a pet")
    print("2. Play with a pet")
    print("3. Exit")

    choice = int(input("\nEnter your choice: "))
    while choice != 3:
        if choice == 1:
            userPets.append(CreatePets.create())
        elif choice == 2:
            if userPets == []:
                print("You have no pets!")
                break
            print("\nWhich pet would you like to play with?")
            for i in range(len(userPets)):
                print(str(i + 1) + ". " + userPets[i].name)
            petChoice = int(input("Enter your choice: "))
            while petChoice < 1 or petChoice > len(userPets):
                print("Invalid choice.")
                petChoice = int(input("Enter your choice: "))
            print("\n")
            petActivity = petActivities[random.randint(0, len(petActivities) - 1)]
            if petActivity == "speak":
                userPets[petChoice - 1].speak()
            if petActivity == "eat":
                userPets[petChoice - 1].eat()
            if petActivity == "sleep":
                userPets[petChoice - 1].sleep()
            if petActivity == "play":
                userPets[petChoice - 1].play()
        else:
            print("Invalid choice.")
        print("\n\tPlay with pets!")
        print("1. Create a pet")
        print("2. Play with a pet")
        print("3. Exit")
        choice = int(input("Enter your choice: "))

if __name__ == "__main__":
    main()