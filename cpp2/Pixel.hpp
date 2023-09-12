#pragma once
#include <tuple>

class Pixel
{
public:
    Pixel();
    std::tuple<int, int, int> getColor();

private:
    int status;
};
