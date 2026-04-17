'''
Arrays

Create the following arrays

unsorted
5, 8, 4, 6, 2, 9, 1, 3, 7

sorted
1, 2, 3, 4, 5, 6, 7, 8, 9

'''

arrayOne = [5, 8, 4, 6, 2, 9, 1, 3, 7]
sortedArrayOne = [1, 2, 3, 4, 5, 6, 7, 8, 9]


'''
Trees
'''
class TreeNode:
    def __init__(self, n):
        self.val = n
        self.left = None
        self.right = None


'''

Create the following BST

        7
    3       11
1     4  9      13
 2

'''

bstOneRoot = TreeNode(7)
bstOneRoot.left = TreeNode(3)
bstOneRoot.right = TreeNode(11)
bstOneRoot.left.left = TreeNode(1)
bstOneRoot.left.left.right = TreeNode(2)
bstOneRoot.left.right = TreeNode(4)
bstOneRoot.right.left = TreeNode(9)
bstOneRoot.right.right = TreeNode(13)


'''
Graphs
'''
class GraphNode:
    def __init__(self, n):
        self.val = n
        self.neighbors = []

'''

Create the following Directed Graphs

dagOne
0 -> 4, 5
1 -> 2, 5
2 -> 3
3 -> 4
4 -> 
5 ->

dagTwo
0 -> 1
1 -> 2
2 -> 3
3 -> 4
4 -> 5
5 ->

dgWCycle
0 -> 1
1 -> 2
2 -> 3, 4
3 ->
4 -> 5
5 -> 2

'''

dagOne = {}
dagOne[0] = [4, 5]
dagOne[1] = [2, 5]
dagOne[2] = [3]
dagOne[3] = [4]
dagOne[4] = []
dagOne[5] = []

dagTwo = {}
dagTwo[0] = [1]
dagTwo[1] = [2]
dagTwo[2] = [3]
dagTwo[3] = [4]
dagTwo[4] = [5]
dagTwo[5] = []

dgWCycle = {}
dgWCycle[0] = [1]
dgWCycle[1] = [2]
dgWCycle[2] = [3, 4]
dgWCycle[3] = []
dgWCycle[4] = [5]
dgWCycle[5] = [2]


'''

Create the following Undirected Graphs

ugOne
0 -> 1
1 -> 0, 2
2 -> 1, 3
3 -> 2, 4
4 -> 3

ugWCycle
0 -> 1
1 -> 0, 2
2 -> 1, 3, 4, 5
3 -> 2
4 -> 2, 5
5 -> 2, 4

'''

ugOne = {}
ugOne[0] = [1]
ugOne[1] = [0, 2]
ugOne[2] = [1, 3]
ugOne[3] = [2, 4]
ugOne[4] = [3]

ugWCycle = {}
ugWCycle[0] = [1]
ugWCycle[1] = [0, 2]
ugWCycle[2] = [1, 3, 4, 5]
ugWCycle[3] = [2]
ugWCycle[4] = [2, 5]
ugWCycle[5] = [2, 4]


'''
Linked Lists
'''
class SingleLLNode:
    def __init__(self, n):
        self.val = n
        self.next = None

'''

Create the following singly linked list

4 -> 7 -> 3 -> 9 -> 1 -> 8

'''

sllOneHead = SingleLLNode(4)
sllOneHead.next = SingleLLNode(7)
sllOneHead.next.next = SingleLLNode(3)
sllOneHead.next.next.next = SingleLLNode(9)
sllOneHead.next.next.next.next = SingleLLNode(1)
sllOneHead.next.next.next.next.next = SingleLLNode(8)

class DoubleLLNode:
    def __init__(self, n):
        self.val = n
        self.next = None
        self.prev = None


