#pragma once
#include <iostream>
#include <cmath>
#include <vector>

class Vector2
{
public:
    Vector2(float u, float v);
    Vector2();

    Vector2 norm() const;
    float dot(const Vector2 &other) const;
    friend std::ostream &operator<<(std::ostream &os, const Vector2 &vec);
    Vector2 operator-(const Vector2 &other) const;
    Vector2 operator+(const Vector2 &other) const;
    Vector2 operator*(float scalar) const;
    Vector2 operator/(float scalar) const;
    float abs() const;
    float u, v, w;
};

class Vector
{
public:
    Vector(float x, float y, float z);
    Vector();
    Vector norm() const;
    float dot(const Vector &other) const;
    Vector cross(const Vector &other) const;
    Vector calcCoord(const Vector &st, float t_val) const;
    float find_t(const Vector &st, const Vector &endPt) const;
    Vector rotate(const Vector &axis, float angle) const;
    friend std::ostream &operator<<(std::ostream &os, const Vector &vec);
    Vector operator-(const Vector &other) const;
    Vector operator+(const Vector &other) const;
    Vector operator*(float scalar) const;
    Vector operator/(float scalar) const;
    float abs() const;
    float operator[](int index) const;
    float x, y, z, t, u, v;
    std::vector<float> arr;
};
