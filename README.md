#What is this?
The very beginnings of a single-player version of ["Hey, that's my fish!"](https://boardgamegeek.com/boardgame/8203/hey-s-my-fish) with a slight rules change. 

# Some Context
This started out as a test to see if sdl2 would be a viable platform layer. I've tried to sepearate out the platform specific stuff so I could switch to a different backend if necessary.

After getting the hex tiles working I decided I wanted to turn this into a small game project. So I decided to make a version of "Hey, that's my fish!" but with some negative tiles. Working title: "Okay, that's *your* negative fish!"

But I've now decided to shelve this for now. Here's the current TODO list if I ever want to pick this up again:

* Hex edges have weird grey and brown edges. These turn out to be in the assests, but they are semi-transparent. Options: remoe the feathering from the assets, switch blend modes?

* make pieces transparent when hovering over them so you can see what is underneath.
    * mobile version?

* have only one piece for now

* unrestricted piece movement

* line restricted piece movement

* add a second piece, switch turens after each movement

* disallow moving to blank hex_dimensions

* collect hexes
