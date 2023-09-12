#include "TriObject.hpp"

TriObject::TriObject(std::string name, std::vector<Triangle> triangles, std::vector<Vector> points, int resolution)
    : name(name), triangles(triangles), points(points), resolution(resolution)
{
    initTexture();
}

void TriObject::initTexture()
{
    texture.resize(resolution, std::vector<Pixel>(resolution, Pixel()));
}

void TriObject::calcBoundingBox()
{
    float minX = std::numeric_limits<float>::max();
    float minY = minX;
    float minZ = minX;
    float maxX = std::numeric_limits<float>::min();
    float maxY = maxX;
    float maxZ = maxX;

    for (const auto &point : points)
    {
        if (point.x < minX)
            minX = point.x;
        if (point.y < minY)
            minY = point.y;
        if (point.z < minZ)
            minZ = point.z;
        if (point.x > maxX)
            maxX = point.x;
        if (point.y > maxY)
            maxY = point.y;
        if (point.z > maxZ)
            maxZ = point.z;
    }

    boundingBox.push_back(Vector(minX, minY, minZ));
    boundingBox.push_back(Vector(maxX, maxY, maxZ));
}
