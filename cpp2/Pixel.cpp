#include "Pixel.hpp"

Pixel::Pixel() : status(0) {}

std::tuple<int, int, int> Pixel::getColor()
{
    std::tuple<int, int, int> green(0, 255, 0);
    std::tuple<int, int, int> red(255, 0, 0);
    std::tuple<int, int, int> grey(150, 150, 150);
    if (status == 0)
        return grey;
    else if (status == 1)
        return green;
    else if (status == 2)
        return red;
    return grey; // default
}
