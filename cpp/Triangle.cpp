#include "Vector.cpp"
#include <string>
using namespace std;

const int RED[3] = {255, 0, 0};

const int GREEN[3] = {0, 255, 0};

const int GREY[3] = {125, 125, 125};

class Pixel
{
public:
    int status;
    Pixel() : status(0) {}

    const int *getColor()
    {
        switch (status)
        {
        case 0:
            return GREY;
        case 1:
            return RED;
        case 2:
            return GREEN;
        default:
            return GREY;
        }
    }
};

class Triangle
{
public:
    Vector a, b, c, normal;
    Vector2 at, bt, ct;
    TriObject object;
};

class TriObject
{
public:
    int resolution;
    string name;
    vector<Triangle> triangles;
    Pixel[][] texture;
};

int main()
{
    Vector v1(1, 1, 1);
    cout << v1 << endl;
}