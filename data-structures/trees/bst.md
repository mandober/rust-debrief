# Binary Search Tree

- DS type: tree
- invented: 1960, P.F. Windley, A.D. Booth, A.J.T. Colin, T.N. Hibbard
- max nodes per level: `2^h`
- max nodes in tree: `2^(h+1) - 1`
- usage: 
  - implement other dynamic ADT (set, multiset, associative array)
  - implement lookup table: finds a value by its key (e.g. phone book)
- pros:
  - principle of binary search: all ops are logarithmic 
- cons:
  - each new node means new allocation
  - no data locality (cache miss increase)
- the shape of BST depends entirely on the order of insertions (deletions)


Time and space complexity:

BST    | Average  | Worst
-------|----------|-------
SPACE  | O(n)     | O(n)
Search | O(log n) | O(n)
Insert | O(log n) | O(n)
Delete | O(log n) | O(n)


## BST
A binary search tree (BST) is a rooted binary tree, whose internal nodes each store a key (and optionally, an associated value) and each have two distinguished sub-trees, commonly denoted left and right. The tree additionally satisfies the binary search property, which states that the key in each node must be greater than (or equal to) any key stored in the left sub-tree, and less than (or equal to) any key stored in the right sub-tree. The leaves do not contain keys and they are indistinguishable from one another.

- The shape of the binary search tree depends entirely on the order of insertions and deletions, and can become degenerate.
- After a long intermixed sequence of random insertion and deletion, the expected height of the tree approaches square root of the number of nodes, `√n`, which grows much faster than `log n`.
- Binary search requires an order relation by which every node can be compared with every other node in the sense of a *total preorder*. The part of the node that is compared against is called a *key*.
- Whether duplicates (different nodes with the same key) are allowed depends on the application only.


## Binary search
By keeping its keys sorted, a BST exploits the principle of binary search when looking up a key (or a place to insert a new key): traverse the tree from root to leaf comparing the keys stored in the nodes; decide, on the basis of the comparison, whether to continue searching in the left or right subtrees.

On average, this means that each comparison allows the operations to skip about half of the tree, so that each lookup, insertion or deletion takes time proportional to the logarithm of the number of items stored in the tree.

Maximum number of nodes to examine: `log n`

This is much better than the linear time required to find items by key in an (unsorted) array, but slower than the corresponding operations on hash tables.


## Height
The tree has height (depth) that excludes the root; if there's only the root node, the height is 0; the root node is always at level 0.

- height of the tree: `h`
- max nodes per level in relation to h: `2^h`
- max nodes in tree in relation to h: `2^(h+1) - 1`
- values: leftSubtree `<` parent `<` rightSubtree



## Insert

```python
function insert(val):
  # start from the root
  let node <- tree.root;
  # does root exist
  if node == NULL
    # insert new node with val
    tree.root <- node.new(val);
    return;
  else:
    let node <- tree.root
    # compare val to node.key:
    if val == node.key
      # val found
      return;
    if val < node.key
      # recurse left
      let node <- node.left;
      node.insert(val);
    else:
      # recurse right
      let node <- node.right;
      node.insert(val);
```


## Remove

- the node has no child: remove that node
- the node just has a left child: the left child of the removing node will take it's position on the tree
- the node has right child, and the right child does not have any left child: the right child of the node will take the position of the removing node on the tree
- the node has right child, and the right child also has left child: the most left child of the right child will be removed (removing this node will cause a recursive algorithm!) and take the position of the removing node.
