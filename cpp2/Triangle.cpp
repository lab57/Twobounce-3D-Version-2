#include "Triangle.hpp"

Triangle::Triangle(std::vector<Vector> coords)
    : a(coords[0]), b(coords[1]), c(coords[2]), object(nullptr), normal(Vector(0, 0, 0)) {}

void Triangle::setTexture(Vector2 at, Vector2 bt, Vector2 ct)
{
    this->at = at;
    this->bt = bt;
    this->ct = ct;
}

float Triangle::getArea()
{
    return 0.5f * (a - b).cross(a - c).norm().abs();
}

Hit Triangle::intersect(Vector ray_start, Vector ray_vec)
{
    Vector edge1 = b - a;
    Vector edge2 = c - a;
    Vector pvec = ray_vec.cross(edge2);
    float det = edge1.dot(pvec);

    if (abs(det) < 1e-5)
        return Hit(false, this);

    float inv_det = 1.0f / det;
    Vector tvec = ray_start - a;
    float u = tvec.dot(pvec) * inv_det;

    if (u < 0.0f || u > 1.0f)
        return Hit(false, this);

    Vector qvec = tvec.cross(edge1);
    float v = ray_vec.dot(qvec) * inv_det;

    if (v < 0.0f || u + v > 1.0f)
        return Hit(false, this);

    float t = edge2.dot(qvec) * inv_det;
    if (t < 1e-5)
        return Hit(false, this);

    return Hit(true, this, object, u, v, t, ray_start, ray_vec);
}
