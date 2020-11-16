class Node:
    def __init__(self, name, age):
        self.left = None
        self.right = None
        self.name = name
        self.age = age

    def insert(self, name, age):
        if self.name:
            if name < self.name:
                if self.left is None:
                    self.left = Node(name,age)
                else:
                    self.left.insert(name, age)
            elif name > self.name:
                if self.right is None:
                    self.right = Node(name, age)
                else:
                    self.right.insert(name, age)
        else:
            self.name = name

    def PrintTree(self):
        if self.left:
            self.left.PrintTree()
        print( self.name, self.age),
        if self.right:
            self.right.PrintTree()


root = Node("Lucas", 12)
root.insert("Leonardo", 21)
root.insert("Marcos", 24)
root.insert("Ana", 10)

root.PrintTree()
