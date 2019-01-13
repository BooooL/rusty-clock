use <box.scad>
use <button.scad>
use <epaper.scad>
use <blue-pill.scad>
include <params.scad>
include <printing.scad>

box();

epaper_placement() epaper();

for (coord = button_coords)
  translate(coord)
    rotate([-90, 0, 0])
    button(thickness);

translate([-box_width/2+thickness+backpanel_pilone_size + 23/2,
           -box_height/2+backpanel_pilone_size - 0.5 * thickness,
           thickness + 53/2 + 2])
rotate([-90, 90, 0])
blue_pill();
