#include "Triangle.hpp"

#include "Vector.hpp"

using namespace std;
int main()
{
    cout << "main" << endl;

    Vector v1 = Vector(0, 0, 0);
    Vector v2 = Vector(1, 2, 3);
    Vector v3 = Vector(0, 0, 1);
    Triangle t = Triangle({v1, v2, v3});
    cout << t.a << endl;
}