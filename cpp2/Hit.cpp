#include "Hit.hpp"

Hit::Hit(bool didHit, Triangle *tri, TriObject *obj, float u, float v, float t, Vector origin, Vector dir)
    : didHit(didHit), tri(tri), obj(obj), u(u), v(v), t(t), origin(origin), dir(dir) {}

Pixel Hit::getPixel()
{
    if (!obj || !u || !tri || !v)
    {
        throw std::runtime_error("Hit has no object");
    }
    // Simplified texture calculation based on given context
    int x = static_cast<int>(obj->resolution * u);
    int y = static_cast<int>(obj->resolution * (1 - v));
    return obj->texture[y][x];
}

Vector Hit::cartesian()
{
    if (!t)
    {
        throw std::runtime_error("Hit did not terminate");
    }
    return dir.calcCoord(origin, t);
}
