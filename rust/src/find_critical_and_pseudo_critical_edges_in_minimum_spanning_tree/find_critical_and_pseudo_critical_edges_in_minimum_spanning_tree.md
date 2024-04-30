

----

Tags: #leetcode #hard #graph #prims #dijkstra

----

## Drawing:
[[find_critical_and_pseudo_critical_edges_in_minimum_spanning_tree.excalidraw]]

----


## solution explanation:
main idea: find target mst with all edges, then iterate over all edges and find mst with force included edge, if cost of such mst is larger then idle, then this edge is not critical or non-critical, simply continue to next edge, else find mst with force exluded edge, if cost of such mst is larger then idle then this edge is critical(we can't find another route with equal price to idle mst without this edge), else, non-critical (we can find another route with equal price to idle mst without this edge)
