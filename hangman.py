import random
import re

with open("movie_list.txt") as infile:
    movie_list = infile.readlines()

used_movies = []
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
    if len(used_movies) >= len(movie_list):
        print("You used up all movies O_o")
        print("Bye, do some work now!")
        break

    valid_movie = False
    while not valid_movie:
        movie = movie_list[random.randrange(len(movie_list))].strip()
        if movie not in used_movies:
            used_movies.append(movie)
            valid_movie = True

    compare_movie = movie.lower()
    guessed_movie = ""
    for word in movie.split():
        guessed_movie += "_" * len(word) + " "
    guessed_movie = guessed_movie.strip()

    remaining_guesses = 7
    used_characters = []
    print("You have 7 guesses")

    while remaining_guesses > 0:
        print(guessed_movie)
        print()

        valid_input = False
        while not valid_input:
            guess = input("Make a guess: ")[0]
            if guess not in used_characters:
                used_characters.append(guess)
                valid_input = True

        if re.search(guess, compare_movie):
            new_guess = ""
            for m in re.finditer(guess, compare_movie):
                new_guess += guessed_movie[len(new_guess) : m.start()] + movie[m.start()]
            new_guess += guessed_movie[len(new_guess):]
            guessed_movie = new_guess
        else:
            remaining_guesses -= 1
            print("Wrong guess, guesses remianing:", remaining_guesses)

        if guessed_movie.lower() == compare_movie:
            print(movie)
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
        print("Would you like to play again?")
        play_again = input("y/Y for yes, n/N for no: ").lower()
        if play_again != 'y':
            q = True
            print("See you later!")
