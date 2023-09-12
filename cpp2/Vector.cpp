#include <iostream>
#include <cmath>
#include <vector>
using namespace std;
class Vector2
{
public:
    float u, v, w;

    Vector2(float u, float v) : u(u), v(v)
    {
        w = 1 - u - v;
    }

    Vector2() : u(0.0f), v(0.0f)
    {
        w = 0;
    }

    Vector2 norm() const
    {
        return *this / abs();
    }

    float dot(const Vector2 &other) const
    {
        return u * other.u + v * other.v;
    }

    friend std::ostream &operator<<(std::ostream &os, const Vector2 &vec)
    {
        os << "<" << vec.u << ", " << vec.v << ">";
        return os;
    }

    Vector2 operator-(const Vector2 &other) const
    {
        return Vector2(u - other.u, v - other.v);
    }

    Vector2 operator+(const Vector2 &other) const
    {
        return Vector2(u + other.u, v + other.v);
    }

    Vector2 operator*(float scalar) const
    {
        return Vector2(u * scalar, v * scalar);
    }

    Vector2 operator/(float scalar) const
    {
        return Vector2(u / scalar, v / scalar);
    }

    float abs() const
    {
        return std::sqrt(u * u + v * v);
    }
};

class Vector
{
public:
    float x, y, z, t, u, v;
    std::vector<float> arr;

    Vector(float x, float y, float z) : x(x), y(y), z(z), t(x), u(y), v(z), arr{x, y, z}
    {
    }
    Vector() : x(0.0f), y(0.0f), z(0.0f)
    {
        t = x;
        u = y;
        v = z;
    }

    Vector norm() const
    {
        return *this / abs();
    }

    float dot(const Vector &other) const
    {
        return x * other.x + y * other.y + z * other.z;
    }

    Vector cross(const Vector &other) const
    {
        return Vector(y * other.z - z * other.y, z * other.x - x * other.z, x * other.y - y * other.x);
    }

    Vector calcCoord(const Vector &st, float t_val) const
    {
        Vector parallel = *this - st;
        Vector coord = st + (*this * t_val);
        return coord;
    }

    float find_t(const Vector &st, const Vector &endPt) const
    {
        Vector diff = endPt - st;
        return diff.x / x;
    }

    Vector rotate(const Vector &axis, float angle) const
    {
        Vector term1 = *this * std::cos(angle);
        Vector term2 = axis.cross(*this) * std::sin(angle);
        Vector term3 = axis * (axis.dot(*this) * (1 - std::cos(angle)));
        return term1 + term2 + term3;
    }

    friend std::ostream &operator<<(std::ostream &os, const Vector &vec)
    {
        os << "<" << vec.x << ", " << vec.y << ", " << vec.z << ">";
        return os;
    }

    Vector operator-(const Vector &other) const
    {
        return Vector(x - other.x, y - other.y, z - other.z);
    }

    Vector operator+(const Vector &other) const
    {
        return Vector(x + other.x, y + other.y, z + other.z);
    }

    Vector operator*(float scalar) const
    {
        return Vector(x * scalar, y * scalar, z * scalar);
    }

    Vector operator/(float scalar) const
    {
        return Vector(x / scalar, y / scalar, z / scalar);
    }

    float abs() const
    {
        return std::sqrt(x * x + y * y + z * z);
    }

    float operator[](int index) const
    {
        return arr[index];
    }
};

int main()
{

    return 0;
}
