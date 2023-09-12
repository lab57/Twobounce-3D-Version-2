#pragma once
#include "Vector.hpp"
#include "Hit.hpp"
class Hit;
class TriObject;
class Triangle
{
public:
    Triangle(std::vector<Vector> coords);
    void setTexture(Vector2 at, Vector2 bt, Vector2 ct);
    float getArea();
    Hit intersect(Vector ray_start, Vector ray_vec);

    Vector a, b, c;
    Vector2 at, bt, ct;
    TriObject *object;
    Vector normal;
};
