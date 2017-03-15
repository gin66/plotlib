/*!

Data structures and helpers for managing plotting data

Five main components of the plotlib pipeline:

1. Data
2. Representation
3. View
4. Plot
5. Rendering

*Data* is the plain Rust data structure that the user brings along.
This might be something like a `Vec`, an `ndarray` or a slice.
This will likely be copied or moved from to contruct the *representation*.

The *representation* is the transformed version of that data which is the base plot object.
Each representation has N dimensions of input and one dimension of output.
For example a scatter plot has x-values as inputs and for each of those a y-value as an output.
A histogram has bins along one axis as its input and counts (or frequencies) as its output.
A surface has x and y values as inputs and a z-value as its output.
A function has some input value (mapped from the x-dimension) as its input
and some value as its output which is projected onto the y-dimension.

Each representation also contains a style which knows how it should look in the abstract.
A concrete interpretation of this style is deferred to when the rendering happens.
So a scatter plot will know what colour and style to use for the markers,
and a histogram will know which colours to use for its bars.

The *view* is how you want this data to be presented.
Each dimension from the representation is mapped onto an axis.
A view can contain multiple representations as long as they can be mapped on to the axes.
For example a 2D 'matrix' histogram could be displayed as a flat grid or as a 3D LEGO plot.

A *plot* is the whole 'page'. It can contain multiple views and specifies how they are laid out.

Finally the rendering is the actual output.
This could be an SVG, a PNG, an ASCII plot or an interactive web page.
A rendering will not necessarilly be able to show all types of views or representations
and may choose to ignore some.

This structure allows some data set to be represented multiple ways in one view
or for a particular representation to be displayed across more than one view.

Example

Data: A linear sequence of numbers as a Vec<f64>
Representation: A binned histogram, stored as a list of `Bin`s,
                each of which is the bounds and the counts.
                Blue bars with no casing.
View: Dimension 0 mapped to x-axis with range 5-19 and counts mapped to y-axis with range 0-60
Plot: A single view on the page
Rendering: An SVG

It starts from the end and works backwards.
The Rendering (an SVG in this case) knows how to layout a *plot*.
It finds a single View inside and so creates teh axes for it.
It knows how to draw the axes for the view.
It also knows how to draw each representation onto that view,
in this case, interpreting the bins and colours to create SVG elements.

*/

extern crate svg;

pub mod representation;
pub mod view;
pub mod plot;

pub mod histogram;
pub mod scatter;
pub mod axis;
pub mod utils;
pub mod text_render;
pub mod svg_render;
