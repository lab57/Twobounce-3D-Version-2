#pragma once
#include <vector>
#include <string>
#include "Triangle.hpp"
#include "Pixel.hpp"
class Triangle;
class TriObject
{
public:
    TriObject(std::string name, std::vector<Triangle> triangles, std::vector<Vector> points, int resolution = 100);

    void initTexture();
    float surfaceArea();
    void calcBoundingBox();

    std::string name;
    std::vector<Triangle> triangles;
    std::vector<Vector> points;
    std::vector<std::vector<Pixel>> texture;
    int resolution;
    std::vector<Vector> boundingBox;
};
