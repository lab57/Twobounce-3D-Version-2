#pragma once
#include "Vector.hpp"
#include "Triangle.hpp"
#include "TriObject.hpp"
#include "Pixel.hpp"
class Triangle;
class TriObject;
class Vector;
class Pixel;

class Hit
{
public:
    Hit(bool didHit, Triangle *tri = nullptr, TriObject *obj = nullptr,
        float u = 0.0f, float v = 0.0f, float t = 0.0f,
        Vector origin = Vector(0, 0, 0), Vector dir = Vector(0, 0, 0));

    Pixel getPixel();
    Vector cartesian();

public:
    Triangle *tri;
    TriObject *obj;
    float u, v, t;
    Vector origin, dir;
    bool didHit;
};
