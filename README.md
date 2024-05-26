# Sort Visualisation

I thought it'd be interesting to experiment with sort visualisation in rust, using egui to handle turning it in to a web application.

# Implemented sorts
* Bubble
* Shaker
* Insertion
* Shell

# TODO
* Add other sort algorithms.
* Decide how to represent them.  All in the same page running simultaneously? Button selector to choose which to show? Both?
* Switch to a Sort trait that implements "step", maybe?  I've got too much code dupe between sort implementations
