# Byte Utils
I created this library mainly for two reasons:
1) Increasing my knowledge and use of rust
2) I needed a library which could perform arithmetic operations on very large numbers 
(numbers larger than can be represented with 128 bits which seems to be the largest integer type which rusts provides)

I decided to create this library after attempting to write a crude implementation of the RSA encryption algorithm for
funzies. I got most of the basic functions implemented but ran into issues when I started to try and actually encrypt
a message. A couple of runtime panics and google searches later, I decided there wasn't a library that did exactly what
I wanted. I may have just blindly decided after 5 minutes that I will go against my instinctual gut feeling that I 
would be rewriting something that probably already exists and to be honest that's okay.

This project provides the following:
* a simple struct which holds a list of bytes
* Addition, Mult, Sub operators have been overloaded
* Functions for doing equality checks and exponentiation

Some also rather interesting implementation of doing binary addition and subtraction using only bit manipulation are
included for funzies as doing 1 + 2 is less cool than shifting bits around and using boolean logic to achieve the same
goal. Also carry and borrow bits are important for this lib to work so yeah maybe more than just fun.

