# Rust Common Collections - Exercise 2

In the book The Rust Programming Language by Steve Klabnik and Carol Nichols,
chapter 8 - Common Collections - presents 3 additional exercises. This is my
solution to exercise 2:

* Convert strings to pig latin. The first consonant of each word is moved to 
the end of the word and "ay' is added, so "first" becomes "irst-fay". Words 
that start with a vowel have "hay" added to the end instead ("apple" becomes
"apple-hay"). Keep in mind the details about UTF-8 encoding!
