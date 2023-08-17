
from ObjectLoader import ObjLoader
from Triangle import *
from Vector import *
from RTree import *
from PencilSource import *
from DiskDetector import *
from PencilSource import *
import multiprocessing as mp
from pprint import pprint
import random as r
from math import sin, cos
from Plotting import *
from Export import *

from numba import jit, njit

N = 100000

@jit
def twobounce():
    loader: ObjLoader = ObjLoader("./")
    objects, triangles = loader.load("2bounce_test_geo.obj")
    R = loader.buildTree(triangles)



    vector_sets = PencilSource(Vector(5, 0, -1), Vector(5, 0, 1)).getEmissionRays(N)
    detector    = DiskDetector(1, Vector(-5, 0, 0), Vector(-1, 0, 0), n=100)

    print("Performing one bounce")
    vis_to_source: list[Hit] = [] #Onebounce hits from source
    for core in vector_sets:
        for vectors in core:
            res:Hit = R.checkIntersections(*vectors)
            if(res.didHit):
                vis_to_source.append(res)
                res.getPixel().status = 1
    print("Performing twobounce")
    vis_to_detector = []
    for i,hit in enumerate(vis_to_source):
        if(detector.isVisible(R, hit.cartesian())):
            hit.getPixel().status = 2
            vis_to_detector.append(hit)

    print("v2s:", len(vis_to_source))
    print("v2d:", len(vis_to_detector))
    #for obj in objects:
       # print("aa", obj.name, obj.skip)

    # ax = createAx()
    # plotGeometry(ax, R)

    # for h in vis_to_source:
    #     plotHit(ax, h)

    # plotDisk(ax, detector)
    # h1 = vis_to_source[0]
    # print(h1.getPixel().status)


    main("2bounce_test_geo", objects)

    





twobounce()










