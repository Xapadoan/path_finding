# PathFinding

This repo contains a small algo to perform path finding on 2D maps.

**NOTE**: It doesn't take into account the width needed for a path.
If you want to use this algo, go take a look into the src/geometry/segment.rs and see how the function `point_left_to_end` and `point_right_to_end` work with `DODGE_DISTANCE`.
Also note that to take into account the width of a line, it could be better to split using more than one point.

**NOTE**: It's possible (although it hasn't been reproduced yet) that the algo goes infinite loop in the case when two obstacles overlap.

## Algorithm

We first create a map (Map) containing the obstacles and a graph with a point of origin and destination.
Upon creation, the graph will create a segment (origin to destination).

When calling graph.solve, it will:
 - test our first path.
 - split it in two paths if not path is OK.
 - if one path is valid, it will keep only the paths (valid or not) shorter than the valid one.
 - repeat until there are no more paths to test.
