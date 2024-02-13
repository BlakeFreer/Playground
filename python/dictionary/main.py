"""
Author: Blake Freer
Date Created: 2024-02-12

Description
-----------
A CLI dictionary tool, based on the dictionary provided by
https://github.com/wordset/wordset-dictionary
"""

import json
import os
import sys

import colorama
from colorama import Fore

colorama.init()

SECOND_COLOR = Fore.BLACK

class Dictionary:
    def __init__(self):
        self.books: dict[str, dict] = {}

    def lookup(self, word: str) -> dict:
        word = word.strip().lower()
        first_letter = word[:1]
        self.__load(first_letter)

        try:
            return self.books[first_letter][word]
        except KeyError:
            return None

    def __load(self, letter: str) -> None:
        # If already loaded, ignore
        if letter in self.books.keys():
            return

        data_file = os.path.join(
            os.path.dirname(__file__),
            "wordset-dictionary",
            "data",
            f"{letter}.json",
        )

        try:
            with open(data_file, "r", encoding='utf-8') as f:
                new_book = json.load(f)
                lower_book = {}

                for k in new_book:
                    key = k.lower()
                    if k not in lower_book:
                        lower_book[key] = new_book[k]
                    else:
                        lower_book[key]['meanings'].extend(new_book[k]['meanings'])

                del new_book
                self.books[letter] = lower_book
        except FileNotFoundError as e:
            msg = (
                "Dictionary file not found. Ensure you have initialized the "
                "wordset-dictionary submodule with 'git submodule init'"
            )
            raise FileNotFoundError(msg) from e

    @staticmethod
    def format(lookup: dict) -> str:
        term_width = os.get_terminal_size().columns
        lines = [
            "-"*term_width,
            f"{Fore.RED}{lookup['word']}{Fore.RESET}",
        ]

        for meaning in lookup["meanings"]:
            meaning_lines = [
                "",
                f"{SECOND_COLOR}({meaning['speech_part']}){Fore.RESET} {meaning["def"]}",
            ]
            if "synonyms" in meaning.keys():
                meaning_lines.append((
                    f"  {SECOND_COLOR}Synonyms:{Fore.RESET} "
                    f"{", ".join(meaning['synonyms'])}"
                ))

            lines.extend(meaning_lines)

        return "\n".join(lines)


D = Dictionary()


def query(word):
    entry = D.lookup(word)
    if entry:
        return D.format(entry)
    else:
        return f"No entry found for \"{word}\"."


def main():
    try:
        while True:
            word = input(f"{SECOND_COLOR}Enter a word:{Fore.RESET} ").strip().lower()

            print(query(word))
            print("")
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    if sys.argv[1:]:
        word = " ".join(sys.argv[1:])
        print(query(word))
    else:
        main()
