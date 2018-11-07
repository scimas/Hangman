import random
import re

with open("phrases.txt") as infile:
    phrase_list = infile.readlines()

used_phrases = []
q = False

print("Welcome to Hangman!")
print()
print("Although there is no hangman to be seen here,")
print("the game remains same - guess without")
print("running out of guesses or LOSE!")
print()
print("Here is the first challenge")
print("Start Guessing")
print()
while not q:
    if len(used_phrases) >= len(phrase_list):
        print("You used up all phrases O_o")
        print("Bye, do some work now!")
        break

    valid_phrase = False
    while not valid_phrase:
        phrase = phrase_list[random.randrange(len(phrase_list))].strip()
        if phrase not in used_phrases:
            used_phrases.append(phrase)
            valid_phrase = True

    compare_phrase = phrase.lower()
    guessed_phrase = ""
    for word in phrase.split():
        guessed_phrase += "_" * len(word) + " "
    guessed_phrase = guessed_phrase.strip()

    remaining_guesses = 7
    used_characters = []
    print("You have 7 guesses")

    while remaining_guesses > 0:
        print(guessed_phrase)
        print()

        valid_input = False
        while not valid_input:
            guess = input("Make a guess: ")
            if guess not in used_characters and len(guess) == 1:
                used_characters.append(guess)
                guess = "[" + guess + "]"
                valid_input = True

        if re.search(guess, compare_phrase):
            new_guess = ""
            for m in re.finditer(guess, compare_phrase):
                new_guess += guessed_phrase[len(new_guess) : m.start()] + phrase[m.start()]
            new_guess += guessed_phrase[len(new_guess):]
            guessed_phrase = new_guess
        else:
            remaining_guesses -= 1
            print("Wrong guess, guesses remianing:", remaining_guesses)
            print()

        if guessed_phrase.lower() == compare_phrase:
            print(phrase)
            print("Congratulations, you guessed it!")
            print()
            print("Would you like to play again?")
            play_again = input("y/Y for yes, n/N for no: ").lower()
            if play_again != 'y':
                q = True
                print("See you later!")
            break

    if remaining_guesses == 0:
        print("Haha, you lost :D")
        print("This was the correct answer:")
        print()
        print(phrase)
        print()
        print("Would you like to play again?")
        play_again = input("y/Y for yes, n/N for no: ").lower()
        if play_again != 'y':
            q = True
            print("See you later!")
