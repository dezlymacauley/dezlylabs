package models

type LinkedList struct {
    head *Node
    tail *Node
    length int
}

func NewLinkedList(value int) *LinkedList {
    var newNode *Node = NewNode(value)
    return &LinkedList {
        head: newNode,
        tail: newNode,
        length: 1,
    }
}
