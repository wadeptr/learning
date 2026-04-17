import random
import datetime


part1_topics = [
    "Implement a Linked List from scratch",
    "Reverse a Linked List",
    "Implement an Array based Fixed Capacity Queue",
    "Implement an Array based Hash Table including the following:\n\n\t- Hash function\n\t- Insert method\n\t- Lookup method\n\t- Remove method\n\t- Collision resolution with doubly linked lists\n\t- Implementation should also re-size the table dynamically when chance of collisions becomes too great",
    "Implement a Graph via Adjacency List, and write methods for BFS and DFS",
    "Implement a Trie from scratch",
    "Return the following for a given list of items:\n\t1) all possible permutations\n\t2) all possible subsets"
]

part2_topics = [
    "Implement Binary Search recursively and iteratively",
    "Write Mergesort",
    "Write Quicksort",
    "Print a binary tree using BFS",
    "Print a binary tree using DFS (in-order) recursively and iteratively",
    "Print a binary tree using DFS (pre-order) recursively and iteratively",
    "Print a binary tree using DFS (post-order) recursively and iteratively",
    "Implement cycle detection for a Graph, both directed and undirected",
    "Implement Topological Sort for a directed Graph, both recursively and iteratively",
    "Implement the following Bit Manipulation tricks:\n\n\t1) Clear the lowest set bit of x\n\t2) Right propogat the lowest set bit of x\n\t3) Extract the lowest set bit from x\n\t4) Clear all bits from LSB to i-th bit\n\t5) Clear all bits from MSB to i-th bit\n\t6) Divide by 2\n\t7) Multiply by 2\n\t8) English alphabet upper case -> lower case conversion\n\t9) English alphabet lower case -> upper case conversion\n\t10) Count set bits in integer\n\t11) Find log base 2 of integer\n\t12) Check if integer is power of 2\n\t13) Calculate x ^ y without using '**'\n\t14) Given two bit positions, swap the bits at these positions\n\t15) Reverse the bits of x",
    "Find the kth smallest element in a list, via \n\t1) Sorting\n\t2) A Heap (both max and min)\n\t3) QuickSelect\n\t4) Median of medians",
]

def generate_daily_review_topic_set():

    print("\n-----------------------------------------------------")
    print("Data Structures and Algorithms Review for", datetime.date.today().strftime("%b %d %Y"))
    print("-----------------------------------------------------\n")

    topics = []

    part1 = input("Include Part I? [y / n] ")
    if part1 == "y":
        topics += part1_topics 

    part2 = input("\nInclude Part II? [y / n] ")
    if part2 == "y":
        topics += part2_topics

    indeces = list(range(0, len(topics)))
    random.shuffle(indeces)

    print("\nWhen finished with a task, press <Enter> to advance to the next\n\n")

    counter = 1
    for i in indeces:
        command = input(str(counter) + ") " + topics[i] + "\n")
        if command == "end":
            break
        counter += 1

    if counter == len(topics) + 1:
        print("----------")
        print("All Done!!")
        print("----------")

generate_daily_review_topic_set()
